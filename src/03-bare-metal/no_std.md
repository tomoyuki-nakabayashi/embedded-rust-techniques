## 3-1. no_std

ベアメタルを想定したRustプログラムには、`#![no_std]`アトリビュートが必須です。この`#![no_std]`アトリビュートを指定すると、`std`クレートではなく、`core`クレートをリンクします。

`std`クレートは、Rustの標準ライブラリです。例えば、皆さんが次のようなRustプログラムを書いた場合、`std`クレートが使われています。

```rust,editable
fn main() {
    let vector = vec!(1, 2, 3);
    println!("vector contains {:?}", vector);
}
```

上記プログラムの1行目に、`#![no_std]`を追加した後、`▶`ボタンをクリックしてプログラムを実行してみて下さい。次のようなコンパイルエラーが発生したはずです。

```
error: cannot find macro `println!` in this scope
 --> src/main.rs:5:5
  |
5 |     println!("vector contains {:?}", vector);
  |     ^^^^^^^

error: cannot find macro `vec!` in this scope
 --> src/main.rs:4:18
  |
4 |     let vector = vec!(1, 2, 3);
  |                  ^^^

error: `#[panic_handler]` function required, but not found

error: language item required, but not found: `eh_personality`
```

<<<<<<< HEAD
**println**マクロは、標準出力にフォーマットされた文字列を表示するマクロです。ベアメタル環境では、標準出力なるものは存在しません (OSが提供するものだからです) 。そのため、標準出力を利用する**println**マクロも利用できません。

Rustの**Vec**は、ヒープにメモリ領域を確保します。ベアメタル環境では、ヒープメモリの確保 / 解放の機能が提供されていません。そのため、**Vec**のオブジェクトを作成する**vec**マクロも、**std**クレートをリンクするアプリケーションと同じようには、使えません*。少し手を加えれば、ベアメタル環境でも**Vec**のようなコレクションを使うことが可能です。これは、[メモリアロケータ]で解説します。
=======
`println!`は、標準出力にフォーマットされた文字列を表示するマクロです。ベアメタル環境では、標準出力なるものは存在しません (OSが提供するものだからです) 。そのため、標準出力を利用する`println!`マクロも利用できません。

Rustの`Vec`は、ヒープにメモリ領域を確保します。ベアメタル環境では、ヒープメモリの確保 / 解放の機能が提供されていません。そのため、`Vec`のオブジェクトを作成する`vec!`マクロも、**`std`クレートをリンクするアプリケーションと同じようには、使えません**。少し手を加えれば、ベアメタル環境でも`Vec`のようなコレクションを使うことが可能です。これは、[メモリアロケータ]で解説します。
>>>>>>> parent of 7c7cbc0... fix: Remove code block.

[メモリアロケータ]: allocator.html

さらに、言語仕様上、パニック発生時の動作を定義する必要があります。panicの主な処理は`std`クレートで定義されています。詳しくは、[panic]で説明します。

[panic]: panic.html

`core`クレートは、`std`クレートのサブセットで、環境 (アーキテクチャ、OS) に依存せず使えるコードが含まれています。`core`クレートは、文字列やスライスのような言語プリミティブと、アトミック操作のようなプロセッサ機能を利用するためのAPIを提供しています。

先程コンパイルエラーになったことからわかるように、`core`クレートを使ったベアメタルプログラミングは、`std`を利用したプログラミングとは一味違ったものになります。

#### 出典

- Embedonomicon: [最小限の`#![no_std]`プログラム]

[最小限の`#![no_std]`プログラム]: https://tomoyuki-nakabayashi.github.io/embedonomicon/smallest-no-std.html