# Intern Glossary (Kernel Bring-up)

Use this glossary while reading `docs/` and implementing the roadmap.

## Boot and CPU Terms
- OpenSBI:
  - firmware that runs before your kernel on QEMU `virt`.
  - hands control to your S-mode kernel entry.

- Hart:
  - one RISC-V hardware thread (similar to a CPU core context).
  - `hart_id` is passed in register `a0` at kernel entry.

- `_start`:
  - first instruction symbol of your kernel entry assembly.
  - must setup stack and call `rust_main`.

- `rust_main`:
  - first Rust function after assembly setup.
  - performs deterministic kernel initialization.

## ABI and Register Terms
- ABI:
  - function calling contract (which registers carry args/returns).

- `a0-a7`:
  - argument/return registers in RISC-V calling convention.
  - syscall convention typically uses `a7` as syscall id.

- `sepc`:
  - CSR holding trap return program counter.

- `scause`:
  - CSR describing why trap happened.

- `stval`:
  - CSR with extra fault data (often bad address).

- `stvec`:
  - CSR containing trap entry address.

## Memory Terms
- Sv39:
  - RISC-V 39-bit virtual memory paging mode.
  - 3-level page table walk (VPN[2], VPN[1], VPN[0]).

- Page:
  - fixed-size memory block (4 KiB in this project).

- PTE:
  - page table entry containing PPN and permission bits.

- PPN:
  - physical page number (page-aligned physical frame index).

- TLB:
  - translation cache inside CPU.
  - must be fenced (`sfence.vma`) after mapping changes.

- SATP:
  - CSR that selects current page table root and mode.

## Trap and Syscall Terms
- Trap:
  - control transfer from user/kernel code into trap handler due to exception/interrupt.

- Exception:
  - synchronous trap caused by current instruction stream (for example `ecall`, page fault).

- Interrupt:
  - asynchronous trap from external/timer events.

- TrapFrame:
  - kernel-owned saved register snapshot used during trap handling.

## Process and Scheduling Terms
- Task/Process:
  - runnable execution context with memory, registers, and open files.

- Context switch:
  - saving one task context and restoring another.

- Runnable:
  - task state meaning scheduler can run it now.

- Zombie:
  - task that exited but has not been reaped yet.

- Preemption:
  - scheduler interrupting a running task (usually via timer interrupt).

## Filesystem and I/O Terms
- File descriptor (FD):
  - integer index into per-process table of open objects.

- Inode:
  - filesystem metadata object describing a file/directory.

- Path walk:
  - resolving path components (`/a/b/c`) through directory inodes.

- Pipe:
  - in-kernel byte stream endpoint pair for IPC.

- EOF:
  - read returns `0` because no writers remain and no more bytes exist.

## Rust Safety Terms
- `unsafe` block:
  - region where compiler cannot enforce all memory rules.
  - must be minimal, justified, and documented with invariants.

- Invariant:
  - condition that must always hold for code to be correct.

- `#[repr(C)]`:
  - struct layout rule required when sharing with assembly or on-disk formats.
