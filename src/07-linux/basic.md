## 7-1. ビルド

32ビットの**ARMv7**と、64ビットの**AArch64**とでビルドできる手順をそれぞれ示します。Raspbianを使用している場合は、32ビットの**ARMv7**をターゲットにして下さい。

**Ubuntu 18.04**で動作するコマンドを掲載しています。他のOSをご利用の方は、お手数ですが読み替えをお願いします。

### 環境構築

まずクロスコンパイルのターゲットをインストールします。

```
# ARMv7
rustup target add armv7-unknown-linux-gnueabihf
# AArch64
rustup target add aarch64-unknown-linux-gnu
```

Rustコンパイラでは、ネイティブ用のリンカしか配布していないため、リンカは別途用意します。Yoctoなどでツールチェインを構築している場合、そのツールチェインを利用できます。

```
# ARMv7
sudo apt install g++-arm-linux-gnueabihf
# AArch64
sudo apt install g++-aarch64-linux-gnu
```

「.cargo/config」でリンカを指定します。

```toml
# ARMv7
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"

# AArch64
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

プロジェクトをビルドする際は、次の通り、ターゲットを指定します。

```
# ARMv7
cargo build --target=armv7-unknown-linux-gnueabihf
# AArch64
cargo build --target=aarch64-linux-gnu-gcc
```

生成されたバイナリ (**target/armv7-unknown-linux-gnueabihf/debug/**または**target/aarch64-unknown-linux-gnu/debug/**にあります) をRaspberry Pi3にコピーするだけで、実行できます。ターゲットシステム上のライブラリに依存する場合は、ライブラリパスなどを別途、指定する必要があります。

QEMUのユーザーモードエミュレーションを使って、動作確認してみましょう。

```
sudo apt install qemu-user-binfmt
```

ダイナミックリンクしているのでクロスルートディレクトリを明示的に指定します。

```
# ARMv7
$ qemu-arm -L /usr/arm-linux-gnueabihf target/armv7-unknown-linux-gnueabihf/debug/raspi
Hello, world!
# AArch64
qemu-aarch64 -L /usr/aarch64-linux-gnu/ target/aarch64-unknown-linux-gnu/debug/raspi
Hello, world!
```

「.cargo/config」にカスタムランナーを設定することで、**cargo run**でQEMU上やRaspberry Pi3上で実行することができます。まず、QEMU上で実行する設定です。

```
cat .cargo/config
```

```toml
# ARMv7
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
runner = "qemu-arm -L /usr/arm-linux-gnueabihf"

# AArch64
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
runner = "qemu-aarch64 -L /usr/aarch64-linux-gnu"
```

```
# ARMv7
cargo run --target armv7-unknown-linux-gnueabihf
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `qemu-arm -L /usr/arm-linux-gnueabihf target/armv7-unknown-linux-gnueabihf/debug/raspi`
Hello, world!

# AArch64
cargo run --target aarch64-unknown-linux-gnu
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `qemu-aarch64 -L /usr/aarch64-linux-gnu target/aarch64-unknown-linux-gnu/debug/raspi`
Hello, world!
```

バイナリサイズを気にしない場合、**musl**のターゲットを利用すると、ターゲット環境のlibcに依存しないバイナリを生成することができます。その場合、**armv7-unknown-linux-musleabihf**もしくは**aarch64-unknown-linux-musl**を指定します。**Hello World**プログラムで、**armv7-unknown-linux-gnueabihf**は約1.5 MB、**armv7-unknown-linux-musleabihf**は約1.8 MBになります。

続いて、カスタムランナーを設定して、リモートのRaspberry Pi3でバイナリを実行する例を示します。以降では、パスワード認証方式でsshすることを想定しています。Raspberry Pi3にsshするための設定は、事前に済ませて下さい。公開鍵認証方式を使用してもかまいませんし、開発期間の間はパスワードなしでssh可能にしても良いです。

shell script内でパスワードを入力するため**expect**をインストールします。

```
sudo apt install expect
```

次のシェルスクリプトを用意します。

```
$ cat run.sh
```

```sh
#!/bin/sh

PW="raspberry"

expect -c "
set timeout 5
spawn env LANG=C /usr/bin/scp $1 pi@<IPアドレス>:/home/pi/raspi
expect \"password:\"
send \"${PW}\n\"
expect \"$\"

spawn env LANG=C /usr/bin/ssh pi@<IPアドレス> ./raspi
expect \"password:\"
send \"${PW}\n\"
expect \"$\"
exit 0
"
```

shell script実行時の第一引数を、**/home/pi/**に**raspi**としてコピーします。次に、Raspberry Pi3上の**raspi**バイナリを実行します。

これをプロジェクトのルートディレクトリ (Cargo.tomlのあるディレクトリ) に置いて、カスタムランナーに指定します。

```toml
# ARMv7
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
runner = "sh run.sh"
```

Raspberry Pi3にssh可能な状態で、**cargo run**を実行します。

```
cargo run --target armv7-unknown-linux-gnueabihf
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `sh run.sh target/armv7-unknown-linux-gnueabihf/debug/raspi`
spawn env LANG=C /usr/bin/scp target/armv7-unknown-linux-gnueabihf/debug/raspi pi@<IPアドレス>:/home/pi/raspi
pi@<IPアドレス>'s password: 
raspi                                         100% 1481KB   1.3MB/s   00:01    
spawn env LANG=C /usr/bin/ssh pi@<IPアドレス> ./raspi
pi@<IPアドレス>'s password: 
Hello, world!
```

## テスト

上述のカスタムランナーを登録しておけば、**cargo test**でQEMU上やRaspberry Pi3上でテストを実行可能です。

QEMU上での実行例です。

```
cargo test --target armv7-unknown-linux-gnueabihf
   Compiling raspi v0.1.0 (embedded-rust-techniques/ci/07-linux/raspi)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running target/armv7-unknown-linux-gnueabihf/debug/deps/raspi-3f64731a0be9b753

running 1 test
test ok ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Raspberry Pi3上での実行例です。

```
cargo test --target armv7-unknown-linux-gnueabihf
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running target/armv7-unknown-linux-gnueabihf/debug/deps/raspi-2fd52b9957ada715
spawn env LANG=C /usr/bin/scp embedded-rust-techniques/ci/07-linux/raspi/target/armv7-unknown-linux-gnueabihf/debug/deps/raspi-2fd52b9957ada715 pi@<IPアドレス>:/home/pi/raspi
pi@<IPアドレス>'s password: 
raspi-2fd52b9957ada715                        100% 2820KB 998.2KB/s   00:02    
spawn env LANG=C /usr/bin/ssh pi@<IPアドレス> ./raspi
pi@<IPアドレス>'s password: 

running 1 test
test ok ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```