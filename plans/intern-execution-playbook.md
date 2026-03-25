# Intern Execution Playbook

This playbook explains exactly how to work day-to-day while implementing the roadmap.

## Daily Start Checklist (15 minutes)
1. Open `plans/implementation-roadmap.md` and confirm current stage.
2. Open `plans/work-breakdown.md` and select 1 to 3 tasks only.
3. Write expected outcomes for the day in your task notes.
4. Confirm which verification gate(s) must pass before day end.

## Daily End Checklist (20 minutes)
1. Run relevant gate tests.
2. Record command evidence and pass/fail results.
3. Write blockers and root-cause hypotheses.
4. List first action for next day.

## Work Chunk Strategy
Use 90-minute chunks:
- Chunk 1: implementation
- Chunk 2: implementation or debugging
- Chunk 3: verification and documentation

If blocked for more than 45 minutes:
- stop random experimentation
- gather concrete evidence
- follow blocker escalation template below

## Blocker Escalation Template

```text
Stage:
Task:
Expected behavior:
Actual behavior:
First failure signal:
Commands already tried:
Hypothesis:
What help is needed:
```

## Stage-by-Stage Execution Details

## Stage 1 Daily Breakdown (Toolchain + Workspace)
Day goals:
- produce compiling workspace skeleton

Suggested sequence:
1. verify host tools
2. configure Rust target
3. create workspace manifests
4. scaffold root module folders under `src/`
5. run first cross-build

Evidence required:
- output of `cargo build --target riscv64gc-unknown-none-elf`
- list of created top-level files and folders

Common blockers:
- linker mismatch
- target missing

## Stage 2 Daily Breakdown (Boot)
Day goals:
- reach deterministic boot output

Suggested sequence:
1. add `_start` and stack setup
2. wire `rust_main`
3. add UART output
4. add panic and `.bss` clear paths
5. verify via QEMU and GDB breakpoint flow

Evidence required:
- `_start` breakpoint reached
- boot log lines captured

Common blockers:
- wrong entry symbol
- silent boot due to bad UART mapping

## Stage 3 Daily Breakdown (MM + Traps)
Day goals:
- stable paging and trap path

Suggested sequence:
1. complete typed address wrappers
2. implement frame allocator
3. implement page table API
4. switch SATP and fence TLB
5. wire trap entry/exit + dispatcher
6. verify syscall roundtrip

Evidence required:
- map/unmap/translate test results
- syscall return evidence in `a0`

Common blockers:
- trapframe mismatch
- trap-loop after SATP switch

## Stage 4 Daily Breakdown (Proc + Scheduler + Exec)
Day goals:
- task switching and process image replacement

Suggested sequence:
1. implement task states and PID allocator
2. implement round-robin run loop
3. implement context switch asm
4. wire timer-driven preemption
5. implement `exec` two-phase commit

Evidence required:
- two tasks progress over time
- valid/invalid exec behavior evidence

Common blockers:
- lock ordering deadlocks
- partial exec commit corruption

## Stage 5 Daily Breakdown (FD + FS + Pipe)
Day goals:
- baseline file and pipe semantics

Suggested sequence:
1. implement fd table and checks
2. implement inode/path operations
3. implement pipe ring buffer
4. wire syscall layer and object refs
5. run deterministic I/O smoke tests

Evidence required:
- file roundtrip pass
- pipe EOF/broken-pipe pass

Common blockers:
- ring index wrap bug
- leaked refs on close

## Stage 6 Daily Breakdown (Hardening + Ops)
Day goals:
- make contributor workflow repeatable

Suggested sequence:
1. implement `xtask` command interface
2. automate smoke test command
3. improve failure messages and logs
4. document first-response debugging flow

Evidence required:
- `xtask build/run/debug/test-smoke` all pass

Common blockers:
- flaky timing-based tests
- unclear pass/fail test output

## Mentor Sync Cadence
- Daily async update:
  - current stage
  - tasks completed
  - gate status
  - blockers
- Weekly synchronous review:
  - architecture and invariant review
  - risk register updates

## Communication Quality Rules
When reporting progress, always include:
- exact file path changed
- exact command run
- exact observed behavior
- what was expected

Avoid vague updates like:
- "it still fails"
- "something wrong with memory"

Use specific updates like:
- "After enabling SATP in `src/mm/page_table.rs`, first fault is `scause=...` at `sepc=...`."

## Personal Quality Bar
Before marking any task done, ask:
1. Is behavior verified or only compiled?
2. Are invariants documented where unsafe exists?
3. Could another intern rerun the same steps deterministically?
