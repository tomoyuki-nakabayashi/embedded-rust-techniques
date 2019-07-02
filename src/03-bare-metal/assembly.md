## 3-5. アセンブリ

Rustでアセンブリを使う方法は2つあります。*インラインアセンブリ(asm!)* と *自由形式アセンブリ(global_asm!)* です。本当に困ったことに、**両方共stableでは使えません**。

ベアメタルプログラミングをする上で、stableで機能が不足することは往々にしてあることです。ここでは、stableで頑張る方法と、nightlyと共に歩む道、両方を紹介します。

### stableでのアセンブリ

stableでアセンブリを書く方法は、外部ファイルに書くことです。`.s`ファイルにアセンブリを書いておき、アセンブラを使ってオブジェクトファイル (`.o`) にアセンブルし、アーカイブ (`.a`) を作り、Rustのコードとリンクします。

この方法では、ターゲットアーキテクチャのアセンブラが必要です。例えば、ARM Cortex-Mをターゲットにする時、アセンブラとして、`arm-none-eabi-gcc` (`arm-none-eabi-as`) を使います。

`Makefile`を使うこともできますが、よりRustらしく、`ビルドスクリプト`を作成します。[cc]クレートを利用し、ビルドスクリプト内でC言語 (C++やアセンブリも可) のコードをビルドします。

[cc]: https://docs.rs/cc/1.0.36/cc/

今、Cargoプロジェクトのトップディレクトリにアセンブリを書いた`asm.s`があるとします。ビルドスクリプトは、次のようになります。

```rust,ignore
use std::{env, error::Error, fs::File, io::Write, path::PathBuf};
use cc::Build;

fn main() -> Result<(), Box<Error>> {
    // このクレートのビルドディレクトリです
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // ライブラリサーチパスを追加します
    println!("cargo:rustc-link-search={}", out_dir.display());

    // `link.x`をビルドディレクトリに置きます
    File::create(out_dir.join("link.x"))?.write_all(include_bytes!("link.x"))?;

    // `asm.s`ファイルをアセンブルします
    Build::new().file("asm.s").compile("asm");

    Ok(())
}
```

`Cargo.toml`に依存関係を追加します。

```toml
[build-dependencies]
cc = "1.0.36"
```

これだけで、外部アセンブリファイルをアセンブルし、アーカイブファイルを作り、Rustコードとリンクしてくれます。

#### 豆知識〜ccクレートでコンパイラを指定〜

`cc`クレートで任意のコンパイラを使用したい場合、環境変数での設定が可能です。例えば、次のようにコマンドを実行します。

```
CC=/opt/toolchain/arm-none-eabi-gcc cargo build
```

`CFLAGS`環境変数によるコンパイラフラグの指定も可能です。

#### 豆知識〜ビルド生成物の配布〜

クレートと共に、あらかじめビルドした生成物を配布することができます。`cc`クレートを使う場合、ビルドマシンにターゲットアーキテクチャのアセンブラが必要です。クレートのユーザーのマシンに、このアセンブラがなくても、クレートを使ってもらえます。詳しいやり方は、Embedonomiconの[stableでのアセンブリ]を参照して下さい。

### nightlyでのアセンブリ

#### asm!

まずは、インラインアセンブリの`asm!`マクロです。1命令のアセンブリを書く時に便利です。

```rust,ignore
#![feature(asm)]

pub unsafe fn wfi() {
    asm!(wfi :::: "volatile");
}
```

記法の詳細は、[インラインアセンブリ]に記載しています。x86アセンブリは、デフォルトではAT&T記法ですが、オプションによりintel記法で書くことも可能です。

[インラインアセンブリ]: https://doc.rust-jp.rs/the-rust-programming-language-ja/1.9/book/inline-assembly.html

マクロでラッパ関数を生成すると便利です。

```rust,ignore
macro_rules! instruction {
    ($fnname:ident, $asm:expr) => (
        #[inline]
        pub unsafe fn $fnname() {
            match () {
                #[cfg(target_arch = "thumv7m")]
                () => asm!($asm :::: "volatile"),
            }
        }
    )
}

// wfi(), wfe()として利用可能です
instruction!(wfi, "wfi");
instruction!(wfe, "wfe");
// ...
```

#### global_asm!

まとまったアセンブリを書く時に便利です。

```rust,ignore
#![feature(global_asm)]

#[cfg(target_arch = "thumv7m")]
#[link_section = ".text.boot"]
global_asm!(r#"
halt:
    wfe
    b halt
"#);
```

外部アセンブリをインクルードすることもできます。

```
$ cat asm.s
```

```asm
.section ".text.boot"
.global halt
    wfe
    b halt
```

```rust,ignore
#![feature(global_asm)]

global_asm!(include_str!("asm.s"));
```

#### 出典

- Embedonomicon: [stableでのアセンブリ]

[stableでのアセンブリ]: https://tomoyuki-nakabayashi.github.io/embedonomicon/asm.html