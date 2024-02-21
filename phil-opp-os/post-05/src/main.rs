#![no_std]
#![no_main]

// 开启自定义的测试方法 需要在 no_std 的情况下使用
#![feature(custom_test_frameworks)]
// 所有 test 的处理函数
#![test_runner(post_05::test_runner)]
// test 的入口函数被重命名了
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use post_05::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    post_05::init();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

/// https://github.com/rust-lang/rust-analyzer/issues/4490
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    post_05::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

