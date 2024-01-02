# 《200行代码讲透RUST FUTURES》 笔记
[资料链接](https://stevenbai.top/rust/futures_explained_in_200_lines_of_rust)

### 异步实现的方法
+ 使用os提供的线程  Thread
    + 已经很好，简单易用
    + 但在类似于高负载web服务器环境（数量大的小请求）下对内存的消耗很大
+ 绿色线程  Green Thread
    + 用户态线程，有一个调度程序执行在 main 中
    + 不是一个零成本抽象，在 Rust 1.0 版本删除了该特性
        + 会给不需要使用该抽象的程序增加成本
+ 函数回调  callback
    + 保存一个指向一组指令的指针，在Rust中将是一个闭包
    + use std::sync::mpsc::{channel, Receiver, Sender}
+ Promises

#### Rust Futures

+ 三事件
    + 轮询 （在 executor 上执行）
    + 等待
    + 唤醒 （由 reactor 等待事件并唤醒 Future）
+ Leaf-future
    + 代表访问一个资源（如socket），运行时创建的实现了 `Future` trait 的对象
+ Non-leaf-future
    +  使用 async 标记的代码块

#### Runtime 运行时

+ Rust与其他语言不同，需要主动选择
+ 且 executor 和 Reactor 是分开的
+ Rust 标准库提供了
    + `Future` trait
    + `async` 和 `await` 关键字，通过他们暂停和恢复 Future
    + Waker 接口，唤醒暂停的Future

```rust
let non_leaf = async {
    let mut stream = TcpStream::connect("127.0.0.1:3000").await.unwrap(); // <-- yield
    
    // request a large dataset
    let result = stream.write(get_dataset_request).await.unwrap(); // <-- yield
    
    // wait for the dataset
    let mut response = vec![];
    stream.read(&mut response).await.unwrap(); // <-- yield

    // do some CPU-intensive analysis on the dataset
    let report = analyzer::analyze_data(response).unwrap();
    
    // send the results back
    stream.write(report).await.unwrap(); // <-- yield
};

```

+ 为防止在analyzer处理数据集时，执行器忙于计算而不是处理新的请求，可以使用的方法：
    + 我们可以创建一个新的`leaf future`，它将我们的任务发送到另一个线程，并在任务完成时解析。 我们可以像等待其他Future一样等待这个`leaf-future`。
    + 运行时可以有某种类型的管理程序来监视不同的任务占用多少时间，并将执行器本身移动到不同的线程，这样即使我们的分析程序任务阻塞了原始的执行程序线程，它也可以继续运行。（问题在于）

#### 唤醒器 Waker 和上下文 Context

+ 胖指针 和 vtable
    + 包括`&[T]` 和`&dyn Trait`，指针中既包含指向对象的指针，也包含它所对应的`vtables`指针
+ 实例代码

```rust
// A reference to a trait object is a fat pointer: (data_ptr, vtable_ptr)
trait Test {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn mul(&self) -> i32;
}

// This will represent our home brewn fat pointer to a trait object
   #[repr(C)]
struct FatPointer<'a> {
    /// A reference is a pointer to an instantiated `Data` instance
    data: &'a mut Data,
    /// Since we need to pass in literal values like length and alignment it's
    /// easiest for us to convert pointers to usize-integers instead of the other way around.
    vtable: *const usize,
}

// This is the data in our trait object. It's just two numbers we want to operate on.
struct Data {
    a: i32,
    b: i32,
}

// ====== function definitions ======
fn add(s: &Data) -> i32 {
    s.a + s.b
}
fn sub(s: &Data) -> i32 {
    s.a - s.b
}
fn mul(s: &Data) -> i32 {
    s.a * s.b
}

fn main() {
    let mut data = Data {a: 3, b: 2};
    // vtable is like special purpose array of pointer-length types with a fixed
    // format where the three first values has a special meaning like the
    // length of the array is encoded in the array itself as the second value.
    let vtable = vec![
        0,            // pointer to `Drop` (which we're not implementing here)
        6,            // lenght of vtable
        8,            // alignment

        // we need to make sure we add these in the same order as defined in the Trait.
        add as usize, // function pointer - try changing the order of `add`
        sub as usize, // function pointer - and `sub` to see what happens
        mul as usize, // function pointer
    ];

    let fat_pointer = FatPointer { data: &mut data, vtable: vtable.as_ptr()};
    let test = unsafe { std::mem::transmute::<FatPointer, &dyn Test>(fat_pointer) };

    // And voalá, it's now a trait object we can call methods on
    println!("Add: 3 + 2 = {}", test.add());
    println!("Sub: 3 - 2 = {}", test.sub());
    println!("Mul: 3 * 2 = {}", test.mul());
}
```

+ 交换上面 vtable 中的方法地址的顺序，将会导致最后执行的结果不能得到相应结果

#### 生成器和async/await

+ 状态机 Enter -> Yield -> Exit
+ 实例代码 
    + generator_test 
    + safe_generator



#### Pin

+ [复习] rust 闭包

    + ```rust
        || 42;
        |x| x + 1;
        |x:i32| x + 1;
        |x:i32| -> i32 { x + 1 };
        ```

    + 闭包可以捕获外部的环境变量（自由变量）。

        闭包捕获变量的方式分为三类：引用（&T）、可变引用（&mut T）和值（T）。

        捕获变量时，闭包会根据上面列出的顺序（从约束最少到约束最多），优先按引用捕获，必要时才会使用后面的捕获方式：

    + ```rust
        /// 闭包捕获可变引用
        let mut count = 0;
        // 闭包按可变引用捕获变量count 
        // incr也必须是可变的，因为它持有可变引用，调用incr会改变闭包的状态
        let mut incr = || { count += 1; println!("count = {}", count); };
        incr();
        
        
        /// 闭包捕获值
        use std::mem;
        // b是不可复制类型，因此按值捕获时所有权会转移
        let b = Box::new(12);
        let f = || {
            println!("b = {}", b);
            // drop函数取T类型，因此闭包会按值捕获变量b
            mem::drop(b);
        };
        f();
        
        /// 或者强行按值捕获
        let f = move || {
            println!("b = {}", b);
        };
        f();
        ```

+ 自引用结构 （[Rust Async: Pin概念解析](https://zhuanlan.zhihu.com/p/67803708)）

    + ```rust
        fn main() {
            async fn func1() -> i32 { 12 }
        
            let func2 = async || -> i32 {
                let t = 1;                  
                let v = t + 1; 
                let b = func1().await;
                let rv = &v;   
                *rv + b
            };
        
            let fut = func2();
            println!("future size: {}", std::mem::size_of_val(&fut));
        }
        ```

    + 从代码形式上看，好像 `t`, `v`是局部变量，运行时存储在stack中。然而由于await的存在，整个函数不再是一气呵成从头执行到尾，而是分成了两段。在执行第二段的时候， 前半段执行的局部变量已经从stack中清理掉了，而第二段捕获了第一段的局部变量 `v`, 因此 `v`只能保存在编译器生成的匿名enum中。这个enum充当了函数执行时的虚拟栈(virtual stack)。 如果将 `letb=func1().await;`和 `letrv=&v;`调换位置，从打印结果来看，生成的enum大小变大了，因为捕获的是 `rv`这个引用变量，而被引用的变量v也得一起保存在enum中， 也就是说借用一旦跨了await，就会导致编译器需要构造一个自引用的结构。

    + 因为async/await生成是匿名的自引用结构，用户无法直接读写结构内部的字段，因此只需要处理好意外移动的问题就可以。

    + 但memcpy等操作...

        + 分配到堆上，使得在移动时移动Box<T>指针，原对象并没有被移动，指针指向的位置依然正确
        + 另外由于`Box`提供的api中可以获取到`&mut T`，进而可以通过 `mem::swap`间接将T移出。 所以只需要提供一个弱化版的智能指针api，防止泄露 `&mut T`就能够达到防止对象被移动。这就是实现 `Pin` api的主要思路。

#### 实现Futures

总的代码见[./overall/src/main.rs](./overall/src/main.rs)



#### 其他看过的文档

[零成本异步I/O](https://zhuanlan.zhihu.com/p/97574385)

[Rust Async: Pin概念解析](https://zhuanlan.zhihu.com/p/67803708)