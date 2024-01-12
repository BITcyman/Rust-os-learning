## rCore-N学习

### rCore-N 异步串口驱动 [链接](https://github.com/duskmoon314/rCore-N/blob/41796b85015a3e3080302270f9ab768827dd1426/user/src/user_uart.rs#L995)

#### 运行

+ 编译QEMU with extN support

目前没跑起来，用`LOG=DEBUG just run`启动内核后，发现内核在初始化`trap`中的 `sideleg` 寄存器时寄了，查找 riscv 仓库中没有该寄存器，发现用的仓库是 fork 原仓库后手动添加了 [sideleg 模块](https://github.com/duskmoon314/riscv/blob/extN/src/register/sideleg.rs)。

原来得用仓库里给的qemu（哭），但目前编译 qemu 出现问题。

看了一下qemu源码发现是由于在把分配的一定空间的buf的首地址强制转化成对应结构体指针的过程中，由于开的buf大小不够放入整个结构体（其实结构体后半部分也确实用不到），就出了这个bug。

![379868a3499d4c3a5549a0724380c72](\img\qemu_build_error.png)

解决方式：在编译时加入额外的参数即可

../qemu/configure --target-list="riscv64-softmmu" --extra-cflags=-Wno-error  
make -j

+ 编译 rCore-N 启动内核

![image-20240112104045277](\img\rCore-N_run.png)

依照上述步骤，得到如下结果

![启动！](\img\rCore-N_start!.png)

+ 运行异步串口测试程序 uart_benchmark

![](\img\rCore-N_uartbenchmark.png)

#### [学习异步串口的实现](https://github.com/duskmoon314/rCore-N/blob/41796b85015a3e3080302270f9ab768827dd1426/user/src/user_uart.rs) 

