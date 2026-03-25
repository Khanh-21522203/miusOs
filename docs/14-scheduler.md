# xv6-riscv in Rust: Scheduler and Context Switching

This chapter introduces a simple, correct scheduler before optimizations.

## 1. Scheduler policy
Start with round-robin over runnable tasks.

Goals:
- Fairness for teaching workloads.
- Predictable behavior for debugging.
- Minimal complexity.

## 2. Scheduler loop sketch
```rust
pub fn run() -> ! {
    loop {
        if let Some(next) = proc::pick_next_runnable() {
            context::switch_to(next);
        } else {
            arch::riscv64::wfi();
        }
    }
}
```

## 3. Context switch boundary
Assembly switches callee-saved registers and stack pointer between two `Context` structs.

Rust type must match asm layout exactly:
```rust
#[repr(C)]
pub struct Context {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}
```

## 4. Preemption model
Use timer interrupts to force periodic reschedule points.

On tick:
- account current task runtime
- set reschedule flag or directly yield if safe

## 5. Lock invariants
- Never switch while holding global scheduler lock.
- Task state update and run-queue update must be atomic wrt other CPUs.

## 6. Verification scenarios
- One runnable task: no starvation or lockup.
- Two CPU-bound tasks: both make progress.
- Sleeping task wakes on event and re-enters runnable queue.

## Checklist
- [ ] Round-robin scheduler loop implemented.
- [ ] Context struct layout is asm-compatible.
- [ ] Timer interrupts can trigger rescheduling.
- [ ] No lock is held across context switch.

Next: [15-file-descriptor.md](15-file-descriptor.md)
