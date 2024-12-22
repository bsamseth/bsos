#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod qemu;
mod serial;
#[cfg(test)]
mod testing;
mod vga;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");

    #[cfg(test)]
    {
        serial_println!("[failed]\n");
        serial_println!("Error: {}", info);
        qemu::exit(qemu::ExitCode::Failed);
    }

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world from a macro{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}
