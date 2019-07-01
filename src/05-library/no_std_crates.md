## no_stdクレート

no_stdや組込みで利用可能なクレートを紹介します。Rustのクレートが登録されている`crates.io`には、[crates.io No standard library]カテゴリがあります。2019/5/16現在、840ものクレートが登録されています。

[crates.io No standard library]: https://crates.io/categories/no-std

組込みに関連するものは、[awesome-embedded-rust]に主要なものがまとめられています。

[awesome-embedded-rust]: https://github.com/rust-embedded/awesome-embedded-rust

### rt (runtime) クレート

rtクレートは、ターゲットアーキテクチャ用の最小限のスタートアップ / ランタイムを提供するクレートです。[cortex-m-rt]、[msp430-rt]、[riscv-rt]の3つのターゲットアーキテクチャに対して実装が存在しています。

[cortex-m-rt]: https://github.com/rust-embedded/cortex-m-rt
[msp430-rt]: https://github.com/rust-embedded/msp430-rt
[riscv-rt]: https://github.com/rust-embedded/riscv-rt

これらのクレートは、以下の機能を提供します。

- `.bss`と`.data`セクションの初期化
- FPUの初期化
- プログラムのエントリポイントを指定するための`#[entry]`アトリビュート
- `static`変数が初期化される前に呼ばれるコードを指定するための`#[pre_init]`アトリビュート
- 一般的なターゲットアーキテクチャ用のリンカスクリプト
- ヒープ領域の開始アドレスを表す`_sheap`シンボル

このクレートを使用することで、次のようにアプリケーションのmainコードからプログラムを記述することができます。

```rust,ignore
#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

// `main`をこのアプリケーションのエントリポイントであるかのように利用できます。
// `main`は戻れません
#[entry]
fn main() -> ! {
    // ここに処理を書きます
    loop { }
}
```

[Embedonomicon]は、このような`rt`クレートの実装方法を解説しています。

[Embedonomicon]: https://tomoyuki-nakabayashi.github.io/embedonomicon/

### [embedded HAL]

組込みRustで共通して利用できる`trait`を定義しているクレートです。例えば、SPIやシリアル、といったトレイトが定義されています。

[embedded HAL]: https://github.com/rust-embedded/embedded-hal

このクレートの抽象を利用して、デバイスドライバを書くことで、アプリケーションの再利用性が向上します。組込みRustの多くのプロジェクトが、このembedded HALを利用しています。

### その他

#### [lazy_static]

[lazy_static]は、実行時にしか初期化できない (`new()`関数でのみオブジェクトが構築できる) ような、複雑なオブジェクトの`static`変数を作るために使います。通常、`new()`関数でオブジェクトを作るような構造体は、コンパイル時に値が計算できないため、`static`変数の初期化には使えません。また、`lazy_static`は、`static`変数を1度だけ初期化する機能も提供します。`lazy_static`マクロで作られた`static`変数は、その変数が実行時に最初に使用される時に、初期化されます。

[lazy_static]: https://crates.io/crates/lazy_static

例えば、[Writing an OS in RustのVGA Text mode Lazy Statics]では、VGAにテキストを描画するグローバルインタフェース`WRITER`の実装で使用しています。

[Writing an OS in RustのVGA Text mode Lazy Statics]: https://os.phil-opp.com/vga-text-mode/#lazy-statics

```rust
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
```

`Mutex<Writer>`という初期化が非常に複雑なオブジェクトの参照が`static`変数になっていることがわかります。このように実行時にしか構築できない値も`static`変数にできる上、どこで初期化するかに悩まなくて済みます。

#### [bitflags]

**型安全**なビットマスクフラグを提供するクレートです。型安全であることがポイントで、誤ったビット操作を起こしにくいです。AndやOrのオペレータも実装されており、`bits()`メソッドで生の値を取り出すことができます。

[bitflags]: https://crates.io/crates/bitflags

```rust
#[macro_use]
extern crate bitflags;

bitflags! {
    struct Flags: u32 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const ABC = Self::A.bits | Self::B.bits | Self::C.bits;
    }
}

fn main() {
    let e1 = Flags::A | Flags::C;
    let e2 = Flags::B | Flags::C;
    assert_eq!((e1 | e2), Flags::ABC);   // union
    assert_eq!((e1 & e2), Flags::C);     // intersection
    assert_eq!((e1 - e2), Flags::A);     // set difference
    assert_eq!(!e2, Flags::A);           // set complement
    assert_eq!(e1.bits(), 5u32);         // get raw value
}
```

#### [bit_field]

ビットフィールドへのアクセスを簡単にするためのクレートです。`BitField`トレイトを提供しており、`i8`, `u8`, `usize`などの整数型が、トレイトを実装しています。

[bit_field]: https://crates.io/crates/bit_field

ビットフィールドへのアクセスは、次のように書けます。

```rust,ignore
let mut value: u32 = 0b110101;

assert_eq!(value.get_bit(1), false);
assert_eq!(value.get_bit(2), true);
assert_eq!(value.get_bits(2..6), 0b1101);

value.set_bit(2, true);
assert_eq!(value, 0b110111);

value.set_bits(0..2, 0b00);
assert_eq!(value, 0b110100);
```

#### [bitfield]

ビットフィールドを定義するマクロを提供するクレートです。`bit_field`とクレート名が似ていますが、別物です。

[bitfield]: https://docs.rs/bitfield/0.13.1/bitfield/

下のように、ビットフィールドを定義します。

```rust,ignore
bitfield! {
    #[derive(Clone, Copy, Debug)]
    pub struct PinSelect(u32);
    pub connected, set_connected: 31;
    reserved, _: 30, 6;
    pub port, set_port: 5;
    pub pin, set_pin: 4, 0;
};

fn main() {
    let mut reg = PinSelect(0);

    reg.set_pin(5);
    reg.set_port(0);
    reg.set_connected(1);
    assert_eq!(0x1000_0005, reg.all_bits());
}
```

#### [micromath]

軽量な数値計算ライブラリです。三角関数などがあります。加速度計など、センサドライバでの計算に利用できます。

[micromath]: https://crates.io/crates/micromath

#### [register-rs]

Rust製のRTOSである`Tock`で利用されているMMIO / CPUレジスタインタフェースです。読み書き可能、読み込み専用、書き込み専用、を表現するジェネリック構造体を提供します。

[register-rs]: https://crates.io/crates/register

#### [volatile_register]

メモリマップドレジスタへのvolatileアクセスを提供します。[register-rs]の簡易版、と言った印象です。

[volatile_register]: https://docs.rs/volatile-register/0.2.0/volatile_register/

```rust,ignore
use volatile_register::RW;

// メモリマップドレジスタブロックを表現するstructを作ります
/// Nested Vector Interrupt Controller
#[repr(C)]
pub struct Nvic {
    /// Interrupt Set-Enable
    pub iser: [RW<u32>; 8],
    reserved0: [u32; 24],
    /// Interrupt Clear-Enable
    pub icer: [RW<u32>; 8],
    reserved1: [u32; 24],
    // .. more registers ..
}

// ベースアドレスをキャストしてアクセスします
let nvic = 0xE000_E100 as *const Nvic;
// unsafeブロックが必要です
unsafe { (*nvic).iser[0].write(1) }
```

#### [embedded-graphics]

2Dグラフィックを簡単に描画するためのクレートです。次の機能を提供します。

- 1ビット / ピクセルの画像
- 8ビット / ピクセルの画像
- 16ビット / ピクセル画像
- プリミティブ
  - 行、四角、丸、三角
- テキスト

[embedded-graphics]: https://crates.io/crates/embedded-graphics

このクレートは、メモリアロケータも事前の巨大なメモリ領域確保も必要としません。
