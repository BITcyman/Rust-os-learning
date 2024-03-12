##### Todo List

+ [x] 学习一些基本的概念
    + [x] 串口
    + [x] uart
    + [x] pac
    + [x] svd
+ [x] 看 rCore-N 的实现
    + [x] 运行 rCore-N + uartbenchmark
    + [x] 学习异步串口实现
+ [x] 独立crate
+ [ ] 移入 Embassy 运行时

### 一些基本概念

#### [串口](https://blog.csdn.net/fuhanghang/article/details/123274451)

+ 数据和控制信息是一位接一位地传送出去的
+ 速度会慢一些但传送距离相较并行口更长
+ 包括RS-232(D型9针串口)、RCA、RS-485、USB、VGA、HDMI、USB等

#### [uart](https://zhuanlan.zhihu.com/p/150504364)

+ Universal Asynchronous Receiver/Transmitter 通用异步收发器
+ 一种串行、异步、全双工的通信协议
+ 串行：数据一位一位的传送
+ 异步：以一个字符为传输单位、通信中两个字符间的时间间隔多少是不固定的，在同一个字符中两个相邻位之间的时间间隔是固定的。
+ 数据传送速率用波特率来表示，即每秒传送的二进制位数

#### svd

SVD文件描述了微控制器的硬件特征。它列出了所有可用的外围设备，包括每个设备的重要寄存器在内存中映射的位置，以及每个寄存器的功能。[例子](https://github.com/duskmoon314/rCore-N/blob/41796b85015a3e3080302270f9ab768827dd1426/pac/qemu-pac/qemu-16550.svd)

##### svd2rust [文档](https://docs.rs/svd2rust/latest/svd2rust/index.html)

通过使用该库，可以快速地将 一个 SVD文件转化成一个 crate（向外暴露safe API 的接口访问外围设备的库）

#### pac

通过 pac 中的接口操作寄存器，从而达到对串口硬件的操作。



### rCore-N 的异步串口驱动

[学习文档](../rCore-N.md) 



### 独立Crate   

[async-uart-driver](https://github.com/BITcyman/async-uart-driver/tree/main)

+ [x] pac 导入

+ [x] 独立 pac

+ [x] 移出 rCore-N 的 Buffered Sync and Async 

+ [ ] 使用 embassy 的运行时  

+ [ ] 改进机制

+ [ ] Alien 适配

    

#### embedded_hal_nb

+ [nb](https://docs.rs/nb/latest/nb/)：Minimal non-blocking I/O layer，代码复用，快速支持非阻塞和阻塞的接口的代码复用
+ [embedded_hal](https://docs.esp-rs.org/esp-idf-hal/embedded_hal/index.html)

#### 独立 pac 为单独的 crate 

+ 使用 svd2Rust 生成 PAC

    ```
    # 先安装 svd2rust, 如果 rustc 有版本问题记得更新到最新版本，可能的话还需要调默认的 rustc
    版本
    cargo install svd2rust
    
    # 安装 form
    cargo install from
    
    # 在只有 qemu-16550.svd 的文件夹下
    svd2rust --target=riscv -i qemu-16550.svd
    rm -rf src
    form -i lib.rs -o src/ && rm lib.rs
    cargo fmt
    ```

+ 编辑 `Cargo.toml` 

    ```
    [package]
    name = "qemu-16550-pac"
    version = "0.0.30"
    edition = "2021"
    authors = ["Gallium70"]
    
    [dependencies]
    bare-metal = "1.0.0"
    riscv = "0.10.0"
    vcell = "0.1.3"
    
    [features]
    board_qemu = []
    board_lrv = []
    
    ```

+ 注意调用时的名字要使用 `qemu_16550_pac` 

#### 移出 BufferedSerial

+ 使用原 BufferSerial 的结果

    ```
    [uart benchmark] User mode interrupt driver benchmark begin.
    [uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
    [uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
    [[uuaarrtt  llooaadd  32] Interrupt mode, claim result: ] Interrupt mode, claim result: 0x0x1000400010005000, enable res: , enable res: 0x0x0
    0
    [uart 3] intr, Intr count: 1, Tx: 0, Rx: 1, err pos: -1
    [uart 3] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
    [uart 2] intr, Intr count: 2, Tx: 0, Rx: 2, err pos: -1
    [uart 2] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
    [uart benchmark] User mode interrupt driver benchmark finished.
    Shell: Process 1 exited with code 0
    ```

+ 使用 独立 crate 中的 BufferSeiral 的结果

    ```
    uart benchmark] User mode interrupt driver benchmark begin.
    [uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
    [uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
    [uart load 3] Interrupt mode, claim result: 0x[uart load 10005000, enable res: 0x0
    2] Interrupt mode, claim result: 0x10004000, enable res: 0x0
    [uart 3] intr, Intr count: 2, Tx: 0, Rx: 2, err pos: -1
    [uart 3] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
    [uart 2] intr, Intr count: 1, Tx: 0, Rx: 1, err pos: -1
    [uart 2] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
    [uart benchmark] User mode interrupt driver benchmark finished.
    Shell: Process 11 exited with code 0
    ```

#### 移出 AsyncSerial 

+ 使用原 AsyncSerial 的结果

    ```
    [uart benchmark] User mode async driver benchmark begin.
    [uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
    [uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
    [uart load [uart load 23] Async mode, claim result: ]0 Axsync mode, claim result: 100050000x10004000, enable res: , enable res: 00xx0
    0
    [uart 3] Async, write: 22*247=5434, read: 1*247=247, refcnt: 6
    [uart 3] Async, Intr count: 2, Tx: 1, Rx: 1, err pos: -1
    [uart 3] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
    [uart 2] Async, write: 22*247=5434, read: 1*247=247, refcnt: 6
    [uart 2] Async, Intr count: 3, Tx: 1, Rx: 2, err pos: -1
    [uart 2] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
    [uart benchmark] User mode async driver benchmark finished.
    Shell: Process 1 exited with code 0
    ```

+ 使用 独立 crate 中的 AsyncSerial  的结果

    ```
    [uart benchmark] User mode async driver benchmark begin.
    [uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
    [uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
    [uart load 2] Async mode, claim result: 0x10004000, enable res: [0uart loadx 0
    3] Async mode, claim result: 0x10005000, enable res: 0x0
    [uart 3] Async, write: 22*247=5434, read: 1*247=247, refcnt: 6
    [uart 3] Async, Intr count: 2, Tx: 1, Rx: 1, err pos: -1
    [uart 3] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
    [uart 2] Async, write: 22*247=5434, read: 1*247=247, refcnt: 6
    [uart 2] Async, Intr count: 2, Tx: 1, Rx: 1, err pos: -1
    [uart 2] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
    [uart benchmark] User mode async driver benchmark finished.
    ```


#### 使用 Embassy 的 运行时

+ 分析当前的 crate 中使用的运行时机制
    + 当前 crate 中对于 Future 的使用方式为：接收到一个IO请求后生成一个新的Future，然后注册一个新的 waker 覆盖掉原有的 IO waker
    + 额外实现的 两个 Future (SerialReadFuture 和 SerialWriteFuture) 也仅仅是实现了 Future 的创建工作，以及记录 IO 相关的数据，并不直接操作 Rx/Tx Buf，对缓冲的相关操作仍需要调用 AsyncSerial 的相关接口
    + 中断服务例程在
+ Embassy 中实现的 uart 驱动
    + 首先由 BufferedUart 分成 BufferedUartRx 和 BufferedUartTx 两部分
        + 两部分 分别用于处理读操作和写操作，操作 State 中各自的部分
        + BufferedUart 只负责 new 和 配置State 
    + 然后会创建一个静态的 State
        + 结构体中包括：rx_buf、tx_buf、rx_waker和tx_waker

