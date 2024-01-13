## rCore-N学习

### rCore-N 异步串口驱动 [链接](https://github.com/duskmoon314/rCore-N/blob/41796b85015a3e3080302270f9ab768827dd1426/user/src/user_uart.rs#L995)

#### 运行

+ 编译QEMU with extN support

目前没跑起来，用`LOG=DEBUG just run`启动内核后，发现内核在初始化`trap`中的 `sideleg` 寄存器时寄了，查找 riscv 仓库中没有该寄存器，发现用的仓库是 fork 原仓库后手动添加了 [sideleg 模块](https://github.com/duskmoon314/riscv/blob/extN/src/register/sideleg.rs)。

原来得用仓库里给的qemu（哭），但目前编译 qemu 出现问题。

看了一下qemu源码发现是由于在把分配的一定空间的buf的首地址强制转化成对应结构体指针的过程中，由于开的buf大小不够放入整个结构体（其实结构体后半部分也确实用不到），就出了这个bug。

![qemu_build_error](/img/qemu_build_error.png)

解决方式：在编译时加入额外的参数即可

../qemu/configure --target-list="riscv64-softmmu" --extra-cflags=-Wno-error  
make -j

+ 编译 rCore-N 启动内核

![rCore-N_run](/img/rCore-N_run.png)

依照上述步骤，得到如下结果

![rCore-N_start!](/img/rCore-N_start!.png)

+ 运行异步串口测试程序 uart_benchmark

![](/img/rCore-N_uartbenchmark.png)

#### [学习异步串口的实现](https://github.com/duskmoon314/rCore-N/blob/41796b85015a3e3080302270f9ab768827dd1426/user/src/user_uart.rs) 

##### 研究 uart_benchmark

```
# /dev/pts/4

# 在 shell 中 输入 uart_benchmark 并敲下回车
[DEBUG 0]: Fork start
[DEBUG 0]: forked task cx ptr: 0xffffffffffff4f58
[DEBUG 0]: new_task 2 via fork	 # os/src/syscall/process.rs/sys_fork
[DEBUG 1]: EXEC uart_benchmark	 # os/src/syscall/process.rs/sys_exec
# 创建 uart_benchmark 进程结束，开始执行

# 接下来初始化用户态中断
# spwan("cpu_load\0")
[DEBUG 1]: SPAWN exec "cpu_load"
[DEBUG 1]: new_task via spawn 3 # os/src/syscall/process.rs/sys_spawn new_pid = 3
# spwan("uart_load\0")
[DEBUG 2]: SPAWN exec "uart_load"
[DEBUG 2]: new_task via spawn 4 # os/src/syscall/process.rs/sys_spawn new_pid = 4
# spwan("uart_load\0")
[DEBUG 2]: SPAWN exec "uart_load"
[DEBUG 2]: new_task via spawn 5 # os/src/syscall/process.rs/sys_spawn new_pid = 5


[DEBUG 3]: [push trap record] pid: 4, cause: 64, message: 72
[DEBUG 3]: [push trap record] pid: 5, cause: 80, message: 80
[DEBUG 0]: [syscall claim] mapping device 14 to pid 4
[DEBUG 3]: [syscall claim] mapping device 15 to pid 5
[DEBUG 0]: [push trap record] pid: 4, cause: 8, message: 14
[DEBUG 2]: [push trap record] pid: 5, cause: 8, message: 15
[DEBUG 0]: [SET EXT INT] dev: 14, enable: 1
[DEBUG 3]: [SET EXT INT] dev: 15, enable: 1
[DEBUG 0]: set UTIP for pid 5
[DEBUG 2]: set UTIP for pid 4
[ INFO 3]: pid: 5 exited with code 0, time intr: 125, cycle count: 2754088817
[ INFO 0]: pid: 4 exited with code 0, time intr: 171, cycle count: 3671655240
[DEBUG 2]: [push trap record] pid: 3, cause: 48, message: 15
[ INFO 2]: pid: 3 exited with code 216821212, time intr: 170, cycle count: 3741433626
[ INFO 3]: pid: 2 exited with code 0, time intr: 23926, cycle count: 3457064619



#/dev/pts/5
>> uart_benchmark
# spwan("cpu_load\0") 结束
[uart benchmark] User mode unbuffered async driver benchmark begin.
# spwan("uart_load\0")
[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...

[uart load] trap init result: 0xffffffffffffd000, now waiting for config init...
[uart load [uart load 23] Async mode, claim result: 0]x Async mode, claim result: 100040000x, enable res: 0x100050000, enable res: 0x
0
[uart 3] Unbuffered Async, refcnt: 4
[uart 3] Unbuffered Async, Intr count: 1, Tx: 1, Rx: 0
[uart 3] Test finished, 16 bytes sent, 16 bytes received, 0 bytes error.
[uart 2] Unbuffered Async, refcnt: 3
[uart 2] Unbuffered Async, Intr count: 2, Tx: 1, Rx: 1
[uart 2] Test finished, 16 bytes sent, 1 bytes received, 0 bytes error.
[uart benchmark] User mode unbuffered async driver benchmark finished.
Shell: Process 2 exited with code 0
```



##### 用户态中断 [文档](./user_interrupt.md)

