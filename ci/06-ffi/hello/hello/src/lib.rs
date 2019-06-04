#![no_std]

use zephyr::{print, println};

#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello Rust");
}

use core::panic::PanicInfo;
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("panic! {:?}", info);
    loop {}
}
