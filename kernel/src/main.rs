#![no_std]
#![no_main]
#![allow(clippy::cast_possible_truncation)]

use core::arch::asm;

static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    // Ensure we got a framebuffer.
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        assert!(
            framebuffer_response.framebuffer_count > 0,
            "No framebuffer provided!"
        );

        // Get the first framebuffer's information.
        let framebuffer = &framebuffer_response.framebuffers()[0];

        for i in 0..100 {
            // Calculate the pixel offset using the framebuffer information we obtained above.
            // We skip `i` scanlines (pitch is provided in bytes) and add `i * 4` to skip `i` pixels forward.
            let pixel_offset = i * framebuffer.pitch as usize + i * 4;

            // Write 0xFFFFFFFF to the provided pixel offset to fill it white.
            //
            // SAFETY: We can safely unwrap the result of `as_ptr()` because the framebuffer
            // address is guaranteed to be provided by the bootloader. We can also safely assume
            // that the framebuffer address is aligned to 4 bytes, because the bootloader
            // guarantees that the framebuffer pitch is a multiple of 4.
            unsafe {
                #[allow(clippy::cast_ptr_alignment)]
                let pixel = framebuffer
                    .address
                    .as_ptr()
                    .unwrap()
                    .add(pixel_offset)
                    .cast::<u32>();
                pixel.write(0xFFFF_FFFF);
            }
        }
    }

    hcf();
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf();
}
