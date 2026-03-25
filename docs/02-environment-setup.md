# xv6-riscv in Rust: Environment Setup

This chapter sets up a reproducible Rust kernel toolchain for RISC-V and verifies every required tool.

## Target platform
- Host: Linux or WSL2 Ubuntu.
- Guest: `qemu-system-riscv64` (`virt` machine).
- Kernel target: `riscv64gc-unknown-none-elf`.

## 1. Install base packages
```bash
sudo apt update
sudo apt install -y \
  build-essential \
  curl git \
  qemu-system-misc \
  gdb-multiarch \
  llvm lld clang
```

Verify:
```bash
qemu-system-riscv64 --version
gdb-multiarch --version
ld.lld --version
```

## 2. Install Rust toolchain
```bash
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src llvm-tools-preview
rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils
```

Verify:
```bash
rustc -Vv
cargo -V
rustup target list --installed | rg riscv64gc-unknown-none-elf
cargo objcopy --help >/dev/null && echo "cargo-objcopy ok"
```

## 3. Create per-project cargo target config
Create `.cargo/config.toml`:
```toml
[build]
target = "riscv64gc-unknown-none-elf"

[target.riscv64gc-unknown-none-elf]
linker = "rust-lld"
rustflags = [
  "-C", "link-arg=-Tkernel/linker.ld",
  "-C", "relocation-model=static",
]
```

## 4. Minimal no_std smoke crate
```bash
cargo new --lib kernel
```

Edit `kernel/src/lib.rs`:
```rust
#![no_std]

#[no_mangle]
pub extern "C" fn kernel_probe() -> usize {
    42
}
```

Then run:
```bash
cargo build -p kernel
```

## 5. QEMU boot smoke test
When boot code exists later, this command pattern is expected:
```bash
qemu-system-riscv64 \
  -machine virt \
  -nographic \
  -bios default \
  -kernel build/kernel.bin
```

You should also reserve a debug variant:
```bash
qemu-system-riscv64 \
  -machine virt -nographic -bios default \
  -kernel build/kernel.bin \
  -S -s
```

Then attach GDB:
```bash
gdb-multiarch build/kernel.elf \
  -ex "set arch riscv:rv64" \
  -ex "target remote :1234"
```

## 6. Common setup mistakes
- Using stable toolchain with nightly-only kernel features.
- Missing `rust-src` causes core/alloc build failures.
- Using host linker instead of `rust-lld`.
- Not passing linker script via `-T...`.

## 7. Chapter deliverable
At the end of this chapter:
- Rust cross-target compiles.
- QEMU and GDB commands are available.
- Project can proceed to workspace structure.

## Checklist
- [ ] `qemu-system-riscv64` works.
- [ ] `gdb-multiarch` works.
- [ ] `rustup target add riscv64gc-unknown-none-elf` completed.
- [ ] `cargo build` for a `no_std` crate succeeds.
- [ ] `.cargo/config.toml` points to `rust-lld` and linker script.

Next: [03-project-structure.md](03-project-structure.md)
