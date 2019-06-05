## ケーススタディ Zephyr binding

Rust Embedded devices WGでもRTOSとRustとのインテグレーションは[issue #62]で議論中です。

[issue #62]: https://github.com/rust-embedded/book/issues/62

ここでは、Cで作られたRTOSであるZephyrをターゲットに、RTOSとのインテグレーションを実験してみます。
ZephyrのAPIを利用して、Rustから`println!`マクロを使って、コンソールに文字を出力します。

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

### 環境

cbindgen, bindgen, Zephyr SDK, QEMUです。

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

### RustからZephyrのAPIを呼び出す

今回はこちらの方が難易度が高いです。
下準備も色々と必要です。