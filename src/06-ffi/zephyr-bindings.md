## ケーススタディ Zephyr binding

Rust Embedded devices WGでもRTOSとRustとのインテグレーションは[issue #62]で議論中です。

[issue #62]: https://github.com/rust-embedded/book/issues/62

ここでは、Cで作られたRTOSであるZephyrをターゲットに、RTOSとのインテグレーションを実験してみます。
ZephyrのAPIを利用して、Rustから`println!`マクロを使って、コンソールに文字を出力します。
また、RTOSのような複雑なCプロジェクトとのインテグレーションが困難な理由を考察します。

そのために、次のことができるようにします。

1. CからRustのAPIを呼び出す
2. RustからZephyrのAPIを呼び出す

双方のバインディングは、`cbindgen`および`bindgen`を用いて自動生成します。
ここで掲載する方法には、まだまだ改善の余地があることに注意して下さい。

次のコードが動くようにします。

```c
{{#include ../../ci/06-ffi/hello/src/main.c:2:3}}
{{#include ../../ci/06-ffi/hello/src/main.c:8:10}}
```

まずC言語から、Rustの`rust_main`関数を呼び出します。
バインディング用のヘッダファイル`rustlib.h`は`cbindgen`で自動生成します。

```rust,ignore
{{#include ../../ci/06-ffi/hello/hello/src/lib.rs:5:8}}
```

Rustには`println!`マクロを実装します。
この`println!`マクロは、ZephyrのAPIを利用してコンソールに文字列を出力します。
Zephyr APIのバインディングは、`bindgen`で自動生成します。

最終的に、C (main) → Rust (rust_main) → C (Zephyr API)というコールグラフになります。

### 環境

これから示すインテグレーション例を試すために必要な環境です。
カッコ内は、著者が試したバージョンです。

- Rust (stable 1.35)
- cbindgen (0.8.3)
- bindgen (0.49.0)
- Zephyr v.1.14
- Zephyr SDK (0.10.0)
- west (v0.5.8)
- qemu-system-arm (2.11.1)

### CからRustのAPIを呼び出す

下のRust関数に対するヘッダファイルを作成します。

```rust,ignore
#[no_mangle]
pub extern "C" fn rust_main() { /* ... */ }
```

ビルドスクリプトでも容易に生成できますが、今回はZephyrとのインテグレーション上、Makefileを使う必要があるため、
Makefile内で次のコマンドを呼び出します。

```
cbindgen src/lib.rs -l c -o lib/rustlib.h
```

これで、次のヘッダファイルが生成されます。

```
cat hello/lib/rustlib.h
```

```c
{{#include ../../ci/06-ffi/hello/hello/lib/rustlib.h:1:6}}
```

このヘッダファイルをCでインクルードします。

```c
{{#include ../../ci/06-ffi/hello/src/main.c:2:3}}
{{#include ../../ci/06-ffi/hello/src/main.c:8:10}}
```

今回は非常に簡潔です。
構造体を引数にしたり、ヒープメモリの管理などリソース管理が加わると、より複雑になります。
しかし、既存のC APIのバインディングを作成するほどは、困難を伴わないことが多いでしょう。

### RustからZephyrのAPIを呼び出す

こちらの方が難易度が高いです。
下準備も色々と必要です。

一般的なライブラリと異なり、多くの組込みOSでは必要最小限の機能だけを組み込んでバイナリを形成します。
ビルド時のコンフィギュレーション次第で、ユーザーアプリケーションが利用できるAPIが増減します。
そのため、全てのコンフィギュレーションで利用可能なバインディングを作ることは、難しいです。
このケーススタディでも、固定のコンフィギュレーションに対してバインディングを作成します。

今回、Zephyrは、デフォルトのZephyr kernel APIに加えて、`newlib`という組込み用標準CライブラリのAPIを有効化します。
この`newlib`を有効化することにより、C標準ライブラリの**一部**がZephyrアプリケーションで利用可能になります。

```
cat prj.config
```

```
# General config
CONFIG_NEWLIB_LIBC=y
```

この設定により、次のようなC標準ライブラリAPIが利用可能になります。

- I/O
  - printf, fwrite, etc
- 文字列
  - strtoul, atoi, etc
- メモリ
  - malloc, free, etc

これらAPIのバインディングを自動生成しつつ、Rustっぽく使えるようにラッピングしていきます。

#### バインディングの自動生成

まず、第一関門です。
ここで難しい点は、2つあります。

1. OSのコンフィギュレーションによって利用できるAPIが異なる
2. 一部APIがZephyrのビルドシステムで自動生成するヘッダに依存している

上記理由から、**一度ターゲットとするZephyrをビルドした後**で、`bindgen`を使用することにしました。
まず、空のアプリケーションを用意して、Zephyrをビルドします。

```
cat src/main.c
```

```c
int main(void) {
	return;
}
```

```
# Zephyrプロジェクトのディレクトリ
source zephyr-env.sh

# ケーススタディプロジェクトのディレクトリ
west build -b cortex_qemu_m3 hello
```

これで、`build/zephyr/include.generated`に必要なヘッダファイルが生成されます。
`Zephyr`の環境変数を利用しながら、`bindgen`でバインディングを生成します。

```
$ bindgen --use-core --ctypes-prefix cty zephyr-sys/headers/bindings.h -o zephyr-sys/src/bindings.rs -- -I${ZEPHYR_BASE}/include -I${ZEPHYR_BASE}/arch/arm/include -I./build/zephyr/include/generated -m32
```

前述の通り、`std`クレートにあるFFI型は利用できないため、`cty`クレートを使います。
`--ctypes-prefix cty`の部分です。

バインディングを生成するためのヘッダファイルは以下の通りです。

```
cat zephyr-sys/headers/bindings.h
```

```c
{{#include ../../ci/06-ffi/hello/zephyr-sys/headers/bindings.h:1:2}}
```

Zephyrのビルドシステムで自動生成されるヘッダ`autoconf.h`内には、アーキテクチャ依存のマクロ定義など、重要な定義が数多く含まれます。
Zephyrのヘッダファイルおよびソースファイルは、この`autoconf.h`に含まれるマクロが定義されていることが前提になっています。
そこで、バインディング作成時にも、まず最初に`autoconf.h`をインクルードしています。

`--`以降 (`-I${ZEPHYR_BASE}`から後) のオプションは、`clang`に与えるオプションです。
`-I`でインクルードパスを、`-m32`でターゲットアーキテクチャが32ビットであることを指示しています。

これで、`bindings.rs`を得ます。
例えば、`printf`のバインディングは、次のように生成されています。

```rust,ignore
extern "C" {
    pub fn printf(fmt: *const cty::c_char, ...) -> cty::c_int;
}
```

この`bindings.rs`をクレートとして利用できるようにします。
次の`lib.rs`を作成します。

```rust,ignore
{{#include ../../ci/06-ffi/hello/zephyr-sys/src/lib.rs:1:7}}
```

Rustとしてはあくまでの`no_std`な環境となるため、`#![no_std]`アトリビュートが必要です。
バインディングを作成したCソースファイルは、Rustの命名規則に沿っていません。
そこで、Rustコンパイラのlintで警告が出ないように、命名規則のlintルールを除外しています。

`lib.rs`と`bindings.rs`を`zephyr-sys`クレートとしてまとめれば、バインディングの生成は完了です。

#### ラッピングクレートの作成

`zephyr-sys`クレートのバインディングは、C APIをそのまま変換しただけなので、Rustらしい安全で使いやすいAPIになっていません。
例えば、`printf`をRustから呼び出そうとすると、次のようなコードになります。

```rust,ignore
    unsafe {
        zephyr::printf(b"Hello from %s\0".as_ptr() as *const cty::c_char,
                       b"Rust\0".as_ptr());
    }
```

お世辞にも使いやすいとは言えません。
その上、文字列をナル文字 (`\0`) で終端し忘れると、未定義動作に突入します。

そこで、`zephyr-sys`をラッピングして、Rustらしく使えるAPIを実装します。
ここが、Cとのインテグレーションで一番大変なところです。

ここでは、Zephyrの`newlib` APIを使って、アプリケーションから安全に利用できる`println!`マクロを作ります。

ほとんど、以前の章で実装した`print`マクロと同じです。
TODO: リンク

#### アプリケーション作成

アプリケーションは、上記で作成した`println!`マクロを呼び出すだけです。

```rust,ignore
{{#include ../../ci/06-ffi/hello/hello/src/lib.rs:3:8}}
```

#### ビルドシステムへのインテグレーション

最後の大仕事です。

### 考察