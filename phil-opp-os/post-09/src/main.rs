#![no_std]
#![no_main]

// 开启自定义的测试方法 需要在 no_std 的情况下使用
#![feature(custom_test_frameworks)]
// 所有 test 的处理函数
#![test_runner(post_09::test_runner)]
// test 的入口函数被重命名了
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

use post_09::println;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // use post_09::memory::active_level_4_table;
    use post_09::memory;
    use x86_64::{structures::paging::Page, VirtAddr}; 
    // use x86_64::structures::paging::PageTable;
    
    println!("Hello World{}", "!");
    post_09::init();


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;


    // 要求分配不存在的内存时将会发生崩溃
    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};
    
    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    post_09::hlt_loop();
}

/// https://github.com/rust-lang/rust-analyzer/issues/4490
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    post_09::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    post_09::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

