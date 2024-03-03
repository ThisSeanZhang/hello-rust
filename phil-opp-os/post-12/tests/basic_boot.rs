#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(post_12::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use post_12::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}


#[test_case]
fn test_println() {
    println!("test_println output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    post_12::test_panic_handler(info)
}
