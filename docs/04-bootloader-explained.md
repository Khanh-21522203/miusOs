# xv6-riscv in Rust: Bootloader Explained

This chapter explains how control moves from machine reset to Rust kernel code.

## 1. Boot flow overview
On QEMU `virt`, firmware (OpenSBI) usually runs before your S-mode kernel. Your startup code must satisfy a strict handoff contract.

Boot sequence:
```text
Power on / Reset
  -> OpenSBI (M-mode firmware)
     -> loads kernel image
     -> sets up basic machine state
     -> enters S-mode at kernel entry
        -> _start (assembly)
           -> set stack for this hart
           -> clear .bss (or call Rust helper)
           -> call rust_main(hart_id, dtb_pa)
              -> kernel init sequence
```

## 2. Why assembly is still required
Rust cannot be your first instruction because:
- CPU needs stack pointer setup first.
- ABI registers must be in known state.
- Early control register setup may be required.

So `_start` is assembly; most logic after that is Rust.

## 3. Entry contract
Recommended entry signature:
```rust
#[no_mangle]
pub extern "C" fn rust_main(hart_id: usize, dtb_pa: usize) -> ! {
    // hart_id from a0, dtb physical address from a1
    let _ = (hart_id, dtb_pa);
    loop {}
}
```

## 4. Multi-hart model
Early boot is easier if:
- Hart 0 performs global init.
- Other harts wait until global init completes.

Use an atomic flag:
```rust
use core::sync::atomic::{AtomicBool, Ordering};

static BOOT_DONE: AtomicBool = AtomicBool::new(false);

fn wait_for_boot() {
    while !BOOT_DONE.load(Ordering::Acquire) {
        core::hint::spin_loop();
    }
}
```

## 5. Common boot failure patterns
- No UART output: stack pointer or jump target wrong.
- Immediate trap loop: bad trap vector or invalid `sepc`.
- Only hart 0 works: missing per-hart stack/trap setup.

## 6. Fast verification strategy
- Break at `_start` in GDB.
- Single-step until `rust_main`.
- Confirm `a0`/`a1` match expected hart/DTB values.

## Checklist
- [ ] I can describe OpenSBI -> `_start` -> `rust_main`.
- [ ] I know why startup cannot be pure Rust.
- [ ] I have a plan for multi-hart boot synchronization.
- [ ] I know which registers carry kernel entry arguments.

Next: [05-bootloader-assembly.md](05-bootloader-assembly.md)
