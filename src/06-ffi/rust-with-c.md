## 6-2. CからRustを呼ぶ

ここでは、CのソースコードからRustのソースコードを呼び出す方法を説明します。やることは2つです。

1. Cが扱えるAPIをRust側に作成する
2. 外部ビルドシステムにRustプロジェクトを組み込む

上記の1.については、[cbindgen]で自動生成できます。2.は、Cのプロジェクトやビルドシステムに強く依存するため、一般的な方法はありません。[ケーススタディ Zephyr bindings]では、cmakeプロジェクトに組み込む一例を示します。

[cbindgen]: https://github.com/eqrion/cbindgen
[ケーススタディ Zephyr bindings]: ./zephyr-bindings.html

### ライブラリプロジェクトの作成

通常のRustプロジェクトではなく、システムライブラリを出力します。

```toml
[lib]
crate-type = ["cdylib"]      # 動的ライブラリ
# crate-type = ["staticlib"] # 静的ライブラリ
```

### C APIの作成

C ABIで呼び出しできるRustのAPIを作成します。おおよそ、次のような関数になります。

```rust
#[no_mangle]
pub extern "C" fn rust_function() {
    // ...
}
```

#### no_mangle

Rustコンパイラは、シンボル名をマングルします。そのため、Cから呼び出すRustの関数は、マングルしないように「#[no_mangle]」アトリビュートを付けます。

#### extern "C"

デフォルトでは、Rustの関数はRustのABIを使用します。そこで、CのABIを仕様するように、コンパイラに指示します。プラットフォーム固有のABI指定については、[External Blocks ABI]にドキュメントがあります。

[External Blocks ABI]: https://doc.rust-lang.org/reference/items/external-blocks.html#abi

### Cヘッダファイル作成

Rustで作ったAPIをCから呼べるように、Cヘッダファイルを作成します。

```rust
#[no_mangle]
pub extern "C" fn rust_function() { ... }
```

上のRust APIはCヘッダファイルでは、次のようになります。

```c
void rust_function();
```

### Cヘッダファイルの自動生成

[cbindgen]により、RustソースコードからCヘッダファイルを自動生成することができます。**cbindgen**をベアメタル環境で使うにあたり注意することは、いくつかの標準ライブラリヘッダをインクルードしたヘッダファイルが生成されることです。

```c
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void rust_function(void);
```

ターゲットシステムによっては**stdlib.h**が与えられていない可能性があるので、注意して下さい。

### 外部ビルドシステムにRustプロジェクトを組み込む

Rust APIを作成し、ヘッダファイルを生成すれば、後はCファイルでヘッダをインクルードするだけです。

```c
#include "rust_lib.h"

void call_rust() {
    rust_fuction();
}
```

ビルドシステムへの組み込みについては、Cプロジェクトのビルドシステムが何か、に強く依存します。Makefileでビルドしている場合、ビルドステップの途中でMakefileから**cargo**を呼び出し、静的ライブラリとしてRustコードをビルドし、リンクします。

#### 出典

- The Embedded Rust Book: [9.2.Cと少しのRust]

[9.2.Cと少しのRust]: https://tomoyuki-nakabayashi.github.io/book/interoperability/rust-with-c.html