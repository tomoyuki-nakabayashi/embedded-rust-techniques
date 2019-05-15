## heaplessクレート

通常、[メモリアロケータ]で説明しているように、コレクション利用には、グローバルメモリアロケータの実装が必須です。
[heapless]クレートは、グローバルメモリアロケータがなくても利用できるコレクションです。

[メモリアロケータ]: ../03-bare-metal/allocator.md
[heapless]: https://github.com/japaric/heapless

> `heapless`クレートは、Rust 1.36からstableで利用可能になります。

単純にクレートの依存関係を追加し、コレクションを`use`するだけです。

```rust
extern crate heapless; // v0.4.x

use heapless::Vec;
use heapless::consts::*;

#[entry]
fn main() -> ! {
    let mut xs: Vec<_, U8> = Vec::new();

    xs.push(42).unwrap();
    assert_eq!(xs.pop(), Some(42));
}
```

通常のコレクションと違う点が2つあります。

1つ目は、コレクションの容量を最初に宣言しなければならないことです。
`heapless`コレクションは固定容量のコレクションです。
上の`Vec`は最大で8つの要素を保持することができます。
型シグネチャの`U8`が容量を表しています。
型シグネチャについては、[typenum]を参照して下さい。

[typenum]: https://crates.io/crates/typenum

2つ目は、`push`など多くのメソッドが`Result`を返すことです。
`heapless`コレクションは、固定容量を超える要素の挿入は、失敗します。
APIは、この操作失敗に対処するために、`Result`を返しています。

`heapless`コレクションは、通常、スタック上にコレクションを割り当てます。
また、`static`変数や、ヒープ上に割り当てることも可能です。

v.0.4.4現在、`heapless`は次のコレクションを提供しています。

- BinaryHeap: 優先度キュー
- IndexMap: ハッシュテーブル
- IndexSet: ハッシュセット
- LinearMap: 
- spsc::Queue: single producer single consumer lock-free queue
- String
- Vec
