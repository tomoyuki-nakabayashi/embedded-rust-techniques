## ビルド

### 環境構築

```
rustup target add aarch64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
```

Rustコンパイラでは、ネイティブ用のリンカしか配布していないため、リンカは別途用意します。

```
sudo apt install g++-arm-linux-gnueabihf
sudo apt install g++-aarch64-linux-gnu
```

```
cargo build --target=armv7-unknown-linux-gnueabihf
cargo build --target=aarch64-linux-gnu-gcc
```

細かいことを考えたくなくて、バイナリが大きくなっても良い場合、`musl`を利用すると、ターゲット環境のlibcに依存しないバイナリを生成することができます。

QEMU

```
sudo apt install qemu-user-binfmt
```

ダイナミックリンクしているのでクロスルートディレクトリを明示的に指定します。

```
$ qemu-arm -L /usr/arm-linux-gnueabihf target/armv7-unknown-linux-gnueabihf/debug/raspi
Hello, world!
```

`.cargo/config`にカスタムランナーを設定することで、`cargo run`でQEMU上で実行することができます。

```
cat .cargo/config
```

```toml
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
runner = "qemu-arm -L /usr/arm-linux-gnueabihf"
```

```
cargo run --target armv7-unknown-linux-gnueabihf
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `qemu-arm -L /usr/arm-linux-gnueabihf target/armv7-unknown-linux-gnueabihf/debug/raspi`
Hello, world!
```

## テスト

```
cargo test --no-run
```

```
cargo test --no-run --target=armv7-unknown-linux-gnueabihf
```

```
$ ls target/armv7-unknown-linux-gnueabihf/debug/
build/                    examples/                 native/                   raspi-3f64731a0be9b753.d  
.cargo-lock               .fingerprint/             raspi                     raspi.d                   
deps/                     incremental/              raspi-3f64731a0be9b753
```

`raspi-3f64731a0be9b753`がテストバイナリです。
このバイナリをターゲット上で実行すると、ターゲット上でテストを実行できます。

```
$ qemu-arm -L /usr/arm-linux-gnueabihf target/armv7-unknown-linux-gnueabihf/debug/raspi-3f64731a0be9b753

running 1 test
test ok ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

カスタムランナーを登録しておけば、`cargo test`でQEMU上でテストを実行可能です。

```
cargo test --target armv7-unknown-linux-gnueabihf
   Compiling raspi v0.1.0 (/home/tomoyuki/others/01.rust/embedded-rust-techniques/ci/07-linux/raspi)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running target/armv7-unknown-linux-gnueabihf/debug/deps/raspi-3f64731a0be9b753

running 1 test
test ok ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```