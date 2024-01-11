## 将异步串口驱动独立出来

##### Todo List

+ [x] 学习一些基本的概念
    + [x] 串口
    + [x] uart
    + [x] pac
    + [x] svd
+ [ ] 看 rCore-N 的实现
+ [ ] 生成一版技术方案
+ [ ] 写独立的crate

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

![uart](..\img\uart.png)



#### svd

SVD文件描述了微控制器的硬件特征。它列出了所有可用的外围设备，包括每个设备的重要寄存器在内存中映射的位置，以及每个寄存器的功能。[例子](https://github.com/duskmoon314/rCore-N/blob/41796b85015a3e3080302270f9ab768827dd1426/pac/qemu-pac/qemu-16550.svd)

##### svd2rust [文档](https://docs.rs/svd2rust/latest/svd2rust/index.html)

通过使用该库，可以快速地将 一个 SVD文件转化成一个 crate（向外暴露safe API 的接口访问外围设备的库）

#### pac

通过 pac 中的接口操作寄存器，从而达到对串口硬件的操作。













