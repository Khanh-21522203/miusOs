# Verification Gates and Test Runbook

This file defines mandatory gates. If a gate fails, stop feature work and debug the failing gate first.

Rule:
- No stage advancement without a green gate.
- Every gate result must include command evidence and observed output summary.

## Gate Workflow (Debug Decision Diagram)

```text
Run gate test
  |
  +--> PASS
  |      |
  |      +--> record evidence -> proceed to next stage
  |
  `--> FAIL
         |
         +--> capture first failure signal (log/pc/scause)
         +--> run first-response debug commands
         +--> identify root cause category (boot/paging/trap/proc/fs)
         +--> fix minimal cause
         `--> rerun same gate from clean boot
```

## Standard Evidence Template
Use this template for each gate run:

```text
Gate: <A-F>
Date:
Commit/working state:
Command(s):
Observed output:
Pass/Fail:
If fail, first bad signal:
Root cause hypothesis:
Next action:
```

## Gate A: Boot Integrity

Goal:
- verify handoff from firmware to Rust entry with stable early console output.

Preconditions:
- kernel image builds
- `_start` symbol linked

Commands:
```bash
qemu-system-riscv64 -machine virt -nographic -bios default -kernel build/kernel.bin
```

Debug commands (if needed):
```bash
qemu-system-riscv64 -machine virt -nographic -bios default -kernel build/kernel.bin -S -s
gdb-multiarch build/kernel.elf \
  -ex "set arch riscv:rv64" \
  -ex "target remote :1234" \
  -ex "break _start" \
  -ex "continue" \
  -ex "info registers" \
  -ex "x/10i $pc"
```

Pass criteria:
- `_start` breakpoint is reachable
- execution reaches `rust_main`
- UART boot logs print deterministically

Typical failure signatures:
- no output at all
- immediate repeated trap/exception
- wrong `pc` after entry

First checks:
- stack pointer alignment
- linker entry symbol correctness
- UART MMIO address usage

## Gate B: Paging Integrity

Goal:
- ensure page table operations are correct and kernel survives SATP switch.

Preconditions:
- `map/unmap/translate` implemented
- satp write + sfence implemented

Required tests:
1. map one aligned page and confirm translation
2. unmap same page and confirm miss
3. double map returns `AlreadyMapped`
4. non-aligned input returns `NotAligned`

Pass criteria:
- all page API tests pass
- enabling paging does not trap-loop

Typical failure signatures:
- crash immediately after `satp` write
- stale translation behavior after remap

First checks:
- root page table physical address
- current instruction mapping existence
- `sfence.vma` execution placement

## Gate C: Trap and Syscall Integrity

Goal:
- validate end-to-end trap entry/dispatch/return behavior.

Preconditions:
- trap asm and `TrapFrame` exist
- syscall dispatch path exists

Required tests:
1. user `ecall` reaches kernel handler
2. `sepc` increments by instruction width (`+4` for `ecall`)
3. syscall return value appears in `a0`
4. timer interrupt path executes repeatedly without corruption

Pass criteria:
- user resumes correctly after syscall
- no register corruption across trap boundary

Typical failure signatures:
- stuck in trap-loop
- wrong user PC after return
- random register corruption symptoms

First checks:
- `TrapFrame` field order vs asm store/load order
- `stvec` value and alignment
- `scause/sepc/stval` values at first failure

## Gate D: Scheduler Integrity

Goal:
- verify fair runnable-task progress and safe context switching.

Preconditions:
- `Task`, `Context`, run queue and switch asm implemented

Required tests:
1. one runnable task remains stable over time
2. two runnable CPU-bound tasks both make progress
3. timer preemption triggers reschedule path

Pass criteria:
- no deadlock in scheduler loop
- no lock held across context switch
- valid task state transitions only

Typical failure signatures:
- kernel stalls in scheduler
- one task starves permanently
- lock-order deadlock under preemption

First checks:
- lock acquire/release trace around switch points
- task state before and after switch
- timer interrupt to scheduler handoff path

## Gate E: `exec` Integrity

Goal:
- ensure process image replacement is atomic and rollback-safe.

Preconditions:
- ELF loading and address-space replacement code implemented

Required tests:
1. valid ELF exec succeeds and PID stays unchanged
2. invalid ELF exec fails and old process image remains active
3. oversized argv fails safely without corruption

Pass criteria:
- two-phase commit behavior observed
- no frame leaks on failed exec paths

Typical failure signatures:
- process corrupted after failed exec
- PID changes unexpectedly
- kernel fault during argv copy

First checks:
- user pointer bounds and copy limits
- swap point location in control flow
- cleanup path on each error branch

## Gate F: FD/FS/Pipe Integrity

Goal:
- validate baseline I/O semantics for files and pipes.

Preconditions:
- fd table, inode operations, pipe implementation exist

Required tests:
1. create -> write -> read -> unlink file roundtrip
2. fd bounds checks and bad-fd error behavior
3. pipe roundtrip with two endpoints
4. EOF behavior when all writers close
5. broken-pipe behavior when readers close

Pass criteria:
- all required scenarios pass deterministically
- no leaked references after close operations

Typical failure signatures:
- deadlock on full/empty pipe
- data corruption in ring wrap-around
- incorrect link count behavior on unlink

First checks:
- pipe read/write index update logic
- inode link count transitions
- wake/sleep channel usage for blocked readers/writers

## Release Gate: Baseline Ready

Goal:
- verify repository is contributor-ready, not just locally functional.

Required commands:
```bash
cargo run -p xtask -- build
cargo run -p xtask -- run
cargo run -p xtask -- debug
cargo run -p xtask -- test-smoke
```

Pass criteria:
- all commands execute successfully from a clean workspace state
- smoke tests clearly report pass/fail
- troubleshooting notes exist for top failure classes

If release gate fails:
- do not start extension features
- fix baseline reproducibility first
