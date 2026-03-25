# xv6-riscv in Rust: Next Steps

You now have a complete baseline curriculum for a Rust teaching kernel. Use this roadmap to extend it without losing stability.

## 1. Stabilize baseline first
Before adding features, lock in:
- repeatable build/run/test scripts
- deterministic boot logs
- core syscall/process/fs smoke tests

## 2. High-value extensions
1. Copy-on-write `fork`.
2. Better VM (`mmap`, lazy allocation).
3. VirtIO block/network drivers.
4. SMP support (multi-hart scheduling).
5. Journaling or write-ahead logging for filesystem.

## 3. Suggested order
```text
Baseline stable
  -> COW fork
  -> VirtIO block + stronger FS tests
  -> SMP bring-up
  -> Networking
  -> Performance tuning
```

## 4. Performance and observability
Add lightweight counters:
- context switches
- syscall counts/latency
- page faults
- block I/O operations

Expose via debug syscall or kernel console command.

## 5. Hardening ideas
- stronger user pointer validation
- kernel stack guard checks
- strict permission auditing for mappings
- fuzzing syscall argument decoders

## 6. Recommended reading path
- RISC-V privileged spec sections on traps and virtual memory.
- MIT xv6 book for design comparisons.
- Rust unsafe code guidelines for low-level patterns.

## Final checklist
- [ ] Baseline OS boots and runs basic user programs.
- [ ] Trap/syscall/process/fs/pipe paths are test-covered.
- [ ] Unsafe boundaries are documented and reviewed.
- [ ] Next milestone is chosen with explicit acceptance tests.

End of series.
