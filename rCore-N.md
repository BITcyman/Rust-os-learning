## rCore-N学习

#### rCore-N 异步驱动 [链接](https://github.com/duskmoon314/rCore-N/blob/41796b85015a3e3080302270f9ab768827dd1426/user/src/user_uart.rs#L995)

目前没跑起来，用`LOG=DEBUG just run`启动内核后，发现内核在初始化`trap`中的 `sideleg` 寄存器时寄了，查找 riscv 仓库中没有该寄存器，发现用的仓库是 fork 原仓库后手动添加了 [sideleg 模块](https://github.com/duskmoon314/riscv/blob/extN/src/register/sideleg.rs)。

原来得用仓库里给的qemu（哭），但目前编译 qemu 出现问题。

