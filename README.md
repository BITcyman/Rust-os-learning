# Rust-os-learning

[2024春季学期计划](./plan/2024春季学期计划-v1-0305.md)



### 24 春季学期

#### 第十一周 5.8-5.14

| 序号 | 任务                                                         | 状态    | 结果                                                         |
| ---- | ------------------------------------------------------------ | ------- | ------------------------------------------------------------ |
| 1    | 完成毕业论文初稿                                             | 13500字 | [论文大纲](https://github.com/BITcyman/Rust-os-learning/blob/main/report/paper/paper-outline.md)    [论文](https://github.com/BITcyman/Rust-os-learning/tree/main/report/paper) |
| 2    | Alien 上板[论文大纲](https://github.com/BITcyman/Rust-os-learning/blob/main/report/paper/paper-outline.md)    [论文](https://github.com/BITcyman/Rust-os-learning/tree/main/report/paper) | 完成    | [过程记录](./board/starfive.md)                              |
| 3    | 准备演示                                                     | 正在搞  |                                                              |
| 4    | 板子上的异步串口驱动                                         |         |                                                              |



#### 第十周 5.1-5.7

| 序号 | 任务                        | 状态 | 结果                                                         |
| ---- | --------------------------- | ---- | ------------------------------------------------------------ |
| 1    | 异步串口驱动在 Alien 中使用 | 完成 | [过程记录](./driver/runtime.md)                              |
| 2    | 写了一部分毕业论文          |      | [论文大纲](https://github.com/BITcyman/Rust-os-learning/blob/main/report/paper/paper-outline.md)    [论文](https://github.com/BITcyman/Rust-os-learning/tree/main/report/paper) |



#### 第九周 4.24-4.30

| 序号 | 任务                                                         | 状态           | 结果                   |
| ---- | ------------------------------------------------------------ | -------------- | ---------------------- |
| 1    | paper-translation                                            | 完成           | 正常显示公式、评论功能 |
| 2    | 异步串口驱动在 Alien 中使用                                  | [遇到问题](./driver/runtime.md#4-30 目前最主要的问题) |                        |
| 3    | 推进内核赛 [Kinako](https://github.com/BITcyman/OSKernel2024-Kinako) | 进行中  | 进展：ch3              |



#### 第八周 4.17-4.23

| 序号 | 任务                                                         | 状态   | 结果                                                         |
| ---- | ------------------------------------------------------------ | ------ | ------------------------------------------------------------ |
| 1    | paper-translation                                            | 完成   | [页面](https://cyman-paper-translation.github.io/main-page/) |
| 2    | 使用 Embassy 模式改进异步串口驱动                            | 完成   | [commit](https://github.com/BITcyman/async-uart-driver/commit/3d1265d17e6b2d6e1ce8df351f6e6d19d04136ce) |
| 3    | 推进内核赛 [Kinako](https://github.com/BITcyman/OSKernel2024-Kinako) | 进行中 | 进展：ch2                                                    |




#### 第七周 4.10-4.16

| 序号 | 任务                       | 状态 | 结果                                                         |
| ---- | -------------------------- | ---- | ------------------------------------------------------------ |
| 1    | 毕设外文翻译               | 完成 | [外文翻译](./report/文献翻译.md)                             |
|      |                            |      | [gitbook 链接](https://lins-organization-5.gitbook.io/translations/) |
| 2    | 异步串口驱动 使用 ats-intc | 完成 | [分支链接](https://github.com/BITcyman/async-uart-driver/tree/ats-ints) |



#### 第六周 4.3-4.9

| 序号 | 任务                     | 状态     | 结果                                                         |
| ---- | ------------------------ | -------- | ------------------------------------------------------------ |
| 1    | 毕设外文翻译             | 正在进行 | [外文翻译](./report/文献翻译.md)                             |
| 2    | 准备中期汇报             | 正在进行 | [中期汇报表格](./report/中期汇报表格.md)<br />[中期汇报ppt](./report/毕业设计中期报告.pptx) |



#### 第五周 3.26-4.2

| 序号 | 任务         | 状态     | 结果                             |
| ---- | ------------ | -------- | -------------------------------- |
| 1    | 毕设外文翻译 | 正在进行 | [外文检索](./report/外文检索.md) |
|  |  |  | [外文翻译](./report/文献翻译.docx) |
| 2    | 改进串口驱动 | 正在进行 |                                  |
| 3    | 准备中期汇报 | 正在进行 | [中期汇报表格](./report/中期汇报表格.md)  |

#### 第四周 3.19-3.25

| 序号 | 任务     | 状态 | 结果 |
| ---- | -------- | ---- | ---- |
| 1    | 考研复试 |      |      |

#### 第三周 3.12-3.18

| 序号 | 任务                                                         | 状态   | 结果                       |
| ---- | ------------------------------------------------------------ | ------ | -------------------------- |
| 1    | 学习星光二上目前已有的 [PAC](https://codeberg.org/weathered-steel/jh71xx-pac) 和 [HAL](https://codeberg.org/weathered-steel/jh71xx-hal) | 已完成 | [文档](./Vision_Five2.md) |
| 2    | 准备考研复试                                                 |        |                            |
|      |                                                              |        |                            |

#### 第二周 3.5-3.11

| 序号 | 任务                  | 状态                                       | 结果                                                         |
| ---- | --------------------- | ------------------------------------------ | ------------------------------------------------------------ |
| 1    | 异步串口驱动crate     | 独立Crate已完成<br />正在移入Embassy运行时 | [文档](./driver/uart-crate.md)<br />[仓库](https://github.com/BITcyman/async-uart-driver) |
| 2    | OS内核赛报名 - Kinako | 已完成                                     | [github仓库](https://github.com/BITcyman/OSKernel2024-Kinako)<br />[gitlab仓库](https://gitlab.eduxiji.net/T202410007992527/oskernel2024-kinako) |
|      |                       |                                            |                                                              |

#### 第一周 2.26-3.4

| 序号 | 任务                           | 状态   | 结果                                                   |
| ---- | ------------------------------ | ------ | ------------------------------------------------------ |
| 1    | 完成学期计划                   | 已完成 | [2024春季学期计划](./plan/2024春季学期计划-v1-0305.md) |
| 2    | 学习了embassy中stm32的uart驱动 | 已完成 | [笔记](./embassy/embassy_uart.md)                      |
| 3    | 把串口驱动写成独立的 crate     | 进行中 |                                                        |



### 23 秋季学期

#### 寒假  1.27 - 2. 25

#### 第四周 1.17-1.26

| 序号 | 任务         | 状态   | 结果                                           |
| ---- | ------------ | ------ | ---------------------------------------------- |
| 1    | 毕设开题工作 | 已完成 | [开题报告.md](./report/毕业设计开题报告.docx)  |
|      |              |        | [开题报告.ppt](./report/毕业设计开题报告.pptx) |

#### 第三周 1.9 - 1.16

| 序号 | 任务                       | 状态   | 结果                                                         |
| ---- | -------------------------- | ------ | ------------------------------------------------------------ |
| 1    | 学习 rCore-N 串口驱动      | 已完成 | [rCore-N.md](rCore-N.md)                                     |
| 2    | 把串口驱动写成独立的 crate | 进行中 | [文档](./driver/uart-crate.md)<br />[仓库](https://github.com/BITcyman/async-uart-driver) |
| 3    | 编写开题报告               | 进行中 | [开题报告.md](./report/毕业设计开题报告.docx)                |

#### 第二周 2024.1.2 - 1.9

| 序号 | 任务                                       | 状态   | 结果                                                         |
| ---- | ------------------------------------------ | ------ | ------------------------------------------------------------ |
| 1    | 结合把 Embassy 引入 rcore 继续学习 Embassy | 已完成 | [embassy.md](./embassy/embassy.md)<br />[embassy-into-rcore.md](./embassy/embassy-into-rcore.md) |
| 2    | 尝试看 fu740 的网络驱动                    | 进行中 |                                                              |

#### 第一周 2023.12.23 - 2024.1.2

| 序号 | 任务                          | 状态   | 结果                                     |
| ---- | ----------------------------- | ------ | ---------------------------------------- |
| 1    | 学习协程实现                  | 已完成 | [200lines.md](./rust-future/200lines.md) |
| 2    | 看 Embassy 文档，运行示例代码 | 进行中 |                                          |





