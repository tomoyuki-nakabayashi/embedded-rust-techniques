## テスト

テストを活用することは、組込みやベアメタルの開発でも非常に重要です。
ここでは、組込み / ベアメタルRustのプロジェクトで利用されているテストやCIについて紹介します。

### Dual target

部品をcrateに切り出し、ホスト上でテストする方法です。

テスト時には、`#![no_std]`でビルドしないようにします。

### custom test framework

Writing an OS in Rustで紹介されている方法です。
QEMUで動作します。

### uTest

2年ほど更新が停止しています。

### インテグレーションテスト

`RTFM`では、QEMUでバイナリを実行し、semi-hosting機能で標準出力に表示した文字列と期待値とを比較しています。

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

差分は出力されません。
わざと差分のあるファイルと比較してみます。

```
$ cat ci/expected/idle.run 
init
idle
```

```
$ cargo run --example binds | diff -u ci/expected/idle.run -
@@ -1,2 +1,4 @@
 init
+foo called 1 time
 idle
+foo called 2 times
```

### コンパイルテスト

[compiletest_rs]

[compiletest_rs]: https://github.com/laumann/compiletest-rs

### lint