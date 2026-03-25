# xv6-riscv in Rust: Overview

## Goal
Build a compact teaching OS from scratch on RISC-V, using Rust for kernel code and a small amount of assembly for CPU entry points.

## What you will learn
- How a RISC-V machine boots into your kernel.
- How virtual memory (Sv39) isolates kernel/user spaces.
- How traps/syscalls cross protection boundaries.
- How processes, scheduling, files, and pipes work together.
- How to debug low-level failures with QEMU + GDB.

## Rust-first mindset
Instead of translating xv6 C line-by-line, we keep xv6 ideas but redesign interfaces around Rust safety.

Core rule:
- Safe API at module boundaries.
- `unsafe` only in small internal blocks where hardware or raw pointers are unavoidable.

Example skeleton:
```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn rust_main(hart_id: usize, dtb_pa: usize) -> ! {
    let _ = (hart_id, dtb_pa);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

## What stays in assembly
- Reset entry symbol (`_start`).
- Initial stack pointer setup.
- Trap trampoline save/restore registers.
- Context switch routine.

Everything else should be Rust where practical.

## Prerequisites
- Rust basics: ownership, borrowing, enums, traits.
- Basic Linux shell usage.
- Basic understanding of CPU registers and call stack.
- Willingness to read small assembly snippets.

## Non-goals
- Production-grade performance.
- Full POSIX compatibility.
- Modern security hardening.

This is a learning kernel.

## Suggested milestone cadence
1. Boot banner in QEMU.
2. Timer interrupt increments a counter.
3. Simple syscall (`getpid`) from user mode.
4. Process creation and round-robin scheduling.
5. Basic file and pipe I/O.

## Deliverable for this chapter
- You can explain the boundary between Rust-safe code and hardware-facing unsafe code.
- You understand why this project is mostly Rust but not “all Rust, no asm.”

## Checklist
- [ ] I understand the project goals and non-goals.
- [ ] I can explain why `#![no_std]` and `#![no_main]` are required.
- [ ] I know where `unsafe` is expected in kernel code.
- [ ] I am ready to install the toolchain.

Next: [02-environment-setup.md](02-environment-setup.md)
