#![no_std]
#![no_main]

mod vga_buffer;

#[cfg(not(test))]
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
/// https://github.com/rust-lang/rust-analyzer/issues/4490
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}



