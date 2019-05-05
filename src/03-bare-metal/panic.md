## panic

Rustのpanicは、プログラムの異常終了処理を安全に行うための機構です。
例えば、下記のようなスライスの境界外アクセスは、panicを発生させます。

```rust
fn main() {
    let s: &[u8] = &[1, 2, 3, 4];
    println!("{}", s[100]);
}
```

上のプログラムを実行すると、下記のようなpanic発生のエラーが出力されます。

```
thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 100', src/main.rs:3:20
```

C言語の未定義動作と異なり、Rustでは**定義されたpanicハンドラ**でプログラミングエラーに対処します。
OSにホストされている環境では、panicハンドラの処理が完了すると、プロセスを強制終了します。
このプロセスの強制終了も、**定義された動作**です。

Rustのpanicについては、[簡潔なQ Rustのパニック機構]が詳しいです。
こちらの解説にある通り、panicの主な処理は、`std`クレート ([`std::panic`]に公開API、[`std::panicking`]にpanic処理の本体) にあります。
そのため、`#![no_std]`なプログラムでは、panicハンドラが未定義のままになっています。

[簡潔なQ Rustのパニック機構]: https://qnighy.hatenablog.com/entry/2018/02/18/223000
[std::panic]: https://doc.rust-lang.org/std/macro.panic.html
[std::panicking.rs]: https://github.com/rust-lang/rust/blob/stable/src/libstd/panicking.rs

そこで、`#[panic_handler]`アトリビュートを使って、panicハンドラを定義します。
最小限の`#![no_std]`プログラムは、次のようになります。

```rust,ignore
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
```

この`PanicInfo`は、panicに関する情報を提供します。
Rust 1.26からは、`Display`トレイトが実装されているため、フォーマットが使える環境を作ることで、panic発生時の情報を容易に得ることができます。
まず、`std`クレートを使い、簡単に実験できるサンプルコードをお見せします。

```rust
#![allow(unused)]
fn main() {
    use std::panic;

    panic::set_hook(Box::new(|panic_info| {
        println!("{}", panic_info);
    }));

    panic!("Normal panic");
}
```

実行結果は、次のようになります。

```
panicked at 'Normal panic', src/main.rs:9:1
```

このことは、`no_std`環境でも同じように使うことができます。
`no_std`環境での`print!`マクロ実装方法は、[print!マクロ]で紹介します。

[print!マクロ]: print.md

```rust,ignore
use core::panic::PanicInfo;
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
```

`assert!`マクロの失敗でも同様の情報が得られるため、非常に有用なテクニックです。

`no_std`環境で利用可能なpanicハンドラを提供するクレートも存在しています。

- [`panic-abort`]は、パニックが発生すると、アボート命令を実行します。
- [`panic-halt`]は、パニックが発生すると、無限ループに入ります。
- [`panic-itm`]は、ARM Cortex-Mがターゲットの時に利用できるクレートで、パニック発生時のメッセージをITM経由でログを出力します。
- [`panic-semihosting`]は、ARM Cortex-Mがターゲットの時に利用できるクレートで、パニック発生時のメッセージを、セミホスティング機能を使ってログ出力します。

[`panic-abort`]: https://crates.io/crates/panic-abort
[`panic-halt`]: https://crates.io/crates/panic-halt
[`panic-itm`]: https://crates.io/crates/panic-itm
[`panic-semihosting`]: https://crates.io/crates/panic-semihosting

[`panic-abort`の実装]を見ると、30行しかありません。
わざわざクレートにする理由はあるのでしょうか？

[`panic-abort`の実装]: https://github.com/japaric/panic-abort/blob/master/src/lib.rs

panicハンドラをクレートに切り分けることで、コンパイル時のプロファイルでpanicハンドラを切り替える場合に、便利です。

```rust,ignore
// 開発プロファイル：パニックをデバッグしやすくします。`rust_begin_unwind`にブレイクポイントが置けます。
#[cfg(debug_assertions)]
extern crate panic_halt;

// リリースプロファイル：バイナリサイズを最小化します。
#[cfg(not(debug_assertions))]
extern crate panic_abort;
```

上記コードでは、`cargo build`した時は`panic-halt`クレートと、`cargo build --release`した時は`panic-abort`クレートとリンクします。

#### 出典

- The Embedded Rust Book: [2.5.パニック]

[2.5.パニック]: https://tomoyuki-nakabayashi.github.io/book/start/panicking.html
