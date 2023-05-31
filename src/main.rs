//不使用标准库
#![no_std]
//告诉Rust编译器我们不使用预定义的入口点
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(link_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;
use x86_64::instructions::port::Port;
mod vga_buffer;
mod serial;
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}",_info);
    link_os::hlt_loop()
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    link_os::test_panic_handler(info)
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}","!");
    link_os::init();//interrupt init
    // fn stack_overflow() {
    //     stack_overflow(); // 每一次递归都会将返回地址入栈
    // }
    //
    // // 触发 stack overflow
    // stack_overflow();
    //invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();
    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash");
    link_os::hlt_loop()
}


