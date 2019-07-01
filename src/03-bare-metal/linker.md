## リンカ

ベアメタルプログラミングを行う上で、リンカは避けられない要素です。ここでは、Rustで*シンボル*や*セクション*を扱う方法について、説明します。

### シンボル名とセクション配置

C++と同様に、デフォルトではRustのシンボルは、コンパイラによってマングルされます。コンパイラが生成したシンボル名は、コンパイラのバージョンごとに異なる可能性があります。そこで、次のアトリビュートを使用して、シンボル名やセクション配置を制御します。

- `#[export_name = "foo"]`は、関数や変数のシンボル名を`foo`に設定します。
- `#[no_mangle]`は、関数名や変数名をマングルせず、そのままシンボル名として使用します。
- `#[link_section = ".bar"]`は、対象のシンボルを、`.bar`というセクションに配置します。

`#[no_mangle]`は、基本的に、`#[export_name = <item-name>]`のシンタックスシュガーです。このことから、`#[no_mangle]`と`[#link_section]`との組み合わせで、任意のシンボルを特定のセクションに配置できます。

次のコードは、ARM Cortex-Mシリーズのリセットベクタを指定セクションに配置する例です。`Reset`関数の関数ポインタを、`RESET_VECTOR`というシンボルで、`.vector_table.reset_vector`セクションに配置します。もちろん、このセクションはリンカスクリプトで定義されている必要があります。

```rust,ignore
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
```

> 補足：上のコードで`extern`を使用している理由は、Cortex-MシリーズのハードウェアがリセットハンドラにC ABIを要求するためです。

### リンカスクリプトのシンボルを参照

Rustからリンカスクリプトのシンボルを参照することも可能です。これは、`.bss`セクションのゼロクリアや、`.data`セクションの初期化に利用できます。

次のように、リンカスクリプトでセクションの開始位置と終了位置にシンボルを作成します。

```
{{#include ../../ci/03-bare-metal/linker/link.x:14:15}}
{{#include ../../ci/03-bare-metal/linker/link.x:35:49}}
```

これらのリンカスクリプトで作成したシンボルは、Rustで次のように利用できます。これは、`.bss`セクションのゼロクリアと、`.data`セクションの初期化を行うコードの例です。

```rust,ignore
{{#include ../../ci/03-bare-metal/linker/src/main.rs:5:26}}
```

リンカスクリプトで作成したシンボルを`u8`の変数として、そのアドレスを利用します。

### extern

[extern]は、Rustのキーワードで、*外部*とのインタフェースに使用されます。外部クレートとの接続にも使われますが、組込みでの重要な利用方法はFFI (Foreign function interfaces) です。

[extern]: https://doc.rust-lang.org/std/keyword.extern.html

他言語の変数や関数を利用する場合、下記の通り`extern`ブロック内でインタフェースを宣言します。

```rust,ignore
{{#include ../../ci/03-bare-metal/linker/src/main.rs:10:11}}
        // ...
{{#include ../../ci/03-bare-metal/linker/src/main.rs:17:17}}
```

逆に、他言語からRustのコードを呼ぶ場合は、次のように関数シグネチャを宣言します。

```rust,ignore
{{#include ../../ci/03-bare-metal/linker/src/main.rs:7:8}}
    // ...
{{#include ../../ci/03-bare-metal/linker/src/main.rs:26:26}}
```

FFIだけでなく、どこか外部にあるRustコードを宣言することも可能です。

```rust,ignore
extern "Rust" {
    fn main() -> !;
}
```

### コラム〜RustのABIは安定化していない！？〜

意外に思うかもしれませんが、**RustのABIは定義されていません**。4年前から[このissue]で議論が続けられています。

[このissue]: https://github.com/rust-lang/rfcs/issues/600

そのため、Rustで安定したABIを提供するためには、`extern "C"`を用いてC言語のABIを使用しなければなりません。

```rust,ignore
/// C ABI
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    /* ... */
}
```

### linkageアトリビュート

`linkage`アトリビュートは、まだunstableの状態です。[linkage feature]のissueで議論が続いています。このアトリビュートは、シンボルのリンケージを制御するものです。例えば、特定のシンボルをweakにしてデフォルト実装を与えたり、明示的に外部リンケージにすることができます。

[linkage feature]: https://github.com/rust-lang/rust/issues/29603

#### 出典

- Embedonomicon
  - [メモリレイアウト]
  - [mainインタフェース]

[メモリレイアウト]: https://tomoyuki-nakabayashi.github.io/embedonomicon/memory-layout.html
[mainインタフェース]: https://tomoyuki-nakabayashi.github.io/embedonomicon/main.html