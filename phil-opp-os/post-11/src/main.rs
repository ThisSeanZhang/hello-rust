#![no_std]
#![no_main]

// 开启自定义的测试方法 需要在 no_std 的情况下使用
#![feature(custom_test_frameworks)]
// 所有 test 的处理函数
#![test_runner(post_11::test_runner)]
// test 的入口函数被重命名了
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

use post_11::println;


entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // use post_11::memory::active_level_4_table;
    use post_11::memory;
    use post_11::allocator; 
    use post_11::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr}; 
    // use x86_64::structures::paging::PageTable;
    
    println!("Hello World{}", "!");
    post_11::init();


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // new
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));
    
    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    post_11::hlt_loop();
}

/// https://github.com/rust-lang/rust-analyzer/issues/4490
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    post_11::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    post_11::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

