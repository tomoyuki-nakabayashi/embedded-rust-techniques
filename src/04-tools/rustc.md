## rustc

rustcは、Rustのコンパイラです。
組込み / ベアメタルプログラミングに限りませんが、コンパイラでできることを知っていると便利なことがあります。

### コマンドライン引数

コマンドライン引数を直接rustcに指定する機会は少ないです。
多くの場合、Cargoの設定を記述し、間接的にrustcのコマンドライン引数を使用します。
しかし、rustcのコマンドライン引数を把握していなければ、Cargoから使いようもありません。
網羅的な説明は、[コマンドライン引数]を参照して下さい。

[コマンドライン引数]: https://doc.rust-lang.org/stable/rustc/command-line-arguments.html

組込み / ベアメタルプログラミングで最も大事なコマンドライン引数は、間違いなく`-C / --codegen`です。
このコマンドライン引数では、使用するリンカの指定や最適化レベルなどの制御ができます。
詳しくは、[コード生成オプション]に記載します。

[コード生成オプション]: rustc.html#コード生成オプション

組込みLinux開発でターゲットシステムのライブラリに依存するクレートをクロスビルドするのであれば、
`--sysroot`オプションでsystem rootのパスを上書きすることができます。

コンパイラより厳密なソースコード検査をするために、コマンドライン引数からlintルールごとに、lintレベルを制御できます。
`-A`, `-W`, `-D`, `-F`フラグがあり、それぞれ、許可、警告、拒絶、禁止を意味します。
lintルールごとに、これらのフラグを設定できます。詳細は、[lint]で紹介します。
下に例を示します。

```
$ rustc lib.rs --crate-type=lib -W missing-docs
```

[lint]: rustc.html#lint

### lint

`lint`はソースコードをコンパイラより厳密なルールに則り、検査するためのツールです。
Rustコンパイラには、様々なlintルールが組み込まれています。
ソースコードをコンパイルする時、自動的にlintによる検査が行われます。

プロジェクトの運用ルールに合わせて、適切なlintルールを設定することで、ソースコードの品質をより向上できるでしょう。

### lintレベル

rustcのlintレベルは、4つに分類されます。

1. allow (許可)
2. warn (警告)
3. deny (拒絶)
4. forbid (禁止)

各lintルールには、デフォルトのlintレベルがあり、コンパイルオプションかアトリビュートで上書きできるようになっています。
まず、lintレベルについて説明します。

#### allow (許可)

lintルールを適用しません。
例えば、次のコードをコンパイルしても、警告は発生しません。

```rust,ignore
pub fn foo() {}
```

```
$ rustc lib.rs --crate-type=lib
```

しかし、このコードは`missing_docs`ルールを違反しています。
lintレベルを上書きしてコンパイルすると、コンパイルエラーになったり、警告が出力されるようになります。

#### warn (警告) 

lintルール違反があった場合、警告を表示します。

```rust
fn main() {
    let x = 5;
}
```

このコードは`unused_variables`のルールに違反しており、次の警告が報告されます。

```
warning: unused variable: `x`
 --> src/main.rs:2:9
  |
2 |     let x = 5;
  |         ^ help: consider prefixing with an underscore: `_x`
  |
  = note: #[warn(unused_variables)] on by default
```

#### deny (拒絶)

lintルール違反があった場合、コンパイルエラーになります。

```rust
fn main() {
    100u8 << 10;
}
```

このコードは、`exceeding_bitshifts`ルールに違反しており、コンパイルエラーになります。

```
error: attempt to shift left with overflow
 --> src/main.rs:2:5
  |
2 |     100u8 << 10;
  |     ^^^^^^^^^^^
  |
  = note: #[deny(exceeding_bitshifts)] on by default
```

#### forbid (禁止)

lintルール違反があった場合、コンパイルエラーになります。
`forbid`は、`deny`より強いレベルで、上書きができません。

下のコードは、アトリビュートで`missing_docs`ルールをallowに上書きしています。

```rust,ignore
#![allow(missing_docs)]
pub fn foo() {}
```

`missing_dogs`ルールを、denyレベルに設定してコンパイルすると、このコードはコンパイルできます。

```
$ rustc lib.rs --crate-type=lib -D missing-docs
```

一方、forbidレベルに設定してコンパイルすると、コンパイルエラーになります。

```
$ rustc lib.rs --crate-type=lib -F missing-docs
error[E0453]: allow(missing_docs) overruled by outer forbid(missing_docs)
 --> lib.rs:1:10
  |
1 | #![allow(missing_docs)]
  |          ^^^^^^^^^^^^ overruled by previous forbid
  |
  = note: `forbid` lint level was set on command line
```

### lintレベルの設定方法

#### コンパイラフラグで設定

コンパイルオプションで、`-A`, `-W`, `-D`, `-F`のいずれかを指定して、lintレベルを設定できます。

```
$ rustc lib.rs --crate-type=lib -W missing-docs
```

もちろん、複数のフラグを同時に設定することも可能です。

```
$ rustc lib.rs --crate-type=lib -D missing-docs -A unused-variables
```

Cargoの設定ファイル内で、lintレベルを設定することも可能です。

```
$ cat .cargo/config
```

```toml
[build]
rustflags = ["-D", "unsafe-code"]
```

#### アトリビュートで設定

ソースコード内のアトリビュートで、`allow`, `warn`, `deny`, `forbid`のいずれかを指定して、lintレベルを設定できます。

```rust,ignore
$ cat lib.rs
#![warn(missing_docs)]

pub fn foo() {}
```

1つのアトリビュートに、複数のlintルールを指定できます。

```rust,ignore
#![warn(missing_docs, unused_variables)]

fn main() {
pub fn foo() {}
}
```

複数のアトリビュートを組み合わせて使うこともできます。

```rust,ignore
#![warn(missing_docs)]
#![deny(unused_variables)]

pub fn foo() {}
```

### lintルール

次のコマンドでlintルールと、デフォルトレベルの一覧が取得できます。

```
$ rustc -W help
```

デフォルトレベルごとに、サンプルコード付きでlintルールが説明されています。

- [Allowed-by-default lints]
- [Warn-by-default lints]
- [Deny-by-default lints]

[Allowed-by-default lints]: https://doc.rust-lang.org/stable/rustc/lints/listing/allowed-by-default.html
[Warn-by-default lints]: https://doc.rust-lang.org/stable/rustc/lints/listing/warn-by-default.html
[Deny-by-default lints]: https://doc.rust-lang.org/stable/rustc/lints/listing/deny-by-default.html

<!-- ### コラム〜MISRA-C〜 -->

### コラム〜Rustのlintツールclippy〜

さらに細かなlintルールで検査したい場合、[clippy]が使用できます。
clippyは、下記のようなルールを含んでいます。

- 不必要にコードを複雑にする書き方の検出
- 正当性がないコードの検出 (常に条件が真になるなど)
- 性能が低下するコードの検出

導入も容易なため、プロジェクトの初期段階からclippyを導入することをお勧めします。
[clippy lintルール一覧]も合わせてご覧ください。

[clippy]: https://github.com/rust-lang/rust-clippy
[clippy lintルール一覧]: https://rust-lang.github.io/rust-clippy/master/index.html

### コード生成オプション

[Codegen options]にコード生成に関するオプション一覧がまとめられています。

[Codegen options]: https://doc.rust-lang.org/stable/rustc/codegen-options/index.html

組込みで特に重要な最適化オプションについて説明します。
デフォルト (cargo build) では、最適化を行いません。
コンパイラオプションとしては、`-C opt-level = 0`を使用します。

#### 速度最適化

`rustc`は、3つの最適化レベルを提供しています。
`opt-level = 1`, `2`, `3`です。
`cargo build --release`を実行した場合、デフォルトでは、`opt-level = 3`です。

`opt-level = 2`, `3`では、バイナリサイズを犠牲にする (大きくする) ことで、速度を向上します。
例えば、`opt-level = 2`以上では、ループ展開が行われます。
ループ展開は、Flash/ROMの容量をより多く使用します。

組込みでは、速度よりもバイナリサイズが制限になる場合があります。
その場合には、バイナリサイズの最適化が必要です。

#### サイズ最適化

`rustc`は、2つのサイズ最適化レベルを提供しています。
`opt-level = "s"`, `"z"`です。
`"z"`は、`"s"`より小さなバイナリを作ります。

これらの最適化レベルは、LLVMのインライン展開しきい値を下げます。
インライン展開しきい値は、`-C inline-threshold`で指定することもできます。
Rust 1.34.1でのしきい値の使われ方は、[ソースコード]を見るとわかります。

[ソースコード]: https://github.com/rust-lang/rust/blob/1.34.1/src/librustc_codegen_llvm/back/write.rs#L735-L759

- `opt-level = 3`は275
- `opt-level = 2`は225
- `opt-level = "s"`は75
- `opt-level = "s"`は25

#### 出典

- [The rustc book]
- [Embedonomicon 最適化]

[The rustc book]: https://doc.rust-lang.org/stable/rustc/
[Embedonomicon 最適化]: https://tomoyuki-nakabayashi.github.io/book/unsorted/speed-vs-size.html