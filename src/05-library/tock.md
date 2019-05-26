## Tock

[Tock]はRust製の組込みOSです。
Cortex-Mアーキテクチャに対応しています。
長期に渡り開発が進められており、2018年2月時点でversion 1.0がリリースされています。

[Tock]: https://www.tockos.org/
[GitHub Tock]: https://github.com/tock

### 主な機能

> Tockは現状RTOSではありません。

### design

#### loadable application

組込みOSの中には、kernelとアプリケーションを1つのファームウェアとしてビルドするものも多く存在します。
Tockは、kernelとアプリケーションを別々にビルドすることができる仕組みになっています。

### サンプルコード