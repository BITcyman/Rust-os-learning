### Embassy 学习文档



#### Embassy-Executor

Embassy 中给出的一个执行器

```rust
//创建一个新的 Executor
pub fn new()  ->  Self;  

// Executor, 启动！
pub fn run(&'static mut self, init: impl FnOnce(Spawner)) -> !
// 这里需要一个 &`static mut self, 意味着我们需要创建一个一直存在的 Executor 实例，并且保证我们能够获取可变引用，可以使用的方法：a StaticCell (safe)、a static mut (unsafe)等
// 用 init 参数来 spawn 初始化 task，当 init 结束以后，Exectutor::run 就开始 run tasks；如果想要 spawn 多个 tasks, 需要持有 Spawner 的 Copies (比方说把他作为一个参数传入到初始化函数中)
```

+ StaticCell [文档](https://docs.rs/static_cell/latest/static_cell/)

    + 用于以下场景：

        + 需要 `&'static T `, 但是由于 `T` 不在一个不变动的上下文中，所以不能够简单的使用 `static`
        + 需要`&'static mut T `

    + ```rust
        use static_cell::StaticCell;
        
        // Statically allocate memory for a `u32`.
        static SOME_INT: StaticCell<u32> = StaticCell::new();
        
        // Initialize it at runtime. This returns a `&'static mut`.
        let x: &'static mut u32 = SOME_INT.init(42);
        assert_eq!(*x, 42);
        
        // Trying to call `.init()` again would panic, because the StaticCell is already initialized.
        // SOME_INT.init(42);
        ```

#### Embassy-Spawner

用于在一个 executor 中 spawn（启动） tasks，但是只能用于 executor 线程上。（如果需要跨线程，使用 [SendSpawner](https://docs.embassy.dev/embassy-executor/git/std/struct.SendSpawner.html)）

```rust
// 创建一个全局的 Embassy 默认的 Executor
static EXECUTOR: StaticCell<Executor> = StaticCell::new();

#[no_mangle]
// 内核入口函数
pub fn rust_main() -> ! {
    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        // 把 spawner 作为参数放入 kernel 启动函数，为支持后面的多线程
        spawner.spawn(kernel_start(spawner)).unwrap();
    });
}

async fn test(){
    let f1 = async {
        println!("========= async test f1 ==============");
    };
    let f2 = async {
        println!("========= async test f2 ==============");
    };
    let f3 = async {
        println!("========= async test f3 ==============");
    };
    f3.await;
    f2.await;
    f1.await;
}

#[embassy_executor::task]
// 把 spawner 作为参数放入 kernel 启动函数，为支持后面的多线程，目前还没用到
async fn kernel_start(spawner: Spawner) {
    clear_bss();
    mm::init();
    UART.init();
    println!("KERN: init gpu");
    let _gpu = GPU_DEVICE.clone();
    println!("KERN: init keyboard");
    let _keyboard = KEYBOARD_DEVICE.clone();
    println!("KERN: init mouse");
    let _mouse = MOUSE_DEVICE.clone();
    println!("KERN: init trap");
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    board::device_init();
    fs::list_apps();
    task::add_initproc();
    *DEV_NON_BLOCKING_ACCESS.exclusive_access() = true;

    // 测试异步调用
    test().await;

    // 开始线程调度
    task::run_tasks().await;
    panic!("Unreachable in rust_main!");
}
```


