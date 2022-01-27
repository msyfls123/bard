# https://colobu.com/2019/12/18/How-to-Cross-Compile-from-Mac-to-Linux-on-Rust/
# FIXME: Not working now, since librocksdb-sys needs linux.h header file and musl building tools.
CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER="x86_64-linux-musl-gcc" CROSS_COMPILE=x86_64-linux-musl- cargo build --bin server --release --target x86_64-unknown-linux-musl
