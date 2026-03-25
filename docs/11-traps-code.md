# xv6-riscv in Rust: Trap Handler Implementation

This chapter implements trap entry/exit and Rust dispatch logic.

## 1. Trapframe layout
Trapframe must match assembly save/restore order.

```rust
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t: [usize; 7],
    pub s: [usize; 12],
    pub a: [usize; 8],
    pub sepc: usize,
    pub sstatus: usize,
}
```

## 2. Assembly entry stub
Responsibilities:
- Switch to kernel stack if needed.
- Save registers into trapframe.
- Call Rust: `trap::handle_trap(tf_ptr)`.
- Restore registers and execute `sret`.

Keep this file tiny and deterministic.

## 3. Rust trap dispatcher
```rust
pub extern "C" fn handle_trap(tf: &mut TrapFrame) {
    match decode_scause() {
        TrapCause::UserEcall => {
            tf.sepc += 4;
            tf.a[0] = syscall::dispatch(tf.a[7], tf.a[0], tf.a[1], tf.a[2]);
        }
        TrapCause::TimerInterrupt => {
            timer::on_tick();
            proc::maybe_yield_from_interrupt(tf);
        }
        TrapCause::PageFault { addr } => {
            proc::kill_current(Fault::Page(addr));
        }
        TrapCause::ExternalInterrupt => {
            drivers::plic::handle();
        }
        TrapCause::Unknown(code) => {
            panic!("unknown trap: {code:#x}");
        }
    }
}
```

## 4. Syscall table pattern
```rust
type SysFn = fn(usize, usize, usize) -> usize;

static SYSCALLS: &[SysFn] = &[
    sys_getpid,
    sys_write,
    sys_exit,
];
```

Validate syscall number bounds before indexing.

## 5. Return-to-user requirements
Before `sret`:
- `sepc` points to next user instruction.
- `sstatus` has SPP=User and proper interrupt bits.
- user `satp` (if switching) is active and fenced.

## 6. Debug checklist
- Breakpoint in trap asm entry.
- Verify trapframe values after save.
- Verify `sepc` change for `ecall`.
- Verify return to expected user PC.

## Checklist
- [ ] Trapframe is `#[repr(C)]` and asm-compatible.
- [ ] Trap entry/exit saves and restores registers correctly.
- [ ] Rust dispatcher handles syscall/timer/fault paths.
- [ ] Return-to-user state is explicitly validated.

Next: [12-process-structure.md](12-process-structure.md)
