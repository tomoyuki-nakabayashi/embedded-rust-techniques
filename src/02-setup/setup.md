# 環境構築

組込み開発では、ホストPCとは異なるアーキテクチャのバイナリを生成しなければならないため、
いくつかのツールが必要になります。
また、バイナリを解析したり、逆アセンブリを行ってデバッグを行う際に、便利なツールも用意しておくと良いでしょう。

ターゲットにできるアーキテクチャは多岐に渡りますし、読者が開発を行う環境もLinux / Mac / Windowsとバリエーションが多いです。
これを本書で網羅することはできないため、(1) 用意するもの、(2) インストール手順を記載したWebサイトへのリンク、
(3) その他備考、のみを示します。

まず、用意するもののリストは、次の通りです。

- Rust (クロスコンパイルツールチェイン含む)
- GDB
- デバッグフレームワーク (OpenOCD, JLinkなど)
- cargo-binutils
- QEMU

Cortex-Mをターゲットとするこれらのインストール手順は、[The Embedded Rust Bookのインストール]に記載されています。

[The Embedded Rust Bookのインストール]: https://tomoyuki-nakabayashi.github.io/book/intro/install.html

## Rust

Rustはクロスコンパイルが簡単な言語ですが、デフォルトのインストールでは、ホストマシンのネイティブコンパイルのみをサポートしています。
そのため、ターゲットとするクロスコンパイラを追加するために、`rustup`でターゲットを追加します。
例えば、ARMのCortex-M0であれば、次の通りです。

```
$ rustup target add thumbv6m-none-eabi 
```

ここで、ハマりどころがあります。

1. Rustがサポートするターゲットシステムの一覧がわからない
2. ターゲットシステムがサポートされていない

これらの詳細は、[コンパイラサポート]に記載しますが、解決方法を簡単にだけ示します。
まず、ターゲットシステム一覧は、次のコマンドで取得できます。

```
$ rustc --print target-list
```

次に、ターゲットシステムがサポートされていない場合ですが、ターゲットの`specification`をJSON形式のファイルで用意します。

[コンパイラサポート]: ../04-tools/compiler.md

## GDB

読者の中には、LLDBに慣れ親しんだ方も居るかと思います。
通常のデバッグに関して、LLDBはGDBと同水準の機能があります。
しかし、ターゲットハードウェアにプログラムをアップロードするGDBの`load`コマンド相当のものが、LLDBにはありません。
そのため、マイクロコントローラのファームウェア開発に限っては、GDBの利用をおすすめします。

## デバッグフレームワーク

マイクロコントローラ上で動作するプログラムをGDBでデバッグするためには、
SWD (Serial Wire Debug) やJTAGプロトコルを使って、*GDBサーバー*のサービスを提供するソフトウェアが必要になります。

このようなソフトウェアで、主要なものとしては、OpenOCDとJLinkがあります。
どちらも、Rustで作成したプログラムをデバッグすることが可能です。

ターゲットとするマイクロコントローラの開発で使いやすい方を選択して下さい。
[Discovery環境構築]では、OpenOCDの環境構築方法が記載されています。

[Discovery環境構築]: https://tomoyuki-nakabayashi.github.io/discovery/03-setup/index.html

## cargo-binutils

[cargo-binutils]は、LLVM binary utilitiesを簡単に利用するためのCargoサブコマンドです。
`llvm-objdump`や`llvm-size`などをCargoから呼び出すことができます。

[cargo-binutils]: https://github.com/rust-embedded/cargo-binutils

ターゲットアーキテクチャ用のGNU binutilsがインストールされており、そのコマンドに慣れている場合、
無理に使う必要はありません。
しかし、Rustでバイナリハックする上で、ターゲットアーキテクチャに依存せず、同じコマンドで利用できる、というのは大きなメリットです。

## QEMU

[QEMU]は、有名なエミュレータです。
実際のハードウェアで開発を行う前に、実験を行う場合に重宝します。

[QEMU]: https://www.qemu.org/