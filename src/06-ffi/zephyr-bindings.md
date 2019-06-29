## ケーススタディ Zephyr binding

Rust Embedded devices WGでもRTOSとRustとのインテグレーションは[issue #62]で議論中です。

[issue #62]: https://github.com/rust-embedded/book/issues/62

ここでは、Cで作られたRTOSである[Zephyr]をターゲットに、RTOSとのインテグレーションを実験してみます。
ZephyrのAPIを利用して、Rustから`println!`マクロを使って、コンソールに文字を出力します。
また、RTOSのような複雑なCプロジェクトとのインテグレーションが困難な理由を考察します。

[Zephyr]: https://www.zephyrproject.org/

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
ベースは、[print!マクロ]で実装したマクロと同じです。
異なる点は、`fmt::Write`トレイトの`write_str`メソッドの実装で、Zephyrの`fwrite`を呼び出す点です。

[print!マクロ]: ../03-bare-metal/print.md

```rust,ignore
{{#include ../../ci/06-ffi/hello/zephyr/src/io.rs:12:25}}
```

今回の実装では、マルチスレッドで動作した場合のアトミック性については考慮しません。
Zephyrの`fwrite` API自体がアトミック性を保証していないため、Rust側で工夫をしても、システム全体のアトミック性が保証できないためです。

もし、C APIが (例えばMutexなどにより) アトミック性を保証する仕組みを持つ場合、`DebugWriter`を空実装にせず、然るべきロック機能を持たせると良いでしょう。

今回、`fwrite`を用いて`println!`マクロを実装した理由は、ランタイムコストを減らすためです。
別解として、`printf`を用いる実装が考えられますが、文字列のフォーマットはRust処理系で安全性が保証されているため、Cのフォーマットを使う理由がありません。
それどころか、`printf`のフォーマット処理分、余分なランタイムコストがかかります。

Rustの`str`は、バイト数を取得する`len`メソッドを備えているため、`stdout`に対して、指定バイト数書き込む実装にすると、ランタイムコストが少なくなります。

最後になりますが、このクレートを`zephyr`クレートとします。
`Cargo.toml`ファイルに`zephyr-sys`クレートへの依存関係を追加します。

```toml
{{#include ../../ci/06-ffi/hello/zephyr/Cargo.toml:7:9}}
```

#### アプリケーション作成

アプリケーションは、上記で作成した`println!`マクロを呼び出すだけです。

```rust,ignore
{{#include ../../ci/06-ffi/hello/hello/src/lib.rs:3:8}}
```

このクレートを`hello`クレートとして、`zephyr`クレートへの依存を追加します。

```toml
{{#include ../../ci/06-ffi/hello/hello/Cargo.toml:7:9}}
```

このクレートは、staticライブラリとしてビルドし、Zephyrとリンクします。

```toml
{{#include ../../ci/06-ffi/hello/hello/Cargo.toml:11:13}}
```

このクレートは、`Makefile`を使ってビルドします。
これは主にZephyrとのインテグレーション上の理由です。
おおよそ、次のコマンドが実行されるように`Makefile`を作成します。

```
cargo build
cargo objcopy -- --weaken lib/librustlib.a
cbindgen src/lib.rs -l c -o lib/rustlib.h
```

`cargo`でプロジェクトのstaticライブラリを作成し、`objcopy`でシンボルを`weak`にします。
これは、Rustコンパイラビルトインの`memcopy`や`memset`といった関数が、Zephyrのシンボルと衝突してしまうためです。

> 余談ですが、現状、Rustにはビルド後のバイナリやライブラリを操作するための**ポストビルドスクリプト**の仕組みがありません。
> 今回のように、`objcopy`などを使いたい場合には、`Makefile`など外部ビルドシステムに依存しなければなりません。

また、このクレートはCから呼び出されるため、Cのバインディングを生成します (先述の通り)。

#### ビルドシステムへのインテグレーション

最後の仕事です。
Zephyrはビルドシステムに`CMake`を採用しています。
Zephyrのビルドプロセス中で外部ビルドシステムを呼び出し、ライブラリをリンクする方法が確立されています。

詳細な説明は省略しますが、`cmake`の`ExternalProject`を用いて、`CMakeLists.txt`を次の通り記述します。

```cmake
{{#include ../../ci/06-ffi/hello/CMakeLists.txt:22:49}}
```

今回は、先の手順で作成した`Makefile`を呼び出す形にしました。
これでようやく全ての準備が整いました。

#### 動作確認

次のCアプリケーションを作成して、`src/main.c`とします。

```c
#include <stdio.h>

int main(void) {
	rust_main();
}
```

次のコマンドで今回のプロジェクトをビルドし、QEMUで実行できます。

```
mkdir build && cd $_
cmake -GNinja -DBOARD=qemu_cortex_m3 ..
ninja run
```

実行すると無事に`Hello Rust`が表示されます。

```
To exit from QEMU enter: 'CTRL+a, x'[QEMU] CPU: cortex-m3
qemu-system-arm: warning: nic stellaris_enet.0 has no peer
***** Booting Zephyr OS zephyr-v1.14.0 *****
Hello Rust
```

### 考察

この通り、RTOSのような複雑なCプロジェクトとRustとのインテグレーションには、特有の困難さがあります。
まず、ラッピングするAPIの数が膨大です。
今回は、単一機能に対してのみラッピングAPIを作成しましたが、これをRTOSがアプリケーションに提供するAPIの数分こなさなければなりません。

加えて、RTOSではターゲットシステムごとに、アプリケーションが利用できるAPIが増減します。
この要素をどのように統一的に扱うか、を解決する必要があります。
今後のコミュニティの動きに注目しましょう。

## コラム〜マイコン上でRustで書いたWASMアプリケーションが動く！？〜

2019年5月、[WebAssembly Micro Runtime]というマイコン上で動作するWASMランタイムが公開されました。
このWASMランタイムは、Zephyr上で動かすことができます。

[WebAssembly Micro Runtime]: https://github.com/intel/wasm-micro-runtime

それほど苦労せずに、256KBのRAMが搭載されているマイコン上でRustのアプリケーションを実行できました。
アプリケーション実行までの簡単な手順を[WebAssembly Micro RuntimeでRustアプリをマイコンで動かす！]で公開しています。
ランタイムの性能が気になるところですが、WASMが動くようになればターゲットアーキテクチャを気にしなくてもRustアプリケーションが動かせるようになるため、今後もWASM Micro Runtimeに注目しましょう。

[WebAssembly Micro RuntimeでRustアプリをマイコンで動かす！]: https://tomo-wait-for-it-yuki.hatenablog.com/entry/2019/06/16/133344?_ga=2.61433454.350764793.1560857949-1518570932.1554416614

## 参考

[freertos.rs]

[freertos.rs]: https://github.com/hashmismatch/freertos.rs