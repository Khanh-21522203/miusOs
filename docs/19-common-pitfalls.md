# xv6-riscv in Rust: Common Pitfalls and Troubleshooting

Use this chapter as a fast diagnosis guide during bring-up.

## 1. Boot prints nothing
Likely causes:
- stack pointer not initialized
- wrong kernel entry symbol
- UART MMIO mapping wrong

Checks:
- break at `_start`
- inspect `sp`, `pc`
- inspect UART base address mapping

## 2. Immediate trap loop
Likely causes:
- `stvec` points to wrong address
- trapframe layout mismatch between Rust and asm
- invalid `sstatus`/`sepc` on return

Checks:
- print `scause`, `sepc`, `stval`
- single-step trap return path

## 3. Paging crashes after enable
Likely causes:
- missing kernel text/data mapping
- wrong `satp` mode bits
- stale TLB entries (missing `sfence.vma`)

Checks:
- verify root page table physical address
- verify mapped ranges include current PC

## 4. Rust-specific unsafe bugs
Patterns:
- aliasing mutable references to same memory
- invalid pointer cast lifetime assumptions
- non-volatile access to MMIO registers

Fix pattern:
- wrap MMIO in explicit volatile helpers
- keep unsafe blocks tiny and documented
- add debug assertions on alignment and ranges

## 5. Scheduler deadlocks
Likely causes:
- lock held across context switch
- invalid lock ordering between task/global queues

Checks:
- trace lock acquire/release points
- add lock-order assertions in debug builds

## 6. `exec` corrupts process
Likely causes:
- partial address-space swap on failure
- argument copy overflow

Checks:
- two-phase commit: build new image first, then swap
- enforce max argv size and count

## 7. Filesystem corruption in tests
Likely causes:
- inode/block write ordering not respected
- missing cache flush boundaries

Checks:
- run deterministic file create/write/read/delete sequence
- verify inode link count transitions

## 8. Triage checklist
When anything fails:
1. identify first bad instruction/PC
2. inspect trap CSRs
3. inspect current task state
4. inspect page-table mapping for fault address
5. reproduce with minimal test case

## Checklist
- [ ] I can quickly inspect trap CSRs in GDB.
- [ ] I know top failure modes for boot, traps, paging, and scheduler.
- [ ] I enforce small, reviewed unsafe blocks.
- [ ] I use deterministic repro tests before refactoring.

Next: [20-next-steps.md](20-next-steps.md)
