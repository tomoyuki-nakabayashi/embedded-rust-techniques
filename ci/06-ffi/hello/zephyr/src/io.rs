//! IO wrapper which provides `safe` println macro using Zephyr API.

use zephyr_sys;
use core::fmt;
use cty;

/// FFI to get stdout file pointer which is defined as a macro.
extern "C" {
    pub fn stdout_as_ptr_mut() -> *mut zephyr_sys::FILE;
}

/// Pseudo writer which uses Zephyr `fwrite` API.
/// Because `fwrite` does not guarantee its atomicity, this wrapper
/// does not provide any lock mechanism.
pub struct DebugWriter {}

impl fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // safe: `fwrite` does not need to guarantee the atomicity.
        unsafe {
            zephyr_sys::fwrite(s.as_ptr() as *const cty::c_void, s.len(), 1, stdout_as_ptr_mut());
        }
        Ok(())
    }
}

/// Like the `print!` macro in the standard library, but calls printf
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but calls printf
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut writer = DebugWriter {};
    writer.write_fmt(args).unwrap(); // Always returns Ok.
}
