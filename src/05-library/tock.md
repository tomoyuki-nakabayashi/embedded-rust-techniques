## Tock

[Tock]はRust製の組込みOSです。
Cortex-Mアーキテクチャに対応しており、[RISC-Vへの移植]も進められています。
長期に渡り開発が進められており、2018年2月時点でversion 1.0がリリースされています。

[Tock]: https://www.tockos.org/
[GitHub Tock]: https://github.com/tock
[RISC-Vへの移植]: https://github.com/tock/tock/issues/1135

Tockのビルドにはnightlyツールチェインが必要です。

### 主な対応ボード

対応ボードは、既存のRTOSと比較すると多くはありません。
一部を紹介します。

- Hail
- TI LAUNCHXL CC26x2 / CC13x2 SimpleLink
- Nordic nRF52x
- STM32 Nucleo
- HiFive1

HiFiveはRISC-Vで、他はARM Cortex-Mが搭載されたボードです。

### 設計概要

<p align="center">
<img title="Tock stack" src="../assets/tock-stack.png">
</p>

> https://github.com/tock/tock/blob/master/doc/tock-stack.pngより

TockのKernelはRustで実装されています。
Kernelは2つの階層に分割されています。

1つは、Core kernelでHAL (Hardware Abstraction Layer) 、スケジューラ、プラットフォーム固有の設定が含まれます。

もう1つは、Capsuleです。
Capsuleは、マイコンに依存しないkernel機能を拡張するためのコンポーネント、という位置づけで、通信スタックやコンソールなどが該当します。
Capsuleは、`unsafe`ブロックの使用が禁止されているなど、Rust固有の安全性を保証する設計が選定されています。

Tockでは、Capsuleもユーザプロセスも`Untrusted`という扱いですが、その中でも差が設けられています。
Capsuleは、kernelのイベントループの中で協調的にスケジューリングされます。
そのため、Capsuleがパニックしたり、イベントハンドラに戻らない場合、システムの回復には再起動が必要です。
一方で、ユーザプロセスは、MPUでメモリが隔離されており、スケジューリングもプリエンプティブです。

### ドキュメント

[Tock Documentation]に、Tockの設計や実装に関するドキュメントがまとめられています。

[Tock Documentation]: https://github.com/tock/tock/tree/master/doc

[Tock Implementation]には、Tockの実装についての解説があり、RustでOSを実装する際に参考にできる情報がまとめられています。
ここでは、OS実装に必要な要素について、Tock内でどのようにRustで実装しているか、が述べられています。
例えば、ライフタイムや可変参照というRust固有の要素をどう扱っているか、メモリアイソレーションやメモリップドレジスタ、システムコールをRustでどのように実装しているか、というトピックが取り上げられています。

[Tock Implementation]: https://github.com/tock/tock/tree/master/doc#tock-implementation

### ユーザランドアプリケーション

ユーザランドアプリケーションはCとRust、どちらでも書くことができます。
C言語用のユーザランドライブラリ[libtock-c]には、`newlib`や`libc++`、`lua`ライブラリが含まれます。

[libtock-c]: https://github.com/tock/libtock-c

[libtock-rs]は、2019年5月現在、`WIP`の状態です。

[libtock-rs]: https://github.com/tock/libtock-rs

### コラム〜TockはRTOSじゃない！？〜

> Tockは現状RTOSではありません。