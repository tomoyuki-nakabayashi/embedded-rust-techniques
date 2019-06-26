## Tock

[Tock]はRust製の組込みOSです。
Cortex-Mアーキテクチャに対応しています。
[RISC-Vへの移植]も進められているようです。
長期に渡り開発が進められており、2018年2月時点でversion 1.0がリリースされています。

[Tock]: https://www.tockos.org/
[GitHub Tock]: https://github.com/tock
[RISC-Vへの移植]: https://github.com/tock/tock/issues/1135

### 主な機能

### design

TockのKernelはRustで実装されています。
Kernelも2つの階層に分割されています。
1つは、Core kernelでHAL (Hardware Abstraction Layer) 、スケジューラ、プラットフォーム固有の設定が含まれます。
もう1つは、Capsuleです。
Capsuleは、kernelのイベントループの中で協調的にスケジューリングされます。
そのため、Capsuleがパニックしたり、イベントハンドラに戻らない場合、システムの回復には再起動が必要です。

#### loadable application

組込みOSの中には、kernelとアプリケーションを1つのファームウェアとしてビルドするものも多く存在します。
Tockは、kernelとアプリケーションを別々にビルドすることができる仕組みになっています。

### サンプルコード

ユーザランドアプリケーションはCとRust、どちらでも書くことができます。
C言語用のユーザランドライブラリ[libtock-c]には、`newlib`や`libc++`、`lua`ライブラリが含まれます。

[libtock-c]: https://github.com/tock/libtock-c

[libtock-rs]は、2019年5月現在、`WIP`の状態です。

[libtock-rs]: https://github.com/tock/libtock-rs

### コラム〜TockはRTOSじゃない！？〜

> Tockは現状RTOSではありません。