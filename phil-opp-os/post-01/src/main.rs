#![no_std]
#![no_main]

#[cfg(not(test))]
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
/// https://github.com/rust-lang/rust-analyzer/issues/4490
#[cfg(not(test))]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
