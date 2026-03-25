# Risk and Change Management Guide

This guide is for interns and mentors to keep implementation quality stable while moving quickly.

## Why This Exists
Kernel bring-up failures are expensive when caused by unmanaged risk.
This file defines what must be reviewed before merging sensitive changes.

## High-Risk Change Categories

### Category 1: Assembly and ABI layout changes
Examples:
- `start.S`, `trap.S`, `switch.S`
- changing `TrapFrame` or `Context` field order

Risk:
- silent register corruption
- hard-to-debug trap loops

Required controls:
- explicit layout review (`#[repr(C)]` correctness)
- gate reruns: A, C, D

### Category 2: Paging and address translation changes
Examples:
- `map/unmap/translate`
- SATP mode or root page table handling

Risk:
- immediate crash after paging enable
- stale mappings and random behavior

Required controls:
- gate reruns: B, C
- document invariant changes in PR notes

### Category 3: Locking and scheduler changes
Examples:
- run queue code
- lock ordering updates

Risk:
- deadlocks under timer preemption
- starvation and nondeterministic stalls

Required controls:
- gate reruns: D
- lock order notes in code comments/PR description

### Category 4: `exec` and user pointer validation changes
Examples:
- argv copy-in logic
- ELF segment load logic

Risk:
- process image corruption
- kernel faults from bad user pointers

Required controls:
- gate reruns: E
- failure-path audit (no partial commits)

### Category 5: FS/pipe reference and lifetime changes
Examples:
- fd close semantics
- inode link count logic
- pipe blocking and wakeup logic

Risk:
- leaked resources
- deadlocks or incorrect EOF behavior

Required controls:
- gate reruns: F
- deterministic repro test attached

## Invariant Logging Policy
For each subsystem change, the PR description must include:
1. changed invariants (if any)
2. unchanged invariants explicitly preserved
3. failure mode this change is expected to prevent

Template:

```text
Subsystem:
Changed invariant:
Preserved invariant:
Failure prevented:
Gate(s) rerun:
```

## Reviewer Assignment Policy
- At least one reviewer with low-level familiarity for:
  - asm files
  - trapframe/context layout changes
  - page-table logic
- At least one reviewer with subsystem ownership for:
  - `proc` changes
  - `fs`/`pipe` changes

## Merge Policy for Intern Branches
- merge only after relevant gates pass
- if gate coverage is missing, mark PR as blocked
- no "will fix in follow-up" for correctness regressions

## Rollback Strategy
If severe regression lands:
1. stop new feature merges
2. identify first bad commit
3. revert only the minimal offending change
4. rerun all affected gates
5. reopen fix with tighter invariant docs

## Expansion Policy After Baseline
You may start advanced features (COW fork, SMP, networking) only when:
- release gate is green
- baseline smoke tests are stable for multiple consecutive runs
- maintainer approves extension scope and acceptance tests
