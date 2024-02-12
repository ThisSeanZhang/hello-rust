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

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    
    loop {}
}
