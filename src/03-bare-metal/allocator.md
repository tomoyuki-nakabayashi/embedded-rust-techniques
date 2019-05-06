## メモリアロケータ

ベアメタルプログラミングでも開発が進むと動的なコレクションが使いたくなります。
`std`が使える通常のRustでは、`Vec`や`String`といった一般的なコレクションが利用できます。
このようなコレクションは、ヒープメモリを利用します。
そのため、デフォルトでは、`no_std`な環境では、これらのコレクションを利用できません。
しかし、`no_std`なRustでも、メモリアロケータを実装することで、コレクションを利用することができます。

メモリアロケータを実装**せず**にコレクションを利用する方法は、[`heapless`]で説明します。

[`heapless`]: ../05-library/heapless.md

ただ、(執筆時点のRust 1.34.1では) 残念なことに**nightly必須です**。
ベアメタルでメモリアロケータを実装するには、[`alloc`]と[`alloc_error_handler`]のフィーチャが必要です。
`alloc`は、Rust 1.36でstableになるため、本書が世に出回っている時点では、stableになっています。
一方、`alloc_error_handler`については、まだ安定化の目途が立っていないようです。
今しばらく、メモリアロケータの実装はnightly専用になりそうです。

[`alloc`]: https://doc.rust-lang.org/alloc/index.html
[`alloc_error_handler`]: https://github.com/rust-lang/rust/issues/51540

一時的にツールチェインをnightlyに切り替えます。
nightlyのツールチェインにCortex-M3用のターゲットを追加します。

```
$ rustup override set nightly
$ rustup target add thumbv7m-none-eabi
```

ここでの目標は、次のプログラムを動作させることです。

```rust,ignore
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:15:15}}
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:18:21}}
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:24:24}}
```

`println!`マクロの実装方法については、[`print!マクロ`]で説明しています。

[`print!マクロ`]: print.md

### グローバルアロケータ

`Vec`や`String`といったコレクションは、デフォルトでは**グローバルアロケータ**を使ってヒープメモリ領域を確保します。
グローバルアロケータとは、`#[global_allocator]`アトリビュートが指定されたアロケータのことです。
このアトリビュートで指定するオブジェクトは、[`GlobalAlloc`]トレイトを実装しなければなりません。

[`GlobalAlloc`]: https://doc.rust-lang.org/1.29.2/core/alloc/trait.GlobalAlloc.html

```rust,ignore
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:55:62}}
```

それでは、グローバルアロケータに指定する`BumpPointerAlloc`の実装を見てみましょう。

### BumpPointerAlloc

これから、`BumpPointerAlloc`という最も単純なアロケータを実装します。
このアロケータは、次のようにヒープメモリを管理します。

- 初期化時に、ヒープメモリ領域の開始アドレスと終了アドレスを受け取ります
- 割り当て可能なメモリ領域の先頭ポインタを1つだけ保持します
- メモリを新しく割り当てると、割り当てた分だけ単純に先頭ポインタを増加します
- 一度割り当てたメモリは、解放しません

上述した通り、このアロケータは、`GlobalAlloc`トレイトを実装します。
全体を示します。

```rust,ignore
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:6:8}}
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:10:12}}
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:26:53}}
```

順番に解説します。

```rust,ignore
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:27:30}}
```

まず、このアロケータは、割り当て可能なメモリ領域の先頭を示す`head`と、末尾を示す`end`を持ちます。
`head`が`UnsafeCell`になっている理由は、`&self`を引数に取る`alloc`メソッドの中で`head`の値を書き換えるためです。

```rust,ignore
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:32:32}}
```

次に`Sync`トレイトを実装します。
これは、グローバルアロケータのオブジェクトが`static`変数になるため、スレッド間で安全に共有できることをコンパイラに伝えるためです。

`GlobalAlloc`トレイトの実装で求められるメソッドは、`alloc`と`dealloc`のみです。
`dealloc`はメモリを解放しないため、何もしません。

```rust,ignore
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:35:48}}
```

引数`layout` ([`Layout`]) は、要求されているメモリブロックです。
`align()`で、アライメントを考慮して、確保しなければならないメモリブロックサイズを返します。
`head`のアドレスが`end`に到達するまで、単純にポインタを増加しながら、メモリを割り当てます。

[`Layout`]: https://doc.rust-lang.org/core/alloc/struct.Layout.html

なお、この実装は、割り込みでメモリアロケータを使用する場合、データ競合が発生し、安全に利用**できません**。

### alloc_error_handler

最後の要素が、アロケーションエラー発生時のハンドラです。
これは、`#[alloc_error_handler]`アトリビュートを指定します。

```rust,ignore
{{#include ../../ci/03-bare-metal/allocator/src/main.rs:64:67}}
```

### 動作確認

`03-bare-metal/allocator`ディレクトリに、Cortex-M3をターゲットにした場合のサンプルコードがあります。
ディレクトリに移動し、次のコマンドで実行結果が確認できます。

```
$ cargo run
```

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv7m-none-eabi/debug/allocator`
[42, 83]
```

無事、`Vec`のデバッグ表示が確認できました。

### メモリアロケータ実装例

ここで紹介した`BumpPointerAlloc`は実用に耐えないものです。
いくつか、より洗練されたメモリアロケータの実装例を紹介します。

#### [linked-list-allocator]

BlogOSの著者が公開しているlinked-listを使ったアロケータです。
Writing an OS in Rust (First Edition) [Kernel Heap]に少し解説があります。

[linked-list-allocator]: https://github.com/phil-opp/linked-list-allocator
[Kernel Heap]: https://os.phil-opp.com/kernel-heap/

#### [Redox Slab allocator]

RustでOSを作るプロジェクト「Redox」のメモリアロケータです。
僭越ながら、簡単な解説を[Redox Slab Allocatorで学ぶRustベアメタル環境のヒープアロケータ]に書いています。

[Redox Slab allocator]: https://gitlab.redox-os.org/redox-os/slab_allocator
[Redox Slab Allocatorで学ぶRustベアメタル環境のヒープアロケータ]: https://qiita.com/tomoyuki-nakabayashi/items/e0bd16e9105163cecafb

#### [alloc-cortex-m]

linked-list-allocatorを、Cortex-MのMutexを使ってラッピングしたメモリアロケータです。

[alloc-cortex-m]: https://github.com/rust-embedded/alloc-cortex-m/

#### [kernel-roulette]

RustでLinux kernelのdriverを書くプロジェクトです。
このプロジェクトでは、`kmalloc`や`kfree`をFFIで呼び出し、Linux kernelの機能を用いてRustのメモリアロケータを実装します。

[kernel-roulette]: https://github.com/souvik1997/kernel-roulette

#### 出典

- The Embedded Rust Book: [コレクション]

[コレクション]: https://tomoyuki-nakabayashi.github.io/book/collections/index.html