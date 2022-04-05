#![feature(panic_info_message)]
#![no_main]
#![no_std]
mod lang_items;

#[macro_use]
mod console;

mod sbi;

use core::arch::global_asm;

use sbi::console_putchar;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}