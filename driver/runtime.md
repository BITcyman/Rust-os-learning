## 异步串口驱动的运行时

#### 5-4

+ [ ] debug，看看异步串口驱动哪里出问题了，感觉可能是读操作的中断接收机制出问题了
+ [ ] Alien 上板



#### 5-3

+ [x] Qemu 中 Alien 使用异步串口驱动 

##### 修改的使用异步启动的 Alien [仓库分支地址](https://github.com/BITcyman/Alien/tree/async-uart) 

直接使用默认的 OpenSBI 试了一下，居然直接就行了，能够接收到外部中断（哭了）

目前在 Alien 中开了双核，4个新的串口；内核启动后，一个核用于正常执行原 Alien的程序，另一个核在初始化完毕后，会使用 Embassy 运行时的启动过程（即在该核上运行的线程为 executor 线程），然后初始化异步串口，执行相关的读写操作。目前控制台输出如下，能够看到成功将新创建的一个串口的中断绑定在 id=1 的 hart 上了。但目前的写操作能够正常写入，但读操作却没有读到相应的内容。

```
[1] driver_init
[1] [devices/src] init async_uart
[1] Init async serial, base_addr:0x10005000,irq:15	<= 新创建了一个串口，用于测试异步驱动
[1] PLIC enable irq 15 for hart 1, priority 1		<= 将多创建的串口绑定在1号hart上，
[1] external interrupt hart_id: 1, irq: 15
[1] [DEBUG] [Async Serial] Interrupt!
[1] [DEBUG] [SERIAL] Transmitter Holding Register Empty
[1] [DEBUG] read task
[1] [DEBUG] pending 4
[1] [DEBUG] read pend
[1] external interrupt hart_id: 1, irq: 15
[1] [DEBUG] [Async Serial] Interrupt!
[1] [DEBUG] [SERIAL] Transmitter Holding Register Empty
[1] [DEBUG] first write successfully! write num is 9  <= [1,2,3,4,5,6,7,8,9]
[1] [0, 0, 0, 0, 0, 0]								<= 显然没有读到 
```





#### 5-2

##### 调了一会还是没能找到 RustSBI 问题所在，尝试修改 OpenSBI 

+ [ ] 拉取 OpenSBI 源码并编译，看看 Alien 是否能够使用该 OpenSBI 正常运行
+ [ ] 修改 OpenSBI 后编译，看看 Alien 是否能够使用修改后的 OpenSBI 正常运行

OpenSBI 编译出问题，唉，难啊。



#### 5-1 

##### QEMU 中 Alien 使用异步串口驱动

###### Qemu 中创建多个串口

在 qemu 启动的参数中加入 `-serial /dev/pts/4 -serial /dev/pts/6 -serial /dev/pts/10` 创建多个串口。

但 Alien 启动后 只检测 一个串口设备。

```shell
/// 在 Alien 初始化设备时打印设备树信息如下
[0]     /
[0]     reserved-memory
[0]     mmode_resv0@80000000
[0]     fw-cfg@10100000
[0]     flash@20000000
[0]     chosen
[0]     memory@80000000
[0]     cpus
[0]     cpu@0
[0]     interrupt-controller
[0]     cpu-map
[0]     cluster0
[0]     core0
[0]     soc
[0]     rtc@101000
[0]     uart@10000000                         《---- only one uart
[0]     poweroff
[0]     reboot
[0]     test@100000
[0]     pci@30000000
[0]     virtio_mmio@10008000
[0]     virtio_mmio@10007000
[0]     virtio_mmio@10006000
[0]     virtio_mmio@10005000
[0]     virtio_mmio@10004000
[0]     virtio_mmio@10003000
[0]     virtio_mmio@10002000
[0]     virtio_mmio@10001000
[0]     plic@c000000
[0]     clint@2000000
```

已询问 陈志扬 学长，需要改一下 Qemu 和 opensbi 的相关配置。

使用修改后的 Qemu 能够正常启动 Alien，并且输出设备信息能够看见多个串口设备。

```
[0] init device start
...
[0]     uart@10000000
[0]     serial@10005000
[0]     serial@10004000
[0]     serial@10003000
[0]     serial@10002000
...
```

由于 rCore-N 中有一个已经改过的 RustSBI，能够启动多个串口，并使得串口能够接收中断，故选择直接把 那里面的 RustSBI 拿过来用。但直接使用导致原 Alien 启动时输出的信息格式出现问题。

```
[rustsbi] Implementation: RustSBI-QEMU Version 0.0.2
[rustsbi-dtb] Hart count: cluster0 with 1 cores
[rustsbi] misa: RV64ACDFHIMSU
[rustsbi] mideleg: ssoft, stimer, sext (0x1666)
[rustsbi] medeleg: ima, ia, la, sa, uecall, ipage, lpage, spage (0xb1a3)
[rustsbi] enter supervisor 0x80200000
[0] 
         _      _   _
                         / \    | | (_)   ___   _ __
                                                       / _ \   | | | |  / _ \ | '_ \
                                                                                      / ___ \  | | | | |  __/ | | | |
                                                                                                                      /_/   \_\ |_| |_|  \___| |_| |_|

                                                                                                                                                      [0] Init logger None
                [0] Boot hart 0
                               [0] This is a device tree representation of a riscv-virtio,qemu machine
                                                                                                      SMP:    1
                                                                                                               Memory: 0x80000000..0xc0000000
                                                                                                                                             PLIC:   0xc000000..0xc800000
               CLINT:  0x2000000..0x2010000
                                           Initrd: Some(
                                                            0x88200000..0x882bb136,
                                                                                   )
                                                                                    Bootargs: Some("rdinit=/init")
                                                                                                                  [0] Page start:0x82947,end:0xc0000,count:0x3d6b9
        [0] Bitmap manage 251569 pages using 8 pages
                                                    [0] Frame allocator init success
                                                                                    [0] Relocate initrd data to 0x000000008294f000
                                                                                                                                  [0] Kernel Heap size: 0x26MB
```

可以看到 os启动后 输出的相关信息的格式有问题，主要是由于换行和空格的位置不是很合理，同时最后并没有唤醒初始线程去执行 shell。 正在查找 bug 位置。[已找到]

输出相关信息乱码的原因已经找到，是由于 在使用 RustSBI 的相关接口输出回车时，需要加入 "\r"（即 "\r\n"）。但由于一些内部包含 “\n” 的信息分散在 Alien 中，还没有加入 "\r"，仍存在少部分的格式错误。

但目前还没有找到 没有唤醒初始线程的原因。

###### 使用 Embassy 的异步运行时启动内核，然后创建一个新的线程用于管理异步串口驱动

+ 目前使用 OpenSBI 后能够正常启动 Alien 内核，执行简单的异步程序
+ 但使用 RustSBI 后初始线程无法被唤醒的问题依然存在



#### 4-30 目前最主要的问题

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



+ 几个串口 -czy