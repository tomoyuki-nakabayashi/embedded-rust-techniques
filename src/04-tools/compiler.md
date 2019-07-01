## コンパイラサポート

Rustコンパイラがサポートしているターゲットについてまとめます。また、ターゲットとするシステムをRustがサポートしていない場合、どのような対応が考えられるか、についても記載します。

### Rustプラットフォームサポート

Rustがサポートしているプラットフォーム一覧は、[Rust Platform Support]に記載されています。

[Rust Platform Support]: https://forge.rust-lang.org/platform-support.html

ここでは、ターゲットシステムを、Tier1からTier3に分類しています。Tier1は「ビルドでき、かつ、動作することが保証されている」ものです。Tier2は「ビルドできることが保証されている」ものです。Tier3は「サポートされているが、ビルドできる保証がない」ものです。

あるアーキテクチャに対して、Rustが提供する最低レベルのサポートは、**有効化されているLLVMバックエンドがある**ことです。次のコマンドにより、Rustコンパイラが使用するLLVMでサポートが有効になっているアーキテクチャを確認できます。

```
$ cargo objdump -- -version
LLVM (http://llvm.org/):
  LLVM version 8.0.0-rust-1.34.1-stable
  // 途中略

  Registered Targets:
    aarch64    - AArch64 (little endian)
    aarch64_be - AArch64 (big endian)
    arm        - ARM
    arm64      - ARM64 (little endian)
    armeb      - ARM (big endian)
    hexagon    - Hexagon
    mips       - MIPS (32-bit big endian)
    mips64     - MIPS (64-bit big endian)
    mips64el   - MIPS (64-bit little endian)
    mipsel     - MIPS (32-bit little endian)
    msp430     - MSP430 [experimental]
    nvptx      - NVIDIA PTX 32-bit
    nvptx64    - NVIDIA PTX 64-bit
    ppc32      - PowerPC 32
    ppc64      - PowerPC 64
    ppc64le    - PowerPC 64 LE
    riscv32    - 32-bit RISC-V
    riscv64    - 64-bit RISC-V
    sparc      - Sparc
    sparcel    - Sparc LE
    sparcv9    - Sparc V9
    systemz    - SystemZ
    thumb      - Thumb
    thumbeb    - Thumb (big endian)
    wasm32     - WebAssembly 32-bit
    wasm64     - WebAssembly 64-bit
    x86        - 32-bit X86: Pentium-Pro and above
    x86-64     - 64-bit X86: EM64T and AMD64
```

もし、手元の環境にLLVMツールの最新版がインストールされている場合、そのターゲットアーキテクチャと見比べて見て下さい。執筆時点での著者の環境では、LLVM 8.0.1がリリースされており、ターゲットアーキテクチャは以下の通りでした。

```
$ llvm-objdump -version
LLVM (http://llvm.org/):
  LLVM version 8.0.1
  // 中略

  Registered Targets:
    aarch64    - AArch64 (little endian)
    aarch64_be - AArch64 (big endian)
    amdgcn     - AMD GCN GPUs
    arm        - ARM
    arm64      - ARM64 (little endian)
    armeb      - ARM (big endian)
    avr        - Atmel AVR Microcontroller
    // 中略
    x86        - 32-bit X86: Pentium-Pro and above
    x86-64     - 64-bit X86: EM64T and AMD64
    xcore      - XCore
```

`amdgcn`、`avr`、`xcore`など、Rustコンパイラではサポートされていないアーキテクチャがあります。Rustコンパイラではこれらのアーキテクチャサポートが無効化されて、配布されています。

### Rustがサポートしていないターゲットのビルド

ここから先は、著者が試したことがないため、参考情報となります。

もし使用したいターゲットが、Rustコンパイラで無効化されている場合 (上述の`amdcgn`や`avr`) 、Rustのソースコードを修正しなければなりません。[rust-lang/rust#52787]の最初の2つのコミットがヒントになります。

[rust-lang/rust#52787]: https://github.com/rust-lang/rust/pull/52787

メインラインのLLVMがターゲットアーキテクチャをサポートしていない場合でも、LLVMのforkが存在しているのであれば、`rustc`のビルド前にLLVMを差し替えることが可能です。[Rust on the ESP and how to get started]では、LLVMのXtensa forkを使用し、ESPをターゲットにRustのコードをコンパイルする方法が紹介されています。

[Rust on the ESP and how to get started]: https://dentrassi.de/2019/06/16/rust-on-the-esp-and-how-to-get-started/

もしGCCでしかターゲットがサポートされていない場合、[mrustc]を使うことができます。これは、非公式のRustコンパイラで、RustプログラムをCコードに変換し、その後、GCCを使ってコンパイルします。

[mrustc]: https://github.com/thepowersgang/mrustc

### target specification

Rustでは、ターゲットシステムに関連する[ターゲット仕様]があります。この仕様では、アーキテクチャ、オペレーティングシステム、データレイアウトなどを記述します。

[ターゲット仕様]: https://github.com/rust-lang/rfcs/blob/master/text/0131-target-specification.md

Rustコンパイラに組み込まれているターゲット仕様があります。コンパイラ組込みのターゲットは、次のコマンドで確認できます。

```
$ rustc --print target-list | column
aarch64-fuchsia                 mips64el-unknown-linux-gnuabi64
aarch64-linux-android           mipsel-unknown-linux-gnu
aarch64-unknown-cloudabi        mipsel-unknown-linux-musl
aarch64-unknown-freebsd         mipsel-unknown-linux-uclibc
aarch64-unknown-linux-gnu       msp430-none-elf
//中略
i686-unknown-openbsd            x86_64-unknown-linux-gnux32
mips-unknown-linux-gnu          x86_64-unknown-linux-musl
mips-unknown-linux-musl         x86_64-unknown-netbsd
mips-unknown-linux-uclibc       x86_64-unknown-openbsd
mips64-unknown-linux-gnuabi64   x86_64-unknown-redox
```

次のコマンドを使って、ターゲット仕様を表示できます (nightlyコンパイラが必要です)。

```
$ rustc +nightly -Z unstable-options --print target-spec-json --target thumbv7m-none-eabi
{
  "abi-blacklist": [
    "stdcall",
    "fastcall",
    "vectorcall",
    "thiscall",
    "win64",
    "sysv64"
  ],
  "arch": "arm",
  "data-layout": "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64",
  "emit-debug-gdb-scripts": false,
  "env": "",
  "executables": true,
  "is-builtin": true,
  "linker": "rust-lld",
  "linker-flavor": "ld.lld",
  "llvm-target": "thumbv7m-none-eabi",
  "max-atomic-width": 32,
  "os": "none",
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-c-int-width": "32",
  "target-endian": "little",
  "target-pointer-width": "32",
  "vendor": ""
}
```

もし、ターゲットとするシステムに対して、コンパイラ組込みのターゲットがない場合、JSON形式でカスタムターゲット仕様を作成します。ターゲットとするシステムに近いコンパイラ組込みターゲットを、上記コマンドで表示し、それをカスタムする方法がお勧めです。本書の執筆時点では、ターゲット仕様の各フィールドが何を意味するか説明する最新のドキュメントがありません。[ターゲット仕様]時点から、追加、変更されているものについては、コンパイラのソースコードを確認する必要があります。

ターゲット仕様ファイルを用意した後は、ファイルパスで指定するか、その名前で参照できます。

```
$ cargo build --target custom.json
# もしくは
$ cargo build --target custom
```

#### 出典

- Embedonomicon: [コンパイラサポートに関する覚書]

[コンパイラサポートに関する覚書]: https://tomoyuki-nakabayashi.github.io/embedonomicon/compiler-support.html