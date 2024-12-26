## Requirements

`rustup` and `cargo`, followed by `cargo install bootimage`.

To run the kernel in QEMU (install `qemu-system` on Debian-based systems), just do `cargo run`.

(Manually with `qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-bsos/debug/bootimage-bsos.bin`)

## Nightly Rust

This requires nightly Rust as of the time of writing. This is due to some unstable `.cargo/config.toml` options that are essential.

```toml
// .cargo/config.toml
[unstable]
build-std = ["core", "compiler_builtins"]
build-std-features = ["compiler-builtins-mem"]
```

There's no real timeline for these, so this will probably require nightly for as long as this project lives.
