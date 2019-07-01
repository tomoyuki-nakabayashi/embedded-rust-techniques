## テスト

テストを活用することは、組込みやベアメタルの開発でも非常に重要です。しかし、Rustのテストフレームワークは標準ライブラリに依存しており、`#[no_std]`環境で使うことができません。ここでは、組込み / ベアメタルRustのプロジェクトで利用されているテストやCIについて紹介します。

### デュアルターゲット

部品をcrateに切り出し、ホスト上でテストする方法です。

テスト時には、`#![no_std]`でビルドしないようにします。そうすることで、標準ライブラリに依存するRustのテストフレームワークを利用することができます。

`lib.rs`でクレートレベルのアトリビュートを次のように指定します。

```rust
#![cfg_attr(not(test), no_std)]
```

これで、テスト時には、`#![no_std]`アトリビュートが有効になりません。後は、通常通りテストを書くだけです。[heapless]クレートのテストが、参考になります。

[heapless]: https://github.com/japaric/heapless

### カスタムテストフレームワーク

[Writing an OS in Rust Testing]で紹介されている方法です。unstableな[custom_test_frameworks]フィーチャを利用します。

[custom_test_frameworks]: https://doc.rust-lang.org/unstable-book/language-features/custom-test-frameworks.html

Rust標準のテストフレームワークと比較すると、パニックすることをテストする`should_panic`などの機能が利用できません。

カスタムテストフレームワークを実装するには、次のコードを`main.rs`に追加します。

```rust,ignore
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
```

`test_runner`の引数は`Fn()`トレイトのトレイトオブジェクトのスライスです。[custom_test_frameworks]によると、`#[test_case]`アトリビュートのついたアイテムが、`test_runner`アトリビュートで指定した関数に渡されます。

プロダクトコードのエントリポイントに、テストビルド時のみ、テストハーネスの`test_main`を呼び出すコードを追加します。

```rust
#![reexport_test_harness_main = "test_main"]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}
```

テストケースを書きます。

```rust
#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
```

後は、テストを実行するだけです。

より詳しい情報は、[Writing an OS in Rust Testing]を参照して下さい。

[Writing an OS in Rust Testing]: https://os.phil-opp.com/testing/

### インテグレーションテスト

QEMUを利用して、特定デバイスのペリフェラルに依存しない試験を実施することができます。`RTFM`では、QEMUでバイナリを実行し、semi-hosting機能で標準出力に表示した文字列と期待値とを比較しています。

```
$ cargo run --example binds
   Compiling cortex-m-rtfm v0.5.0-alpha.1
     Running `qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting-config enable=on,target=native -kernel target/thumbv7m-none-eabi/debug/examples/binds`
init
foo called 1 time
idle
foo called 2 times
```

```
$ cat ci/expected/binds.run 
init
foo called 1 time
idle
foo called 2 times
```

```
$ cargo run --example binds | diff -u ci/expected/binds.run -
```

期待通りの動作結果の場合、差分は出力されません。テストの実行結果、差分が出力されるかどうかを検証することで、インテグレーションテストを実施しています。

### コンパイルテスト

複雑な (手続き) マクロを利用するクレートでは、様々な利用方法でコンパイルが通るかどうか、をテストします。

[compiletest_rs]: https://github.com/laumann/compiletest-rs

`RTFM`の`tests`ディレクトリにコンパイルテストのテストケース[compiletest.rs]があります。

[compiletest.rs]: https://github.com/japaric/cortex-m-rtfm/blob/master/tests/compiletest.rs

ここでは、`cfail`ディレクトリにコンパイルが失敗するソースファイルが、`cpass`にコンパイルが成功するソースファイルが置かれています。

```rust
use std::{fs, path::PathBuf, process::Command};

use compiletest_rs::{common::Mode, Config};

#[test]
fn cfail() {
    let mut config = Config::default();

    config.mode = Mode::CompileFail;
    config.src_base = PathBuf::from("tests/cfail");
    config.link_deps();
// 中略
    compiletest_rs::run_tests(&config);
}
```

コンパイルテストの設定`compiletest_rs::Config`を作成し、`compiletest_rs::run_tests`でテストを実行します。これで、`cfail`ディレクトリ内の全てのRustソースファイルのコンパイルに失敗すると、テストがパス、という扱いになります。

一方、コンパイルが成功するテストは、同テストケース内で次のように実装されています。

```rust
use tempdir::TempDir;
// ...
    let td = TempDir::new("rtfm").unwrap();
    for f in fs::read_dir("tests/cpass").unwrap() {
        let f = f.unwrap().path();
        let name = f.file_stem().unwrap().to_str().unwrap();

        assert!(Command::new("rustc")
            .args(s.split_whitespace())
            .arg(f.display().to_string())
            .arg("-o")
            .arg(td.path().join(name).display().to_string())
            .arg("-C")
            .arg("linker=true")
            .status()
            .unwrap()
            .success());
    }
```

システムコマンドで`rustc`を呼び出して、終了ステータスが`success`かどうか、をテストしています。