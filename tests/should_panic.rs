#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;

use bsos::{hlt_loop, qemu, serial_print, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    qemu::exit(qemu::ExitCode::Success);
    hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    hlt_loop();
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        qemu::exit(qemu::ExitCode::Failed);
    }
    qemu::exit(qemu::ExitCode::Success);
}

#[test_case]
#[allow(clippy::missing_panics_doc)]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
