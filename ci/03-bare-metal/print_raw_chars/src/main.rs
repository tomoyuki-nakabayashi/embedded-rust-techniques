#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    for c in b"Hello Rust!".iter() {
        write_byte(*c);
    }

    // 戻れないため、ここで無限ループに入ります
    loop {}
}

fn write_byte(c: u8) {
    // TX buffer on LM3S6965 microcontroller.
    const UART0_TX: *mut u8 = 0x4000_c000 as *mut u8;
    unsafe {
        *UART0_TX = c;
    }
}

// リセットベクタは、リセットハンドラへのポインタです
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}