## RustからCを呼ぶ

ここでは、RustのソースコードからC言語のソースコードを呼び出す方法を説明します。
やることは2つです。

1. CのAPIを、Rustで使えるようにインタフェースを定義する
2. Cのコードを、Rustのコードを一緒にビルドする

上記の1.については、[bindgen]で自動生成できます。
2.についは、基本的に、Rustの`build.rs`スクリプトで対応します。

[bindgen]: https://github.com/rust-lang/rust-bindgen

### インタフェース定義

まず、手動でインタフェースを定義する例を示します。
標準ライブラリが使える環境と異なる点は、`cty`クレートを使う点です。

今、次のようなヘッダファイルが公開されているとします。

```c
/* target.h */
typedef struct MyStruct {
    int32_t x;
    int32_t y;
} MyStruct;

void my_function(int32_t i, MyStruct* ms);
```

このヘッダファイルをRustに変換すると、インタフェースは次のようになります。

```rust,ignore
/* bindings.rs */
#[repr(C)]
pub struct MyStruct {
    pub x: cty::int32_t,
    pub y: cty::int32_t,
}

pub extern "C" fn my_function(
    i: cty::int32_t,
    ms: *mut MyStruct
);
```

`#[repr(C)]`アトリビュートにより、Rustコンパイラは、構造体のデータをCと同じルールで構成します。
デフォルトでは、Rustコンパイラは、`struct`内のデータ順やサイズを保証しません。

```rust,ignore
pub extern "C" fn my_function(...);
```

このコードは、`my_function`という名前の、C ABIを使った関数を宣言します。
関数の定義は、別の場所で与えるか、静的ライブラリから最終バイナリにリンクする必要があります。

### インタフェースの自動生成

上述の通り、手動でインタフェースを作成することもできますが、これは単純作業であり、ミスが発生する可能性があります。
そこで、インタフェースを自動生成してくれる`bindgen`を利用します。
ここでは、一般的な手順を説明し、具体的な例は、[ケーススタディ Zephyr bindings]で紹介します。

[ケーススタディ Zephyr bindings]: ./zephyr-bindings.md

標準ライブラリが使える環境と、ベアメタル環境とで共通する手順は、次の通りです。

1. Rustで使いたいインタフェースやデータ型を定義している全てのCヘッダを集める
2. ステップ1で集めたヘッダファイルを`#include "..."`する`binding.h`ファイルを書く
3. `bindgen`に`binding.h`を与えて実行する
4. `bindgen`の出力を、`bindings.rs`にリダイレクトする
5. `bindings.rs`を`include!`する`lib.rs`を用意し、クレートとして利用できるようにする
6. ステップ5で作成したクレートをRustらしく使えるAPIに変換するラッパクレートを作成する

ステップ5で作成する自動生成されたコードのクレートは、`-sys`という名前にすることが慣例になっています。

ベアメタル環境でバインディングを作る場合、標準ライブラリが使える環境と異なる点は、次の3点です。

1. `bindgen`利用時に、コマンドラインなら`--ctypes-prefix`に`cty`オプションを使う、または、ビルドスクリプトなら`Builder.ctypes_prefix("cty")`を使う
2. bindingクレートに`cty`クレートとの依存関係を追加する
3. bindingクレートに`#![no_std]`アトリビュートを追加する

`#[no_std]`環境をターゲットに`bindgen`をコマンドラインから利用する場合、例えば次のようなオプションを指定します。

```
bindgen --use-core --ctypes-prefix cty ...
```

`--use-core`は、標準ライブラリの型を使わずに`core`クレートの型を使うコードを生成します。
`--ctypes-prefix cty`は、Cの型プレフィックスを`cty`クレートの物が使われるようにします。

`bindgen`で生成したRustコードを、クレートにまとめます。
その際、`cty`クレートの依存を`Cargo.toml`に追加します。

```toml
[dependencies]
cty = "0.2.0"
```

ほぼ定型作業ですが、bindgenで自動生成したコードは`bindings.rs`としておき、`lib.rs`からインクルードします。
`lib.rs`には、次の内容を書いておきます。

```rust
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

Cのソースコードは、Rust推奨のコーディングスタイルに沿っていないため、関連する警告が出ないようにします。
加えて、`#[no_std]`アトリビュートを追加します。
