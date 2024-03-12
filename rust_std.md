# Rust 一些 Std的学习

### AtomicU16

https://doc.rust-lang.org/std/sync/atomic/struct.AtomicU16.html

一个可以在线程间安全共享的整数类型

#### std::sync::atomic::Ordering

```rust
#[non_exhaustive]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```

### PhanotomData

[深入理解 PhantomData](https://zhuanlan.zhihu.com/p/533695108)

+ std [文档](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
+ 用于指明一个不会用到但需要指明的生命周期 或者 参数类型
+ `PhantomData<T>` 只在编译时等同于 `T`，在运行时等同于 `()`



