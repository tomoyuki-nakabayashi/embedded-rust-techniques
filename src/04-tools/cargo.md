## 3-1. Cargo

Rustでの開発に[Cargo]は欠かせません。Cargoは、Rustのパッケージマネージャですが、それ以上のことができます。3rd party製のサブコマンド拡張をインストールすることで、Cargoの機能を拡張できます。ここでは、組込み / ベアメタルでのRust開発をより便利にするCargoに機能やサブコマンド拡張について紹介します。

[Cargo]: https://doc.rust-lang.org/cargo/index.html

### 設定ファイル

まず、欠かせないのが、設定ファイルです。どのような設定項目が書けるか、は[Cargo: 3.3 Configuration]に掲載されています。

[Cargo: 3.3 Configuration]: https://doc.rust-lang.org/cargo/reference/config.html

Cargo設定ファイルはTOML形式で記述し、プロジェクトの`.cargo/config`に作成することが多いです。実際は、階層的な作りになっています。どのような階層構造になっているか、は[Cargo: 3.3 Configuration 階層構造]を参照して下さい。

[Cargo: 3.3 Configuration 階層構造]: https://doc.rust-lang.org/cargo/reference/config.html#hierarchical-structure

組込み / ベアメタルでよく使う設定項目は、`target`と`build`です。

#### target.$triple

`target.$triple`はターゲットトリプルごとに、カスタムする内容を設定します。`$triple`の部分に、有効なターゲットトリプルを指定します。カスタムランナーの設定やコンパイルオプションの指定などに使います。例えば、Cortex-M3ターゲットの時は`qemu-system-arm`で、RISC-Vターゲットの時は`qmue-system-riscv32`を、それぞれカスタムランナーにしたい場合、次のように設定ファイルを記述します。

```toml
[target.thumbv7m-none-eabi]
runner = "qemu-system-arm -machine lm3s6965evb -nographic -kernel"

[target.riscv32imac-unknown-none-elf]
runner = "qemu-system-riscv32 -nographic -machine sifive_u -kernel"
```

これで、`cargo run --target thumbv7m-none-eabi`や`cargo run --target riscv32imac-unknown-none-elf`というコマンドを実行すると、QEMUでビルドしたバイナリを実行します。

次の4項目が設定できます。

- linker = ".."
- ar = ".."
- runner = ".."
- rustflags = ["..", ".."]

#### target.'cfg

`target.$triple`は、ターゲットトリプルを完全に指定する方法です。一方、`target.'cfg`は、条件を複数指定して、カスタマイズできます。

下記の例は、ターゲットアーキテクチャが32bitのARMで、OSなしのターゲットトリプル全てに適用されます。

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
rustflags = [
  "-C", "link-arg=-Tlink.x",
]
```

#### build

デフォルトのターゲットシステムが固定の場合、`cargo run --target thumbv7m-none-eabi`という長いコマンドを毎回入力するのは面倒です。そこで、`build`設定でデフォルトターゲットシステムを指定できます。

```toml
[build]
target = "thumbv7m-none-eabi"
```

`cargo run`で、`cargo run --target thumbv7m-none-eabi`と等価になります。

### binutils

組込み / ベアメタルの開発において、バイナリを調査することは、息をするより自然なことです。バイナリの調査を行う際、`objdump`、`size`、`readelf`、`nm`などのツールを利用します。GNUのbinutilsを使用しても良いのですが、LLVMのものを利用すると、rustcがサポートするターゲットアーキテクチャ全てに対応しており、便利です。

Cargoのサブコマンドである[cargo binutils]が提供されており、Cargoから`objdump`や`size`コマンドを利用できます。

[cargo binutils]: https://github.com/rust-embedded/cargo-binutils

インストールも非常に簡単です。

```
$ cargo install cargo-binutils
$ rustup component add llvm-tools-preview
```

プロジェクトバイナリ名が`app`の場合、次のように使用します。

```
$ cargo size --target thumbv7m-none-eabi --bin app
```

Cargo設定ファイルで、buildターゲットを指定している場合、次のコマンドで同じことができます。

```
$ cargo size --bin app
```

リリースビルドしたバイナリを調査する場合、`--release`を追加します。

```
$ cargo size --bin app --release
```

LLVMツール自体のオプションを使用する場合、空の`--`を入力した後にLLVMのオプションを指定します。

```
$ cargo objdump --bin app -- -d -no-show-raw-insn
#                         ^^^^^^^^^^^^^^^^^^^^^^^
```
