## Alien 上板

### 安装 mkimage
sudo apt-get install  u-boot-tools

### 安装 tftp 服务器
1. sudo apt-get install tftp-hpa tftpd-hpa xinetd
2. 配置/etc/default/tftpd-hpa

``` 
TFTP_USERNAME="tftp"
TFTP_DIRECTORY="/work/tftpboot"   //修改成自己要使用的tftpboot目录
TFTP_ADDRESS="0.0.0.0:69"
TFTP_OPTIONS="--secure -c"   //-c 表示可以创建文件 -c  create
```
3. 重启服务 sudo /etc/init.d/tftpd-hpa start
4. 测试 tftp

在自己使用的tftpboot目录下创建一个 test.md 文件，然后在另外一个目录下 tftp 127.0.0.1，进入 tftp 后尝试 get test.md，quit 后查看当前目录下是否有 test.md 文件，有则表明 tftp 服务器正常启动。

[链接](https://www.cnblogs.com/jalynfang/p/9450528.html)


### 安装minicom  
将板子上的串口 <-> 串口转USB <-> ubuntu 笔记本的USB口相连，在 Ubuntu 上使用 minicom 来向板子输入指令。

[安装链接](https://blog.csdn.net/yinminsumeng/article/details/128931916)

参照上面的链接安装并检测后发现串口插入后就立刻断连了，
```
[  256.706676] usb 1-1.4.1: ch341-uart converter now attached to ttyUSB0
[  257.319324] ch341-uart ttyUSB0: ch341-uart converter now disconnected from ttyUSB0
```

参照这篇[教程](https://blog.csdn.net/zym787/article/details/128854952)，发现已有一个brlyyu进程占用了这个串口号，将其删除就行了

sudo apt remove brltty

### 编译内核

修改 Alien 根目录下 Makefile 文件中的 TFTPBOOT，
然后进入 tools\initrd 目录下进行 make all，后回到根目录下 make vf2


#### 启动
启动能看到OpenSBI，uboot正常启动，加载内核后 go 0x40200000 不继续执行。

tftp传入的Alien大小只有1M左右，感觉不太对劲，应该是编译参数有问题
