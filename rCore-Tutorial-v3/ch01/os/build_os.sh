file target/riscv64gc-unknown-none-elf/release/os
cargo build --release
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
stat target/riscv64gc-unknown-none-elf/release/os
stat target/riscv64gc-unknown-none-elf/release/os.bin