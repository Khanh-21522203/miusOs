# xv6-riscv in Rust: Project Structure

This chapter defines a clean Rust workspace so each subsystem has a clear ownership boundary.

## 1. Recommended repository layout
```text
xv6-riscv/
├── Cargo.toml                 # workspace root
├── rust-toolchain.toml
├── .cargo/
│   └── config.toml
├── kernel/
│   ├── Cargo.toml
│   ├── linker.ld
│   └── src/
│       ├── lib.rs
│       ├── main.rs            # optional split for entry
│       ├── arch/
│       │   └── riscv64/
│       │       ├── csr.rs
│       │       ├── trap.S
│       │       └── switch.S
│       ├── mm/
│       │   ├── addr.rs
│       │   ├── frame.rs
│       │   └── page_table.rs
│       ├── trap/
│       │   ├── mod.rs
│       │   └── syscall.rs
│       ├── proc/
│       │   ├── task.rs
│       │   └── sched.rs
│       ├── fs/
│       │   ├── inode.rs
│       │   └── file.rs
│       └── drivers/
│           └── uart.rs
├── user/
│   ├── Cargo.toml
│   └── src/
│       └── bin/
│           ├── init.rs
│           └── sh.rs
├── xtask/
│   ├── Cargo.toml
│   └── src/main.rs            # cargo run -p xtask -- run/debug/test
├── scripts/
│   ├── run-qemu.sh
│   └── debug-gdb.sh
└── docs/
```

## 2. Workspace manifest example
`Cargo.toml` at repo root:
```toml
[workspace]
members = ["kernel", "user", "xtask"]
resolver = "2"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
lto = true
```

## 3. Module ownership rules
- `arch/riscv64/*`: hardware-facing details and asm glue.
- `mm/*`: address types, allocator, page table.
- `trap/*`: trap entry, dispatch, syscall boundary.
- `proc/*`: task state, scheduler, context switching policy.
- `fs/*`: inode/files/pipe abstractions.
- `drivers/*`: UART/block device interactions.

Avoid mixing these layers in one file.

## 4. Rust kernel coding constraints
- `#![no_std]` in kernel and low-level user runtime.
- `panic = "abort"` for predictable bring-up behavior.
- Use `#[repr(C)]` for shared layouts with asm or disk.
- Wrap raw pointers in typed APIs whenever possible.

## 5. Minimal entry skeleton
```rust
#![no_std]
#![no_main]

mod arch;
mod mm;
mod trap;
mod proc;
mod fs;
mod drivers;

#[no_mangle]
pub extern "C" fn rust_main(hart_id: usize, dtb_pa: usize) -> ! {
    let _ = (hart_id, dtb_pa);
    loop {}
}
```

## 6. Chapter deliverable
- Workspace is created.
- Kernel modules are scaffolded.
- You can explain what belongs in each module.

## Checklist
- [ ] Workspace root configured (`members = ["kernel", "user", "xtask"]`).
- [ ] `kernel/linker.ld` exists and is referenced by rustflags.
- [ ] Subsystem folders (`mm`, `trap`, `proc`, `fs`, `drivers`) exist.
- [ ] Assembly files are isolated under `arch/riscv64`.

Next: [04-bootloader-explained.md](04-bootloader-explained.md)
