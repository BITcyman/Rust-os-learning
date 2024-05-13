## 星光二板子上使用异步串口驱动

默认的 UART 使用的GPIO引脚为 GND、GPIO5(UART TX) 和 GPIO6(UART RX)，对应的引脚号为6、8、10

在[用户手册]((https://doc.rvspace.org/VisionFive2/PDF/VisionFive2_40-Pin_GPIO_Header_UG.pdf))的第8章中指出最多支持6路UART，将一些未使用的 GPIO 配置为 UART，需要修改 dts 文件。



#### 相关资料
+ [GPIO引脚分布信息](https://doc.rvspace.org/VisionFive2/Datasheet/VisionFive_2/gpio_pin_assig.html)
+ [昉·星光 2 40-Pin GPIO Header用户手册](https://doc.rvspace.org/VisionFive2/PDF/VisionFive2_40-Pin_GPIO_Header_UG.pdf)
+ [jh7110-starfive-visionfive-2.dtsi](https://github.com/starfive-tech/linux/blob/JH7110_VisionFive2_devel/arch/riscv/boot/dts/starfive/jh7110-starfive-visionfive-2.dtsi)