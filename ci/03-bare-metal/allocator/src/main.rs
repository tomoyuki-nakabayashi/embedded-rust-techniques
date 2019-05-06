#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

use core::panic::PanicInfo;
use core::ptr;
use core::cell::UnsafeCell;
use core::alloc::GlobalAlloc;

extern crate alloc;
use alloc::alloc::Layout;
use alloc::vec::Vec;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    init_ram();

    let mut xs = Vec::new();
    xs.push(42);
    xs.push(83);
    println!("{:?}", xs);

    loop {}
}

// *シングル*コアシステム用のポインタを増加するだけのアロケータ
struct BumpPointerAlloc {
    head: UnsafeCell<usize>,
    end: usize,
}

unsafe impl Sync for BumpPointerAlloc {}

unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let head = self.head.get();

        let align = layout.align();
        let res = *head % align;
        let start = if res == 0 { *head } else { *head + align - res };
        if start + align > self.end {
            // ヌルポインタはメモリ不足の状態を知らせます
            ptr::null_mut()
        } else {
            *head = start + align;
            start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // このアロケータはメモリを解放しません
    }
}

// グローバルメモリアロケータの宣言
// ユーザはメモリ領域の`[0x2000_0100, 0x2000_0200]`がプログラムの他の部分で使用されないことを
// 保証しなければなりません
#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc {
    head: UnsafeCell::new(0x2000_0100),
    end: 0x2000_0200,
};

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}

// 以降、変更なし
// リセットベクタは、リセットハンドラへのポインタです
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

fn write_byte(c: u8) {
    // TX buffer on LM3S6965 microcontroller.
    const UART0_TX: *mut u8 = 0x4000_c000 as *mut u8;
    unsafe {
        *UART0_TX = c;
    }
}

unsafe fn init_ram() {
    // RAMの初期化
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);
}

use core::fmt::{self, Write};
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn _print(args: fmt::Arguments) {
    let mut writer = UartWriter {};
    writer.write_fmt(args).unwrap();
}

struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            write_byte(c);
        }
        Ok(())
    }
}