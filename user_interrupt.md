## 用户态中断 in rCore-N

+ 用户态程序调用 `sys_init_user_trap` 系统调用后初始化用户态中断

    + 主要是在进程控制块中保存的 `user_trap_info` 的初始化

        ```rust
        pub struct UserTrapInfo {
        
            pub user_trap_buffer_ppn: PhysPageNum,    
            // 一般保存在 TRAP_CONTEXT 下偏移1个page的 USER_TRAP_BUFFER处
            // 指向一个 UserTrapQueue 保存用户态下收到中断请求的 来源cause 和 信息msg
            pub devices: Vec<(u16, bool)>,
            // 用于保存设备用户态中断的使能情况
        }
        ```

+ 用户进程之间，可以靠 `sys_send_msg` 系统调用向某一进程发送信息

    + 指明 `pid` 和 `msg`

        ```rust
        pub fn send_msg(pid: usize, msg: usize) -> isize {
            sys_send_msg(pid, msg)
        }
        ```

    + 内核将把对应的 `msg` push入对应进程的 `UserTrapQueue` 中

        ```rust
        pub fn sys_send_msg(pid: usize, msg: usize) -> isize {
            if push_trap_record(
                pid,
                UserTrapRecord {
                    cause: pid << 4,
                    message: msg,
                },
            )
            .is_ok()
            {
                0
            } else {
                -1
            }
        }
        ```

+ 什么时候进行用户态中断的处理？

    + 当前内核态中断入口？
    + 用户态中断入口在什么地方设置的？

