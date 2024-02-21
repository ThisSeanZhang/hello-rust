use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

use lazy_static::lazy_static;


// 因为 IDT 不能在栈上创建, 而我们目前还没有堆, 所以赋值为静态变量
// 但是 静态 muts 很容易出现数据竞争
// 所以我们使用 lazy_static
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}

