## entryポイント

ベアメタルプログラミングの開始地点では、シンボルを駆使したプログラミンを行います。
このことは時として、Rustの安全性に頼らず、開発者が安全性を保証しなければならないことを意味します。

ベアメタルプログラムの最初のプロセスは、**無限ループを実行し、プロセスが決して停止しないように実装します**。
ここでは、Rustの型検査を用いて、ベアメタルプログラミングでのエントリーポイントを再利用性が高く、安全にする方法について説明します。

### 再利用できるリセットハンドラ

一度作って終わり、であればリセットハンドラを再利用可能とすることに、あまり意味はありません。
私個人としては、自身の作るプログラムをより良い設計のものにしたり、他人に使ってもらいたいです。
また、将来的に、自分が新しくプログラムを作ったり、既存のものを作り直す場合に、ソフトウェアの再利用性が高いことが重要です。

そこで、まず、リセットハンドラをアプリケーション (2nd stageブートローダかもしれませんし、OSかもしれません) から独立させます。

```rust,ignore
#![no_std]

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "Rust" {
        fn main() -> !;
    }

    main()
}
```

上のプログラムでは、リセットハンドラは、「外部にあるRust ABIの`main`というシンボルがついた関数」を呼び出します。
このリセットハンドラをクレートとして切り出せば、リセットハンドラとアプリケーションとを、異なるクレートで管理できます。
リセットハンドラのクレートを`reset`クレートとします。
アプリケーションクレートでは、次のように`main`関数を定義します。

```rust,ignore
#![no_std]
#![no_main]

extern crate reset;

#[no_mangle]
pub fn main() -> ! {
    let _x = 42;

    loop {}
}
```

ただし、このシンボルをインタフェースとしてやり方は、**安全ではありません**。
アプリケーション側の`main`関数を`!`なしで定義しても、コンパイルが通ってしまい、未定義動作を引き起こす可能性があります。

### 型安全にする

クレートの利用者や、将来の自分 (!) が誤った使い方をできないようにしましょう。
シンボルの代わりに、マクロをインタフェースとして利用します。

```rust,ignore
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // 与えられたパスの型チェック
            let f: fn() -> ! = $path;

            f()
        }
    }
}
```

アプリケーションは、次のようにこのマクロを利用します。

```rust,ignore
#![no_std]
#![no_main]

use rt::entry;

entry!(main);

fn main() -> ! {
    let _x = 42;

    loop {}
}
```

マクロ内で型チェックを行うため、`main`の戻り値が`!`でなければ、コンパイルエラーになります。

### 補足：発散する関数

Rustの関数には、発散する関数 (diverging functions) という種別があります。
これは、[The Book 1st edition: Functions#diverging-functions]に説明があります。

[The Book 1st edition: Functions#diverging-functions]: https://doc.rust-lang.org/1.30.0/book/first-edition/functions.html#diverging-functions

戻り値に`!`の型を持つ関数は、決してその関数から戻らないことを意味します。
次のプログラムをビルド (実行) してみて下さい。

```rust
fn main() -> ! {
    println!("I'll be back!");
}
```

次のようなエラーが発生したはずです。

```
error[E0308]: mismatched types
 --> src/main.rs:1:14
  |
1 | fn main() -> ! {
  |    ----      ^ expected !, found ()
  |    |
  |    this function's body doesn't return
  |
  = note: expected type `!`
             found type `()`
```

下記エラーが示す通り、`!`を戻り値として持つ関数が、関数から戻るようなコードになっている場合には、コンパイルエラーになります。

```
this function's body doesn't return
```

次に、無限ループを挿入して、再び実行してみます。

```rust
fn main() -> ! {
    println!("I never return!!");
    
    loop {}
}
```

Rust Playgroundでは、タイムアウトでプロセスが強制終了されますが、コンパイルが通ることがわかります。

#### 出典

- Embedonomicon: [mainインタフェース]
 
[mainインタフェース]: https://tomoyuki-nakabayashi.github.io/embedonomicon/main.html