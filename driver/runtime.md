## 异步串口驱动的运行时

#### 目前最主要的问题

异步的 串口读写操作 无法在 同步的内核中 通知 调取读写操作的线程 相应的读写操作已经完成

+ Q: 是否需要在 OS 和 异步串口驱动 中都使用 同一种 异步运行时（或者说同一种执行器）

    + 内核启动时是否需要采用一些不同的启动手段，类似于 `#[embassy_executor::task]`，来将其运行在一个 executor 线程上

        ```rust
        /// 在 rcore 中引入异步运行时
        
        #[no_mangle]
        // 内核入口函数
        pub fn rust_main() -> ! {
            let executor = EXECUTOR.init(Executor::new());
            executor.run(|spawner| {
                // 把 spawner 作为参数放入 kernel 启动函数，为支持后面的多线程
                spawner.spawn(kernel_start(spawner)).unwrap();
            });
        }
        
        #[embassy_executor::task]
        // 把 spawner 作为参数放入 kernel 启动函数，为支持后面的多线程，目前还没用到
        async fn kernel_start(spawner: Spawner) {
        	//... 
        }
        ```

+ Q: 如果需要同一种异步运行时，是否需要该异步运行时也是一个独立的 Crate

    + 因为目前异步串口驱动中的 Task、Waker 以及相应的 Executor 都是在驱动内部实现的，并不是一个独立的库
    + 这种异步运行时是否可以直接使用 Embassy 的运行时，还是 学长改过的运行时？因为之前用过 [ats-intc](https://github.com/ATS-INTC/ats-intc)，但似乎不能用到 星光2 开发板上

+ Q: 具体到实现上，怎么通知 线程 相应的读写操作已经完成

    + 在目前的实现中，管理具体读写任务的 future 被包装在一个 Task 中，这个 Task 直接加入执行器，并不会返回到内核，相应的内核不知道这个 Task 的 执行状态，就更不可能通知 线程 相应的读写操作已经完成。（这种实现下，同步的 OS 无法通过 block_on 使其阻塞地等待该事件完成）

        ```rust
        /// 目前的异步串口驱动的 async read 实现
        
        pub async fn read(self: Arc<Self>, buf: &'static mut [u8]) {
                let future = SerialReadFuture {
                    buf,
                    read_len: 0,
                    driver: self.clone(),
                };
                // 注册
                let task = Task::new(Box::pin(future), self.clone(), crate::task::TaskIOType::Read);
                match task.clone().poll(){
                    Poll::Ready(_) => {
                        log::debug!("first read successfully");
                        drop(task)
                    },
                    Poll::Pending=> {
                        self.register_readwaker(
                            unsafe { from_task(task.clone()) }
                        );
                        self.executor.push_task(Task::from_ref(task.clone()));
                    }
                }
            }
        
        ```

    + 如果是使用同一个异步运行时，又该怎么去通知对应的线程？（或者这个时候可以叫唤醒？）Embassy中时钟中断的例子：采用一个静态的数组去记录等待在某个时钟上升沿的事件的waker，然后等到时钟信号到来唤醒该 waker

        ```rust
        /// embassy 中 有关 时钟中断 的处理
        static EXTI_WAKERS: [AtomicWaker; EXTI_COUNT] = [NEW_AW; EXTI_COUNT];
        
        // 接收到中断请求的时候就
        // Wake the tasks
            for pin in BitIter(bits) {
                EXTI_WAKERS[pin as usize].wake();
            }
        
        unsafe fn on_irq() {
        	// ...
            // Wake the tasks
            for pin in BitIter(bits) {
                EXTI_WAKERS[pin as usize].wake();
            }
        
            //...
        }
        ```

    + 那么如何记录这些等待在某个读写事件上的线程？（原则上上面的 `EXTI_WAKERS` 是按照时间中断的引脚 pin 来限制，不是按照动态产生的某个读写事件来排列的）是否可以在调用读写事件的时候动态分配一个id，然后 让 该线程的 waker 存到对应 id 的位置处，同时读写事件也会保存这个 id，等到事件完成以后，就去 wake 对应id位置的 waker