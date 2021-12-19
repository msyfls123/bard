# https://colobu.com/2019/12/18/How-to-Cross-Compile-from-Mac-to-Linux-on-Rust/
CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="x86_64-linux-musl-gcc" CROSS_COMPILE=x86_64-linux-musl- cargo build --release --target x86_64-unknown-linux-musl
