## 5-3. svd2rust

[SVD] (System View Description) ファイルからRustの**struct**を自動生成するツールです。SVDファイルはXMLファイルで、特にペリフェラルのメモリマップドレジスタの記述を形式化したものです。

[SVD]: http://www.keil.com/pack/doc/CMSIS/SVD/html/index.html

[svd2rust]は、Cortex-M, MSP430, RISCVのマイクロコントローラに対応しています。**svd2rust**で自動生成されたクレートは、**PAC (Peripheral Access Crate)** と呼ばれています。主要なPACは、[Peripheral Access Crates]にまとめられています。

[svd2rust]: https://docs.rs/svd2rust/0.14.0/svd2rust/
[Peripheral Access Crates]: https://github.com/rust-embedded/awesome-embedded-rust#peripheral-access-crates

ちょっとしたプログラムを書く場合、**svd2rust**から生成されたPACは間違いを犯しにくいです。**svd2rust**で生成されたレジスタアクセス関数では、数値ではなく*クロージャ*を引数に取ります。例えば、GPIOピン (8番ピン) を出力設定にして、highレベルを出力するコードは、次のようになります。

```rust,ignore
    // ピンを出力に設定します
    gpioe.moder.modify(|_, w| {
        w.moder8().output();
    });

    // LEDを点灯します
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
    });
```

クロージャを引数に取る利点は、**modify()**メソッドの利用時にあります。**modify()**メソッドは、メモリマップドレジスタのリード・モディファイ・ライトを行うAPIです。操作対象のレジスタがクロージャ内でしか操作できないため、別レジスタを誤って操作するような事故が発生しません。

単純なレジスタ読み書きより複雑なコードに見えますが、コンパイラの最適化により、リリースビルドされたバイナリは、通常のレジスタアクセスと同等の機械語になります。

[Discovery]では、**svd2rust**で生成したPACを利用して、LEDを点灯したり、シリアル通信します。

[Discovery]: https://tomoyuki-nakabayashi.github.io/discovery/

後述する[RTFM for ARM Cortex-M]でも、**svd2rust**で生成したPACを利用します。

[RTFM for ARM Cortex-M]: https://github.com/japaric/cortex-m-rtfm