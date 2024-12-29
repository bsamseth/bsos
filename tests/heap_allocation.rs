#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(bsos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::missing_panics_doc)]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use core::panic::PanicInfo;

use bootloader::{entry_point, BootInfo};
use bsos::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
};
use x86_64::VirtAddr;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    bsos::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    bsos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    bsos::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..bsos::allocator::HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
