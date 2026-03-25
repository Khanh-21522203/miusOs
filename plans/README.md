# miusOS Implementation Planning Handbook

This `plans/` directory is the execution handbook for building the Rust xv6-style teaching OS described in `docs/`.

Audience:
- Primary: interns and junior engineers implementing the kernel.
- Secondary: mentors/reviewers validating safety, correctness, and sequencing.

Current repository state (as of this document):
- The repository currently contains curriculum docs under `docs/`.
- The kernel/workspace code does not exist yet.
- This planning set explains what to create, in what order, and how to verify each step.

## How to Read This Folder
Read in this exact order:
1. `plans/glossary.md`
2. `plans/implementation-roadmap.md`
3. `plans/intern-execution-playbook.md`
4. `plans/work-breakdown.md`
5. `plans/verification-gates.md`
6. `plans/risk-and-change-management.md`

Then use this cycle every work day:
1. Pick one stage task from `implementation-roadmap.md`.
2. Pull concrete file-level tasks from `work-breakdown.md`.
3. Run the matching gate from `verification-gates.md`.
4. Log progress blockers and risk notes in your task tracker.

## Architecture Context (System Diagram)

```text
Host Machine (Linux/WSL)
  |
  | build + run + debug
  v
+---------------------------------------------------------------+
| Rust Workspace (to be created)                                |
|                                                               |
|  root Cargo workspace                                         |
|   |- kernel/   (no_std kernel code + asm glue)               |
|   |- user/     (user binaries, syscall stubs)                |
|   |- xtask/    (developer command runner)                    |
|   `- scripts/  (optional helper shell scripts)               |
+---------------------------------------------------------------+
  |
  | ELF/BIN kernel image
  v
+---------------------------------------------------------------+
| QEMU virt machine                                              |
|   OpenSBI (M-mode firmware) -> S-mode kernel -> user tasks    |
+---------------------------------------------------------------+
```

## Documentation Map and Purpose
- `plans/glossary.md`
  - Vocabulary for boot, memory, traps, scheduling, filesystem.
  - Read first to remove terminology confusion.

- `plans/implementation-roadmap.md`
  - Stage-by-stage execution playbook.
  - Defines prerequisites, implementation steps, outputs, and exit criteria.

- `plans/work-breakdown.md`
  - Module ownership and detailed backlog by subsystem.
  - Includes target file paths and interface boundaries.

- `plans/intern-execution-playbook.md`
  - Day-by-day execution guide for interns.
  - Includes blocker escalation and reporting templates.

- `plans/verification-gates.md`
  - Hard quality gates that must pass before moving forward.
  - Includes command recipes, expected behavior, and first-response debugging.

- `plans/risk-and-change-management.md`
  - Rules for high-risk changes and mentor review checkpoints.
  - Keeps progress fast without accumulating unsafe or architectural debt.

## Intern Working Agreement
- Do not skip stage order.
- Do not merge code that fails a gate.
- Keep unsafe code tiny and documented with invariants.
- Preserve module boundaries (`arch`, `mm`, `trap`, `proc`, `fs`, `drivers`).
- Escalate blockers early with exact evidence (command + output + observed behavior).

## Evidence You Must Produce Per Stage
At minimum, keep these artifacts in your notes or PR description:
- what files changed
- what commands were run
- what passed/failed
- one-paragraph explanation of root cause for each failure
- next action if unresolved

## Definition of "Baseline Complete"
Baseline is complete when all are true:
- kernel boots in QEMU with deterministic logs
- traps and timer interrupts are stable
- user syscall path works end-to-end
- scheduler runs tasks without deadlock
- basic FS and pipe behavior pass smoke tests
- `xtask` provides repeatable `build/run/debug/test-smoke` workflows
