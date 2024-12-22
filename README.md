## Requirements

`rustup` and `cargo`, followed by `cargo install bootimage`.

To run the kernel in QEMU (install `qemu-system` on Debian-based systems), just do `cargo run`.

(Manually with `qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-bsos/debug/bootimage-bsos.bin`)
