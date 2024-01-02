
#![allow(unused_variables)]
fn main() {
    enum GeneratorState<Y, R> {
        Yielded(Y),  
        Complete(R), 
    }

    trait Generator {
        type Yield;
        type Return;
        fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
    }

    enum GeneratorA {
        Enter,
        Yield1 {
            to_borrow: String,
            borrowed: *const String, // NB! This is now a raw pointer!
        },
        Exit,
    }

    impl GeneratorA {
        fn start() -> Self {
            GeneratorA::Enter
        }
    }
    impl Generator for GeneratorA {
        type Yield = usize;
        type Return = ();
        fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return> {
                match self {
                GeneratorA::Enter => {
                    let to_borrow = String::from("Hello");
                    let borrowed = &to_borrow;
                    let res = borrowed.len();
                    *self = GeneratorA::Yield1 {to_borrow, borrowed: std::ptr::null()};
                    
                    // NB! And we set the pointer to reference the to_borrow string here
                    if let GeneratorA::Yield1 {to_borrow, borrowed} = self {
                        *borrowed = to_borrow;
                    }
                
                    GeneratorState::Yielded(res)
                }

                GeneratorA::Yield1 {borrowed, ..} => {
                    let borrowed: &String = unsafe {&**borrowed};
                    println!("{} world", borrowed);
                    *self = GeneratorA::Exit;
                    GeneratorState::Complete(())
                }
                GeneratorA::Exit => panic!("Can't advance an exited generator!"),
            }
        }
    }

    let mut generator = GeneratorA::start();
    if let GeneratorState::Yielded(n)  = generator.resume(){
        println!("Got value {}", n);
    }
    if let GeneratorState::Complete(()) = generator.resume() {
        ()
    };
}