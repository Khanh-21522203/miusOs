# Detailed Work Breakdown

This document turns the roadmap into actionable engineering tasks by subsystem.

Usage rule:
- Pick tasks from the current stage only.
- Each task must define file targets, interface surface, and acceptance criteria.

## Planned Workspace File Tree (Target)

```text
/home/khanh/Projects/miusOS/
  Cargo.toml
  rust-toolchain.toml
  .cargo/config.toml
  linker.ld
  src/
    lib.rs
    arch/riscv64/
      mod.rs
      start.S
      trap.S
      switch.S
      csr.rs
    mm/
      mod.rs
      addr.rs
      frame.rs
      page_table.rs
    trap/
      mod.rs
      syscall.rs
    proc/
      mod.rs
      task.rs
      sched.rs
    fs/
      mod.rs
      file.rs
      inode.rs
      pipe.rs
    drivers/
      mod.rs
      uart.rs
  scripts/
    run.sh
    debug.sh
```

## Critical Control Flow Diagram (Syscall Path)

```text
user mode
  |
  | place syscall id in a7, args in a0-a5
  | ecall
  v
trap entry asm (`trap.S`)
  |
  | save regs -> TrapFrame
  v
Rust dispatcher (`trap/mod.rs`)
  |
  +--> decode cause
  |      |
  |      +--> UserEcall -> syscall table (`trap/syscall.rs`)
  |      +--> Timer interrupt -> scheduler tick path
  |      `--> Fault -> kill/diagnose
  |
  v
trap return asm
  |
  | restore regs, sret
  v
user mode resumes
```

## Ownership and Boundaries

### `arch/riscv64` (hardware boundary)
Responsibilities:
- startup entry
- CSR read/write helpers
- trap entry/exit assembly
- context switch assembly

Must not own:
- scheduling policy
- process table logic
- filesystem logic

Acceptance criteria:
- exported low-level functions are minimal and stable
- assembly and Rust struct layouts are consistent

### `mm` (memory boundary)
Responsibilities:
- typed addresses
- frame allocation
- page table map/unmap/translate
- kernel page table activation

Must not own:
- syscall dispatch
- filesystem objects

Acceptance criteria:
- all page operations enforce alignment checks
- unsafe pointer walks localized and documented

### `trap`
Responsibilities:
- trap cause decode
- syscall ABI glue
- trapframe lifecycle integration

Must not own:
- page table internals
- run queue implementation details

Acceptance criteria:
- `ecall` path increments `sepc`
- unknown traps are diagnosable and not silently ignored

### `proc`
Responsibilities:
- task states and lifecycle
- scheduler and preemption
- current-task tracking per hart

Must not own:
- inode/path lookup details

Acceptance criteria:
- valid state transitions only
- no lock held across context switch

### `fs`
Responsibilities:
- inode and path walk
- fd table behavior
- pipe object behavior

Must not own:
- timer interrupt logic

Acceptance criteria:
- fd bounds enforcement
- deterministic file and pipe semantics

### `drivers`
Responsibilities:
- UART bring-up and output primitives
- optional additional devices later

Must not own:
- syscall policy

Acceptance criteria:
- early panic output remains available

## Detailed Task Queue

## 1. Root Workspace Setup
File targets:
- `/home/khanh/Projects/miusOS/Cargo.toml`
- `/home/khanh/Projects/miusOS/rust-toolchain.toml`
- `/home/khanh/Projects/miusOS/.cargo/config.toml`

Tasks:
1. Define root package metadata and panic strategy.
2. Pin nightly toolchain.
3. Configure riscv target and linker script flags.

Done when:
- package metadata resolves
- root crate compiles for riscv target

## 2. Root Crate Skeleton
File targets:
- `/home/khanh/Projects/miusOS/Cargo.toml`
- `/home/khanh/Projects/miusOS/src/lib.rs`
- `/home/khanh/Projects/miusOS/linker.ld`

Tasks:
1. Set `#![no_std]` and `#![no_main]` where applicable.
2. Add entry symbol integration.
3. Define linker sections and exported symbols (`sbss`, `ebss`, `kernel_end`).

Done when:
- linker symbols are accessible in Rust
- resulting ELF has expected sections

## 3. Startup and Early Boot
File targets:
- `/home/khanh/Projects/miusOS/src/arch/riscv64/start.S`
- `/home/khanh/Projects/miusOS/src/arch/riscv64/mod.rs`
- `/home/khanh/Projects/miusOS/src/drivers/uart.rs`

Tasks:
1. Setup per-hart stacks and call `rust_main`.
2. Ensure 16-byte stack alignment at call boundaries.
3. Add infinite park loop if `rust_main` returns.
4. Add UART init/write helpers for logs.

Done when:
- `_start` is hit by GDB
- boot banner prints on QEMU serial

## 4. MM Base and Paging
File targets:
- `/home/khanh/Projects/miusOS/src/mm/addr.rs`
- `/home/khanh/Projects/miusOS/src/mm/frame.rs`
- `/home/khanh/Projects/miusOS/src/mm/page_table.rs`
- `/home/khanh/Projects/miusOS/src/arch/riscv64/csr.rs`

Tasks:
1. Implement address wrappers and page helper functions.
2. Implement frame allocator API and initial implementation.
3. Implement Sv39 walk and mapping APIs.
4. Add SATP write + global TLB flush.

Done when:
- one-page map/unmap/translate tests pass
- kernel still executes after enabling paging

## 5. Trap Path and Syscalls
File targets:
- `/home/khanh/Projects/miusOS/src/arch/riscv64/trap.S`
- `/home/khanh/Projects/miusOS/src/trap/mod.rs`
- `/home/khanh/Projects/miusOS/src/trap/syscall.rs`

Tasks:
1. Define trapframe layout in Rust (`#[repr(C)]`).
2. Save/restore register state in asm to match layout.
3. Decode trap causes.
4. Add syscall dispatch table bounds checks.
5. Validate user pointers before memory access.

Done when:
- user syscall roundtrip works
- timer interrupt enters trap path and returns cleanly

## 6. Process and Scheduler
File targets:
- `/home/khanh/Projects/miusOS/src/proc/task.rs`
- `/home/khanh/Projects/miusOS/src/proc/sched.rs`
- `/home/khanh/Projects/miusOS/src/arch/riscv64/switch.S`

Tasks:
1. Define task states and enforce valid transitions.
2. Implement PID allocator and task slot lifecycle.
3. Add run queue selection (round-robin).
4. Add asm context switch matching `Context` layout.
5. Integrate timer preemption logic.

Done when:
- two runnable tasks both make progress
- no lock-order violations across switch path

## 7. Exec Implementation
File targets:
- `/home/khanh/Projects/miusOS/src/trap/syscall.rs`
- `/home/khanh/Projects/miusOS/src/proc/task.rs`
- `/home/khanh/Projects/miusOS/src/mm/page_table.rs`
- `/home/khanh/Projects/miusOS/src/fs/inode.rs`

Tasks:
1. Copy `path` and `argv` from user to kernel buffers with strict limits.
2. Load and validate ELF.
3. Build new address space and user stack.
4. Swap current process memory only after full build success.
5. On any error, preserve old process image.

Done when:
- valid ELF exec succeeds with same PID
- invalid ELF returns error without corrupting process

## 8. FD, FS, and Pipe
File targets:
- `/home/khanh/Projects/miusOS/src/fs/file.rs`
- `/home/khanh/Projects/miusOS/src/fs/inode.rs`
- `/home/khanh/Projects/miusOS/src/fs/pipe.rs`

Tasks:
1. Implement fd table (`NOFILE`, lowest-free allocation).
2. Add `FileOps` and object wrappers.
3. Add inode read/write/create/unlink baseline operations.
4. Add path walk for absolute paths.
5. Add pipe ring buffer with blocking wake/sleep behavior.

Done when:
- file and pipe smoke tests pass
- EOF and broken-pipe semantics are verified

## 9. Operational Workflow Tooling
File targets:
- optional `/home/khanh/Projects/miusOS/tools/xtask/src/main.rs`
- optional `/home/khanh/Projects/miusOS/scripts/*.sh`

Tasks:
1. Add command routing for `build/run/debug/test-smoke`.
2. Standardize output so failures are easy to scan.
3. Ensure commands are deterministic and non-interactive.

Done when:
- same command sequence works on clean setup

## Mentor Review Checklist Per PR
- boundaries respected (no subsystem leakage)
- unsafe blocks explained with invariants
- tests/evidence included for changed behavior
- no skipped gate relevant to modified subsystem
