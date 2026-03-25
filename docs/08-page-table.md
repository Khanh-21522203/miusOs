# xv6-riscv in Rust: Virtual Memory and Paging (Theory)

This chapter explains Sv39 paging before implementation.

## 1. Why paging exists
- Isolate processes from each other.
- Isolate user space from kernel space.
- Support sparse virtual address spaces.
- Enable permission checks (R/W/X/U bits).

## 2. Sv39 translation model
Virtual address layout:
```text
| VPN[2] (9) | VPN[1] (9) | VPN[0] (9) | offset (12) |
```

Each table page has 512 entries. Translation walks L2 -> L1 -> L0.

## 3. Page table entry (PTE)
A valid leaf entry includes:
- Physical Page Number (PPN).
- Permission bits: `R`, `W`, `X`, `U`, `A`, `D`.

Rust bitflags sketch:
```rust
bitflags::bitflags! {
    pub struct PteFlags: u64 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}
```

## 4. Typed address APIs
Avoid mixing physical/virtual addresses.

```rust
pub struct VirtAddr(pub usize);
pub struct PhysAddr(pub usize);
pub struct PageNum(pub usize);
```

This catches many bugs before runtime.

## 5. Kernel mapping policy
Common teaching setup:
- Identity-map or direct-map kernel RAM region.
- Map UART and other MMIO regions.
- Map trampoline at a fixed high virtual address.
- Map kernel text read-exec and data read-write.

## 6. TLB behavior
After changing mappings, execute `sfence.vma` on relevant harts. If you skip this, CPU may use stale translations.

## 7. Common mental model mistakes
- Confusing physical address with physical page number.
- Forgetting page alignment.
- Setting `W` without `R` (invalid in many setups).
- Forgetting to clear `U` on kernel mappings.

## Checklist
- [ ] I can explain Sv39 walk from VA to PA.
- [ ] I know the meaning of each core PTE flag.
- [ ] I understand why typed wrappers reduce paging bugs.
- [ ] I know when to issue `sfence.vma`.

Next: [09-page-table-code.md](09-page-table-code.md)
