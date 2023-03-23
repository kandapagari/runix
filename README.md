- Change to rust nightly to get access to unstable packages like `build-std` using:
    ```bash
    rustup override set nightly
    rustc --version
    rustup component add rust-src
    ```
- Then install bootimage and llvm-tools using:
    ```bash
    cargo install bootimage
    rustup component add llvm-tools-preview
    cargo bootimage
    ```
- After this you will be able to use `cargo run` or `cargo build`
- To test the bootimage in VM like `qemu` run:
    ```bash
    qemu-system-x86_64 -drive format=raw,file=target/x86_64-runix-none/debug/bootimage-runix.bin
    ```