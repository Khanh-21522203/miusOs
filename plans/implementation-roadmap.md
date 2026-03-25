# Detailed Implementation Roadmap

This roadmap is intentionally detailed so an intern can execute without guessing hidden assumptions.

Scope:
- Build a teaching kernel based on `docs/00-index.md` through `docs/20-next-steps.md`.
- Establish a stable baseline before advanced features.

Primary principle:
- Every stage has explicit entry conditions, tasks, outputs, and exit tests.
- No stage may be skipped.

## Runtime Lifecycle Diagram (Sequence)

```text
Power on
  -> OpenSBI firmware (M-mode)
    -> jump to kernel entry `_start` (S-mode)
      -> setup stack per hart
      -> clear .bss
      -> call `rust_main(hart_id, dtb_pa)`
        -> init UART
        -> init traps
        -> init frame allocator + page tables
        -> enable paging
        -> init process table
        -> create first user process
        -> run scheduler loop
          -> user ecall/interrupt
            -> trap entry asm
            -> Rust trap handler/syscall dispatch
            -> trap return asm (sret)
```

## Stage 0: Preparation and Expectations

Linked curriculum: 00, 01

Objective:
- Align expectations and shared vocabulary before coding.

Entry requirements:
- Read `docs/00-index.md` and `docs/01-overview.md` fully.
- Read `plans/glossary.md` fully.

Tasks:
1. Write a short note (internal) explaining:
   - why assembly is still required
   - where unsafe Rust is expected
2. Confirm you can explain the boot chain: OpenSBI -> `_start` -> `rust_main`.

Outputs:
- written summary note (can be in task tracker)

Exit criteria:
- mentor confirms conceptual understanding in review chat.

Common failure mode:
- jumping to coding without understanding ABI and trap invariants.

---

## Stage 1: Toolchain and Workspace Scaffold

Linked curriculum: 02, 03

Objective:
- Produce a reproducible Rust cross-build and a clean module-based layout.

Entry requirements:
- Stage 0 completed.

Implementation tasks:
1. Install/verify host tools (`qemu-system-riscv64`, `gdb-multiarch`, `lld`).
2. Install Rust nightly and required components/target.
3. Create workspace files:
   - `/home/khanh/Projects/miusOS/Cargo.toml`
   - `/home/khanh/Projects/miusOS/rust-toolchain.toml`
   - `/home/khanh/Projects/miusOS/.cargo/config.toml`
4. Create root module directories:
   - `/home/khanh/Projects/miusOS/src/arch/`
   - `/home/khanh/Projects/miusOS/src/mm/`
   - `/home/khanh/Projects/miusOS/src/trap/`
   - `/home/khanh/Projects/miusOS/src/proc/`
   - `/home/khanh/Projects/miusOS/src/fs/`
   - `/home/khanh/Projects/miusOS/src/drivers/`
5. Add linker script at:
   - `/home/khanh/Projects/miusOS/linker.ld`

Expected outputs:
- cross-target build command succeeds for root crate.
- module directories resolve and compile from `src/lib.rs`.

Verification commands:
```bash
cargo build --target riscv64gc-unknown-none-elf
rustup target list --installed | rg riscv64gc-unknown-none-elf
qemu-system-riscv64 --version
```

Exit criteria:
- Stage 1 gate in `plans/verification-gates.md` passes.

Common failure modes:
- host linker accidentally used instead of `rust-lld`.
- missing `rust-src` causing `core` build failures.

---

## Stage 2: Boot Path and Early Kernel Init

Linked curriculum: 04, 05, 06, 07

Objective:
- Reach a deterministic `rust_main` and print early boot logs.

Entry requirements:
- Stage 1 complete.

Implementation tasks:
1. Add startup assembly:
   - `/home/khanh/Projects/miusOS/src/arch/riscv64/start.S`
2. Define `_start` responsibilities:
   - preserve `a0`/`a1`
   - setup per-hart stack
   - call `rust_main`
   - park forever on unexpected return
3. Add early Rust runtime:
   - no_std/no_main kernel entry
   - panic handler with UART output
   - `.bss` clear helper
4. Add UART driver:
   - `/home/khanh/Projects/miusOS/src/drivers/uart.rs`
5. Implement deterministic init order in `rust_main`.

Expected outputs:
- QEMU boots kernel and prints startup lines every run.
- GDB can break at `_start` and step to `rust_main`.

Verification commands:
```bash
qemu-system-riscv64 -machine virt -nographic -bios default -kernel build/kernel.bin
qemu-system-riscv64 -machine virt -nographic -bios default -kernel build/kernel.bin -S -s
gdb-multiarch build/kernel.elf -ex "set arch riscv:rv64" -ex "target remote :1234"
```

Exit criteria:
- Boot integrity gate passes.

Common failure modes:
- stack alignment incorrect at Rust call boundary.
- wrong symbol name or linker section for `_start`.
- no UART output because MMIO base mapping is wrong.

---

## Stage 3: Virtual Memory and Trap Foundation

Linked curriculum: 08, 09, 10, 11

Objective:
- Implement Sv39 mappings and stable trap/syscall boundary.

Entry requirements:
- Stage 2 complete with stable boot logs.

Implementation tasks:
1. Implement typed memory primitives:
   - `PhysAddr`, `VirtAddr`, page alignment helpers
2. Implement frame allocator API and initial allocator.
3. Implement page table module with API:
   - `map`, `unmap`, `translate`
4. Add SATP + TLB operations:
   - `write_satp_sv39`
   - `sfence_vma_all`
5. Define trapframe with exact asm-compatible layout.
6. Implement trap entry/exit asm and Rust trap dispatcher.
7. Implement first syscall table entries (`getpid`, `write`, `exit` stubs).

Expected outputs:
- kernel can enable paging and continue execution.
- user `ecall` enters and returns through trap path.

Verification focus:
- no trap-loop after enabling paging
- `sepc` increments by 4 on syscall path

Exit criteria:
- Paging + trap gate passes.

Common failure modes:
- mismatched trapframe layout between Rust and asm.
- mapping current code address incorrectly before SATP switch.
- forgetting TLB fence after mapping changes.

---

## Stage 4: Process Lifecycle and Scheduling

Linked curriculum: 12, 13, 14

Objective:
- Run user tasks with correct state transitions and context switches.

Entry requirements:
- Stage 3 complete.

Implementation tasks:
1. Create process core structures:
   - `Task`, `TaskState`, `Context`
2. Implement PID allocator and task table locking rules.
3. Add first runnable process (`init`) creation path.
4. Implement round-robin selection.
5. Implement context switch asm routine.
6. Wire timer interrupt preemption and yield path.
7. Implement `exec` two-phase commit flow.

Expected outputs:
- scheduler loops without deadlock.
- tasks run and switch.
- `exec` preserves PID on success.

Exit criteria:
- Scheduler + exec gates pass.

Common failure modes:
- lock held across context switch.
- partial `exec` commit corrupting current process.
- invalid state transitions not rejected.

---

## Stage 5: File Descriptors, Filesystem, and Pipes

Linked curriculum: 15, 16, 17

Objective:
- Provide baseline Unix-like I/O semantics.

Entry requirements:
- Stage 4 complete.

Implementation tasks:
1. Implement FD table with bounds and ownership checks.
2. Implement `FileOps` abstraction and object types.
3. Implement inode and path walk basics.
4. Implement read/write/create/unlink baseline flows.
5. Implement anonymous pipe with ring buffer and blocking semantics.
6. Integrate `pipe()` syscall with two returned FDs.

Expected outputs:
- file read/write roundtrip passes.
- pipe read/write roundtrip passes.
- EOF and broken-pipe behavior are correct.

Exit criteria:
- FS/FD/pipe gate passes.

Common failure modes:
- incorrect lock handling in inode/path walk.
- ring index wrap bugs in pipe buffer.
- incorrect close semantics causing leaked references.

---

## Stage 6: Developer Workflow and Baseline Hardening

Linked curriculum: 18, 19, 20

Objective:
- Make build/run/debug/test repeatable for future contributors.

Entry requirements:
- Stage 5 complete.

Implementation tasks:
1. Implement repeatable run commands in scripts or a helper crate:
   - `build`, `run`, `debug`, `test-smoke`
2. Add deterministic smoke tests for:
   - boot
   - timer trap
   - syscall path
   - `exec`
   - fs and pipe roundtrip
3. Document first-response troubleshooting steps for failures.
4. Stabilize logs for easier issue comparison.

Expected outputs:
- one-command smoke test run.
- concise failure reproduction guidance.

Exit criteria:
- release gate passes.

Common failure modes:
- tests rely on nondeterministic timing.
- missing clear pass/fail signals in logs.

---

## Suggested Time Budget (Intern)
- Stage 0: 0.5 day
- Stage 1: 1 to 2 days
- Stage 2: 2 to 4 days
- Stage 3: 4 to 7 days
- Stage 4: 4 to 7 days
- Stage 5: 5 to 10 days
- Stage 6: 2 to 4 days

Total baseline range:
- approximately 3 to 5 weeks full-time depending on debugging time.

## Stage Handoff Template
Use this template at every stage completion:

```text
Stage: <name>
Date:
Files changed:
Commands run:
Gate results:
Known limitations:
Next stage risks:
```
