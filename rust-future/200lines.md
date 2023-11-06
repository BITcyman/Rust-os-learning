# 《200行代码讲透RUST FUTURES》 笔记
[资料链接](https://stevenbai.top/rust/futures_explained_in_200_lines_of_rust)

### 异步实现的方法
+ 使用os提供的线程  Thread
+ 绿色线程  Green Thread
+ 函数回调  callback
    + 回调地狱问题
+ Promise