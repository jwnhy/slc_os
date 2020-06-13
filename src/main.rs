#![no_std]// do not link the Rust standard library
#![no_main]// disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(slc_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

use slc_os::{allocator, println};
use slc_os::memory::BootInfoFrameAllocator;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
// Use bootloader marco to check and unwrap entry function into _start, no mangled version.
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    slc_os::init();
    use slc_os::memory;
    use x86_64::VirtAddr;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = reference_counted.clone();
    println!("current ref count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("ref count is {} now", Rc::strong_count(&cloned_reference));

    println!("It did not crash!");
    #[cfg(test)]
        test_main();
    slc_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    slc_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    slc_os::test_panic_handler(info)
}