## print!マクロ

ベアメタル環境でデバッグする上で、自在にテキストを表示できることは、非常に重要です。
Rustでは、`print!`や`println!`マクロを使うことで、数値や文字列、構造体までフォーマットして、テキストで表示することができます。

```rust
fn main() {
    println!("{}, {:?}", 1, vec!(1, 2, 3));
}
```

マイクロコントローラでは、文字出力はUARTで行うことが多いです。
例えば、今、次のような関数を使って、1文字のASCII文字を表示できるとします。
UARTペリフェラルの初期化がが必要な点や、TXバッファに空きがあるかどうか調べなければならない点は、一旦目を瞑って下さい。

``` rust,ignore
{{#include ../../ci/03-bare-metal/print_raw_chars/src/main.rs:16:16}}
{{#include ../../ci/03-bare-metal/print_raw_chars/src/main.rs:19:22}}
```

この状態で数値や文字列、構造体をテキストで表示しようとすると、まず文字列に変換しなければなりません。
これを、自前で実装するのは、容易ではありません。
読者の中には、C言語で`printf()`関数を (部分的に) 自作した経験がある方が、多数いらっしゃるかと思います。
あれはあれで貴重な経験ではありますが、Rustではより簡単に、**型安全**なテキスト表示マクロを実装できます。

では、上記関数を使って、`std`環境と同じ`print!` / `println!`マクロを使えるようにしましょう。

まず、全貌をお見せします。

```rust,ignore
{{#include ../../ci/03-bare-metal/print/src/main.rs:22:49}}
```

これで全てです。この30行にも満たないコードを追加するだけで、`std`環境と同じように`println!`が使えます。
順番に解説していきます。

まず、`print!`マクロの実装です。

```rust,ignore
{{#include ../../ci/03-bare-metal/print/src/main.rs:24:27}}
```

最も重要な部分は、`format_args!`マクロの呼び出しです。
`format_args!`マクロは、コンパイラ組込みの手続きマクロで、文字列フォーマットの中心を担うAPIです。
このマクロは、与えられたフォーマット文字列と引数群から、`core::fmt::Arguments`を構築するコードを生成します。
この辺りの話については、[Rustの文字列フォーマット回り (改訂版)]で非常に詳しく解説されています。ここでは詳細を割愛します。

[Rustの文字列フォーマット回り (改訂版)]: https://ubnt-intrepid.github.io/blog/2017/10/11/rust-format-args/

`$crate::_print()`は、`format_args!`マクロの出力である`core::fmt::Arguments`を引数に取るラッパー関数です。

```rust,ignore
{{#include ../../ci/03-bare-metal/print/src/main.rs:35:38}}
```

フォーマット文字列をUARTに出力する`UartWriter`構造体のオブジェクトを作成し、`core::fmt::Write`トレイトの`write_fmt`メソッドを呼び出します。
`UartWriter`は、ここではかなり実装を簡略化しており、中身のない空の構造体です。
ハードウェアの排他制御などは、ここでは考えません。

```rust,ignore
{{#include ../../ci/03-bare-metal/print/src/main.rs:40:49}}
```

フォーマット文字列を取り扱うために、`UartWriter`は`core::fmt::Write`トレイトを実装します。
`write_fmt`メソッドは、デフォルトメソッドなので、`write_str`だけ実装すれば良いです。
`write_str`メソッドの関数シグネチャは、`fn (&mut self, &str) -> fmt::Result`となっており、
`&str`の形で渡されるフォーマット済み文字列をどのように出力するか、を実装します。
上記コードでは、イテレータで1バイトずつ取得し、`write_byte`関数でUARTに1バイトずつ送信します。

それでは、実行してみましょう。
`03-bare-metal/print`ディレクトリに、QEMUで動作するサンプルがあります。
リセットベクタ内で、`println!`マクロを呼び出します。

```rust,ignore
{{#include ../../ci/03-bare-metal/print/src/main.rs:6:8}}
{{#include ../../ci/03-bare-metal/print/src/main.rs:11:12}}
```

次のコマンドで実行できます (`thumbv7m-none-eabi`のクロスコンパイラとqemu-system-armが必要です) 。

```
$ cargo run
```

```
     Running `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv7m-none-eabi/debug/print`
Hello Rust
```

> 注意：このサンプルはQEMUでしか動作しません。QEMUのUARTは初期設定不要で雑に使えるため、
> 非常に便利です。

また、[panic]で紹介した通り、panic時の情報を表示する際も便利です。

```rust,ignore
{{#include ../../ci/03-bare-metal/print/src/main.rs:7:7}}
{{#include ../../ci/03-bare-metal/print/src/main.rs:10:10}}
{{#include ../../ci/03-bare-metal/print/src/main.rs:12:13}}
{{#include ../../ci/03-bare-metal/print/src/main.rs:56:60}}
```

実行すると、panicを発生させたソースコードの位置と、メッセージを表示します。

```
panicked at 'explicit panic!', src/main.rs:10:5
```