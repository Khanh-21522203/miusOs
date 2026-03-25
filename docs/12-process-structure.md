# xv6-riscv in Rust: Process Structures and Management

This chapter defines task/process data structures and state transitions.

## 1. Core process model
A process includes:
- Execution context (registers + kernel stack).
- Address space (page table).
- Open file table.
- Scheduling state.

## 2. Rust data types
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskState {
    Unused,
    Runnable,
    Running,
    Sleeping,
    Zombie,
}

pub struct Task {
    pub pid: u32,
    pub state: TaskState,
    pub kstack_top: usize,
    pub context: Context,
    pub pagetable: mm::PageTable,
    pub trapframe_pa: usize,
    pub parent: Option<u32>,
    pub exit_code: i32,
}
```

## 3. Locking discipline
Rules for correctness:
- Process table lock protects task slot allocation.
- Per-task lock protects task mutable state.
- Never sleep while holding global lock.

Document lock order to avoid deadlock.

## 4. PID allocation
Use monotonic PID allocator with wrap-around checks:
```rust
static NEXT_PID: core::sync::atomic::AtomicU32 =
    core::sync::atomic::AtomicU32::new(1);
```

## 5. State transitions
Valid transitions:
- `Unused -> Runnable` (new task)
- `Runnable -> Running` (scheduled)
- `Running -> Sleeping` (blocking wait)
- `Running -> Runnable` (yield/preempt)
- `Running -> Zombie` (exit)
- `Zombie -> Unused` (reaped)

Reject invalid transitions in debug builds.

## 6. First process (`init`)
Boot flow should create one runnable init process with:
- user page table
- initial user stack
- entry PC to first user program

## Checklist
- [ ] `Task` struct separates scheduling and memory data clearly.
- [ ] Lock ownership rules are written and enforced.
- [ ] PID allocation is race-safe.
- [ ] Task state transitions are explicit and validated.

Next: [13-exec-syscall.md](13-exec-syscall.md)
