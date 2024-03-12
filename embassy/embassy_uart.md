## Embassy_uart

/embassy-stm32/src/usart/buffered.rs

#### Embassy为stm32编写的BufferedUart

由 BufferedUartRx 和 BufferedUartTx 两部分构成，向 BufferedUart 提出的IO请求会分配给 BufferedUartRx 和 BufferedUartTx 进行处理，同时对同步、异步IO的实现都在一个结构体中给出。

#### 使用测例

/tests/stm32/src/bin/uart_dma.rs

```rust
	let (mut tx, mut rx) = usart.split();

    for n in 0..42 {
        for i in 0..LEN {
            tx_buf[i] = (i ^ n) as u8;
        }

        let tx_fut = async {
            tx.write(&tx_buf).await.unwrap();
        };
        let rx_fut = async {
            rx.read(&mut rx_buf).await.unwrap();
        };


        join(rx_fut, tx_fut).await;

        assert_eq!(tx_buf, rx_buf);
    }
```

在测试用例中，初始化串口后，通过 split 接口获得了 tx 和 rx 的使用地址，接下来的串口读写也直接从 tx 和 rx 中进行读写。



#### 针对之前提出的一些疑问的回答

+ 对OS和驱动的边界不明晰
    + rx和tx的既可以在驱动中初始化，也可以通过一些接口将地址提供给OS，供OS使用







