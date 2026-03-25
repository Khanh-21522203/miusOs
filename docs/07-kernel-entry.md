# xv6-riscv in Rust: Kernel Entry and Initialization

This chapter defines what `rust_main` must do, in order, to produce a stable kernel.

## 1. Initialization order
Use this sequence and do not reorder casually:
1. Clear `.bss`.
2. Initialize UART logging.
3. Initialize trap vectors (per hart).
4. Initialize physical frame allocator.
5. Build and enable kernel page table.
6. Initialize process table and first process.
7. Start scheduler.

Runtime lifecycle:
```text
_start (asm)
  -> rust_main(hart, dtb)
     -> early_bss_clear()
     -> uart::init()
     -> trap::init_per_hart()
     -> mm::init_global()
     -> mm::enable_paging()
     -> proc::init_global()
     -> proc::spawn_init()
     -> sched::run()   (never returns)
```

## 2. Kernel entry skeleton
```rust
#[no_mangle]
pub extern "C" fn rust_main(hart_id: usize, dtb_pa: usize) -> ! {
    arch::riscv64::clear_bss();
    drivers::uart::init();
    log::info!("boot hart={} dtb={:#x}", hart_id, dtb_pa);

    trap::init_per_hart();
    mm::init_global();
    mm::enable_kernel_paging();

    proc::init_global();
    proc::spawn_init().expect("init process");

    sched::run()
}
```

## 3. Global vs per-hart init
- Global once: frame allocator, global process table, inode cache.
- Per-hart: trap vector (`stvec`), local timer setup, current task pointer.

Use `AtomicBool` or `Once` to protect one-time initialization.

## 4. Panic strategy during bring-up
Use `panic = "abort"` and a panic handler that prints to UART then halts.

```rust
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    drivers::uart::println("panic");
    if let Some(loc) = info.location() {
        drivers::uart::println(loc.file());
    }
    arch::riscv64::halt()
}
```

## 5. Bring-up checks
- You see deterministic boot logs every run.
- Page table enable does not immediately trap-loop.
- Scheduler runs even with one runnable task.

## Checklist
- [ ] `rust_main` follows a deterministic init sequence.
- [ ] Global/per-hart init responsibilities are separated.
- [ ] Panic path prints something useful.
- [ ] Kernel reaches scheduler without crash.

Next: [08-page-table.md](08-page-table.md)
