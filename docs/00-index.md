# xv6-riscv in Rust: Curriculum Index

This series guides you from zero to a small teaching OS on RISC-V using Rust instead of C.

## Who this is for
- Junior engineers who know basic Rust syntax and Linux terminal usage.
- Anyone who wants to understand how kernels boot, manage memory, and run processes.

## Learning contract
- Build incrementally, verify each step, and avoid skipping milestones.
- Keep unsafe code small and documented.
- Prefer clear invariants over clever tricks.

## Full sequence
1. [01-overview.md](01-overview.md): Goals, architecture, and Rust kernel mindset.
2. [02-environment-setup.md](02-environment-setup.md): Install and verify Rust + QEMU + GDB toolchain.
3. [03-project-structure.md](03-project-structure.md): Create a Rust workspace and module boundaries.
4. [04-bootloader-explained.md](04-bootloader-explained.md): Boot flow from reset to `rust_main`.
5. [05-bootloader-assembly.md](05-bootloader-assembly.md): Startup assembly needed for Rust handoff.
6. [06-memory-layout.md](06-memory-layout.md): Sv39 memory model and linker symbols.
7. [07-kernel-entry.md](07-kernel-entry.md): Kernel init sequence.
8. [08-page-table.md](08-page-table.md): Paging theory for Sv39.
9. [09-page-table-code.md](09-page-table-code.md): Rust page-table implementation.
10. [10-traps-and-syscalls.md](10-traps-and-syscalls.md): Trap and syscall model.
11. [11-traps-code.md](11-traps-code.md): Trap handlers in asm + Rust.
12. [12-process-structure.md](12-process-structure.md): Task/process data structures.
13. [13-exec-syscall.md](13-exec-syscall.md): `exec` and user image replacement.
14. [14-scheduler.md](14-scheduler.md): Scheduling and context switch.
15. [15-file-descriptor.md](15-file-descriptor.md): File descriptor abstraction.
16. [16-filesystem.md](16-filesystem.md): Inode filesystem basics.
17. [17-pipe.md](17-pipe.md): Pipe IPC.
18. [18-build-and-test.md](18-build-and-test.md): Build, run, debug, smoke tests.
19. [19-common-pitfalls.md](19-common-pitfalls.md): Failure patterns and fast diagnosis.
20. [20-next-steps.md](20-next-steps.md): Advanced roadmap.

## Architecture map (high-level)
```
┌────────────────────────── Host Linux ──────────────────────────┐
│ cargo + rustc + llvm-tools + qemu-system-riscv64 + gdb         │
└─────────────────────────────────────────────────────────────────┘
                           │ build/run
                           ▼
┌──────────────────────── Rust Workspace ─────────────────────────┐
│ boot/ (startup asm + handoff)                                  │
│ kernel/ (mm, trap, proc, fs, drivers)                          │
│ user/ (simple user programs, ABI stubs)                        │
│ xtask/ (dev commands: build, run, debug)                       │
└─────────────────────────────────────────────────────────────────┘
                           │ ELF/bin
                           ▼
┌────────────────────────── QEMU virt ────────────────────────────┐
│ OpenSBI -> kernel entry -> scheduler -> user processes          │
└─────────────────────────────────────────────────────────────────┘
```

## Stage-by-stage deliverables
- Stage 1 (01-03): toolchain verified and repo scaffolded.
- Stage 2 (04-06): kernel reaches Rust entry with known memory layout.
- Stage 3 (07-11): traps and syscalls work; controlled user transitions.
- Stage 4 (12-14): process lifecycle and scheduler are functional.
- Stage 5 (15-17): basic Unix-like I/O abstractions and filesystem.
- Stage 6 (18-20): testability, troubleshooting, and extension planning.

## Minimum done definition
- Kernel boots on QEMU and prints startup log.
- Timer interrupt ticks and trap path is stable.
- One user program can call a syscall and exit.
- You can debug with GDB at instruction and Rust symbol levels.

## Study rhythm
- Read one chapter.
- Implement exactly that chapter’s checklist.
- Run smoke test before moving on.

Next: [01-overview.md](01-overview.md)
