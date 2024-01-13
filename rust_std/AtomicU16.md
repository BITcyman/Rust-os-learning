## AtomicU16

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

