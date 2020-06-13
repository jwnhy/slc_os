#![no_std]// do not link the Rust standard library
#![no_main]// disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(slc_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> !{
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo)-> ! {
    slc_os::test_panic_handler(info)
}

use slc_os::{println, serial_print, serial_println};

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}


