fn main() {
    let start = Instant::now();
    let reactor = Reactor::new();


    let fut1 = async {
        let val = Task::new(reactor.clone(), 1, 100).await;
        println!("Got {} at time: {:.2}.", val, start.elapsed().as_secs_f32());
    };


    let fut2 = async {
        let val = Task::new(reactor.clone(), 2, 2).await;
        println!("Got {} at time: {:.2}.", val, start.elapsed().as_secs_f32());
    };

    let mainfut = async {
        println!("mainfut begins");
        fut1.await;
        println!("funt1 finished");
        fut2.await;
        println!("funt2 finished");
    };

    // println!("");

    block_on(mainfut);
    println!("after close");
    reactor.lock().map(|mut r| r.close()).unwrap();
}

use std::{
    future::Future, sync::{ mpsc::{channel, Sender}, Arc, Mutex, Condvar},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker}, mem, pin::Pin,
    thread::{self, JoinHandle}, time::{Duration, Instant}, collections::HashMap
};
// ============================= EXECUTOR ====================================
#[derive(Default)]
struct Parker(Mutex<bool>, Condvar);

impl Parker {
    fn park(&self) {
        let mut resumable = self.0.lock().unwrap();
            while !*resumable {
                resumable = self.1.wait(resumable).unwrap();
            }
        *resumable = false;
    }

    fn unpark(&self) {
        *self.0.lock().unwrap() = true;
        self.1.notify_one();
    }
}

fn block_on<F: Future>(mut future: F) -> F::Output {
    let parker = Arc::new(Parker::default());
    let mywaker = Arc::new(MyWaker { parker: parker.clone() });
    let waker = mywaker_into_waker(Arc::into_raw(mywaker));
    let mut cx = Context::from_waker(&waker);
    
    // SAFETY: we shadow `future` so it can't be accessed again.
    let mut future = unsafe { Pin::new_unchecked(&mut future) }; 
    loop {
        match Future::poll(future.as_mut(), &mut cx) {
            Poll::Ready(val) => break val,
            Poll::Pending => parker.park(),
        };
    }
}
// ====================== FUTURE IMPLEMENTATION ==============================
#[derive(Clone)]
struct MyWaker {
    parker: Arc<Parker>,
}

#[derive(Clone)]
pub struct Task {
    id: usize,
    reactor: Arc<Mutex<Box<Reactor>>>,
    data: u64,
}

fn mywaker_wake(s: &MyWaker) {
    let waker_arc = unsafe { Arc::from_raw(s) };
    waker_arc.parker.unpark();
}

fn mywaker_clone(s: &MyWaker) -> RawWaker {
    let arc = unsafe { Arc::from_raw(s) };
    std::mem::forget(arc.clone()); // increase ref count
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

const VTABLE: RawWakerVTable = unsafe {
    RawWakerVTable::new(
        |s| mywaker_clone(&*(s as *const MyWaker)),   // clone
        |s| mywaker_wake(&*(s as *const MyWaker)),    // wake
        |s| mywaker_wake(*(s as *const &MyWaker)),    // wake by ref
        |s| drop(Arc::from_raw(s as *const MyWaker)), // decrease refcount
    )
};

fn mywaker_into_waker(s: *const MyWaker) -> Waker {
    let raw_waker = RawWaker::new(s as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

impl Task {
    fn new(reactor: Arc<Mutex<Box<Reactor>>>, data: u64, id: usize) -> Self {
        Task { id, reactor, data }
    }
}

impl Future for Task {
    type Output = usize;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut r = self.reactor.lock().unwrap();
        if r.is_ready(self.id) {
            *r.tasks.get_mut(&self.id).unwrap() = TaskState::Finished;
            Poll::Ready(self.id)
        } else if r.tasks.contains_key(&self.id) {
            // This is important. The docs says that on multiple calls to poll,
            // only the Waker from the Context passed to the most recent call
            // should be scheduled to receive a wakeup. That's why we insert
            // this waker into the map (which will return the old one which will
            // get dropped) before we return `Pending`.
            r.tasks.insert(self.id, TaskState::NotReady(cx.waker().clone()));
            Poll::Pending
        } else {
            r.register(self.data, cx.waker().clone(), self.id);
            Poll::Pending
        }
    }
}
// =============================== REACTOR ===================================
#[derive(Debug)]
enum TaskState {
    Ready,
    NotReady(Waker),
    Finished,
}

struct Reactor {
    dispatcher: Sender<Event>,
    handle: Option<JoinHandle<()>>,
    tasks: HashMap<usize, TaskState>,
}

#[derive(Debug)]
enum Event {
    Close,
    Timeout(u64, usize),
}

impl Reactor {
    fn new() -> Arc<Mutex<Box<Self>>> {
        let (tx, rx) = channel::<Event>();
        let reactor = Arc::new(Mutex::new(Box::new(Reactor {
            dispatcher: tx,
            handle: None,
            tasks: HashMap::new(),
        })));
        
        let reactor_clone = Arc::downgrade(&reactor);
        let handle = thread::spawn(move || {
            let mut handles = vec![];
            let mut t = 0;
            for event in rx {
                println!("========================");
                println!("come in loop");
                t += 1;
                let reactor = reactor_clone.clone();
                match event {
                    Event::Close => break,
                    Event::Timeout(duration, id) => {
                        let event_handle = thread::spawn(move || {
                            thread::sleep(Duration::from_secs(duration));
                            let reactor = reactor.upgrade().unwrap();
                            println!("before wake");
                            for (k, v) in &reactor.lock().unwrap().tasks{
                                println!("{} : {:?}", k, v);
                            }
                            reactor.lock().map(|mut r| r.wake(id)).unwrap();

                            println!("after wake");
                            for (k, v) in &reactor.lock().unwrap().tasks{
                                println!("{} : {:?}", k, v);
                            }
                        });
                        handles.push(event_handle);
                    }
                }
                println!("finish loop {}",t);
            }
            println!("finished all the loop");
            handles.into_iter().for_each(|handle| handle.join().unwrap());
        });
        reactor.lock().map(|mut r| r.handle = Some(handle)).unwrap();
        reactor
    }

    fn wake(&mut self, id: usize) {
        let state = self.tasks.get_mut(&id).unwrap();
        match mem::replace(state, TaskState::Ready) {
            TaskState::NotReady(waker) => waker.wake(),
            TaskState::Finished => panic!("Called 'wake' twice on task: {}", id),
            _ => unreachable!()
        }
    }

    fn register(&mut self, duration: u64, waker: Waker, id: usize) {
        if self.tasks.insert(id, TaskState::NotReady(waker)).is_some() {
            panic!("Tried to insert a task with id: '{}', twice!", id);
        }
        self.dispatcher.send(Event::Timeout(duration, id)).unwrap();
    }

    fn close(&mut self) {
        self.dispatcher.send(Event::Close).unwrap();
    }

    fn is_ready(&self, id: usize) -> bool {
        self.tasks.get(&id).map(|state| match state {
            TaskState::Ready => true,
            _ => false,
        }).unwrap_or(false)
    }
}

impl Drop for Reactor {
    fn drop(&mut self) {
        self.handle.take().map(|h| h.join().unwrap()).unwrap();
    }
}
