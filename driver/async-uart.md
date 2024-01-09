### 在 rCore 中引入异步串口

#### 初始化设备 

同时提供异步和同步串口设备接口，在  `feature = "async"` 下使用异步串口设备实现，在  `feature = "sync"` 下使用同步串口设备实现。

```rust
pub type AsyncCharDeviceImpl = crate::drivers::chardev::AsyncNS16550a<VIRT_UART>;
pub type CharDeviceImpl = crate::drivers::chardev::NS16550a<VIRT_UART>;

pub fn device_init() {
    use riscv::register::sie;
    let mut plic = unsafe { PLIC::new(VIRT_PLIC) };
    let hart_id: usize = 0;
    let supervisor = IntrTargetPriority::Supervisor;
    let machine = IntrTargetPriority::Machine;
    plic.set_threshold(hart_id, supervisor, 0);
    plic.set_threshold(hart_id, machine, 1);
    //irq nums: 5 keyboard, 6 mouse, 8 block, 10 uart
    for intr_src_id in [5usize, 6, 8, 10] {
        plic.enable(hart_id, supervisor, intr_src_id);
        plic.set_priority(intr_src_id, 1);
    }
    unsafe {
        sie::set_sext();
    }
}
```



#### 中断处理

根据不同的 feature 值调用不同的中断请求处理接口，在  `feature = "async"` 下使用异步串口设备实现，在  `feature = "sync"` 下使用同步串口设备实现。

```rust
pub fn irq_handler() {
    let mut plic = unsafe { PLIC::new(VIRT_PLIC) };
    let intr_src_id = plic.claim(0, IntrTargetPriority::Supervisor);
    match intr_src_id {
        5 => KEYBOARD_DEVICE.handle_irq(),
        6 => MOUSE_DEVICE.handle_irq(),
        8 => BLOCK_DEVICE.handle_irq(),
        #[cfg(feature = "async")]
        10 => ASYNC_UART.handle_irq(),
        #[cfg(feature = "sync")]
        10 => UART.handle_irq(),
        _ => panic!("unsupported IRQ {}", intr_src_id),
    }
    plic.complete(0, IntrTargetPriority::Supervisor, intr_src_id);
}
```

异步状态下的串口中断处理接口

```rust
pub fn handle_irq(&self) {
        self.inner.clone().exclusive_session(|inner| {
            if let Some(ch) = inner.read() {
                if let Some(waker) = inner.read_waker_list.pop_front() {
                    inner.read_buffer.push_back(ch);
                    waker.clone().wake();
                }
            }

            if inner.writable() {
                if let Some(waker) = inner.write_waker_list.pop_front() {
                    waker.clone().wake();
                }
            }
        });
    }
```

在一个 `NS16550aRaw` 中维护了一个`Read_waker_list`和`Write_waker_list`，用于存储由于等待事件而处于pending状态下的协程们的唤醒器。

+ 异步`NS16550aRaw` 的 read 和 write 请求将分发给 `AsyncCharReader` 和 `AsyncCharWriter` 进行轮询，为此需要为 `AsyncCharReader` 和 `AsyncCharWriter` 实现 Future 轮询接口

    ```rust
    pub struct AsyncCharWriter<const BASE_ADDR: usize> {
        ns16550a: Arc<AsyncNS16550a<BASE_ADDR>>,
        ch: u8,
    }
    
    impl<const BASE_ADDR: usize> Future for AsyncCharWriter<BASE_ADDR> {
        type Output = ();
    
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let mut raw = self.ns16550a.inner.exclusive_access();
            let write_end = raw.write_end();
            if write_end.lsr.read().contains(LSR::THR_EMPTY) {
                // writable
                write_end.thr.write(self.ch);
                Ready(())
            } else {
                let waker = cx.waker().clone();
                raw.write_waker_list.push_back(waker);
                Pending
            }
        }
    }
    
    pub struct AsyncCharReader<const BASE_ADDR: usize> {
        ns16550a: Arc<AsyncNS16550a<BASE_ADDR>>,
    }
    
    impl<const BASE_ADDR: usize> Future for AsyncCharReader<BASE_ADDR> {
        type Output = u8;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let clone = self.ns16550a.clone();
            let mut raw = clone.inner.exclusive_access();
            if let Some(ch) = raw.read_buffer.pop_front() {
                // readable
                drop(raw);
                Ready(ch)
            } else {
                let waker = cx.waker().clone();
                let will_wake = raw.read_waker_list.iter()
                    .any(|x| x.will_wake(&waker));
                if !will_wake {
                    raw.read_waker_list.push_back(waker);
                    drop(raw);
                }
                return Pending;
            }
        }
    }
    
    ```

    

#### 用户态测例

```rust
// user/src/bin/async.rs
pub fn uart() -> ! {
    let time = uart_test();
    println!("============================");
    println!("uart: {}ms", time);
    println!("============================");
    exit(0)
}

// user/src/file.rs
pub fn uart_test() -> isize {
    sys_uart_test()
}

// user/src/syscall.rs
pub fn sys_uart_test() -> isize {
    syscall(SYSCALL_UART_TEST, [0, 0, 0])
}
```

进入内核态后，调用的系统调用

```rust
pub fn uart_test() -> isize {
    let start = get_time_ms();
    #[cfg(feature = "async")]
    {
        let executor = EXECUTOR.init(Executor::new());
        executor.run(|spawner| {
            spawner.spawn(send_data(spawner)).unwrap();
        });
    }
    #[cfg(feature = "sync")]
    {
        let mut  cmd = String::new();
        loop {
            mock_other_resource();
            let ch = UART.read();
            cmd.push(ch as char);
            print!("{}", ch as char);
            if cmd.len() >= 20 {
                println!("You Got It!");
                break;
            }
        }
    }
    (get_time_ms() - start) as isize
}

#[embassy_executor::task]
pub async fn send_data(_spawner: Spawner) {
    for _ in 0..20 {
        for ch in 33..123 {
            for _ in 0..250 {
                ASYNC_UART.clone().write(ch as u8).await;
            }
            ASYNC_UART.clone().write('\n' as u8).await;
        }
    }
    // mark work as finished
    let mark = WorkMarker {};
    mark.mark_finish();
}
```

