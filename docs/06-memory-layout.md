# xv6-riscv in Rust: Memory Layout

This chapter defines the memory model you will use for paging and protection.

## 1. Sv39 virtual addressing recap
RISC-V Sv39 splits virtual address:
- VPN[2] (9 bits)
- VPN[1] (9 bits)
- VPN[0] (9 bits)
- page offset (12 bits)

Page size: `4096` bytes.

## 2. Teaching-kernel memory map
```text
Virtual Address Space (Sv39 canonical form)

0xFFFF_FFFF_FFFF_FFFF
┌──────────────────────────────────────────────┐
│ Kernel high mappings (optional design)       │
├──────────────────────────────────────────────┤
│ Trampoline / trap page                        │
├──────────────────────────────────────────────┤
│ Kernel text / rodata / data / bss             │
│ Direct map of RAM (design dependent)          │
├──────────────────────────────────────────────┤
│ Guard pages for kernel stacks                 │
├──────────────────────────────────────────────┤
│ User stack (grows down)                       │
│ User heap  (grows up)                         │
│ User text/data                                │
└──────────────────────────────────────────────┘
0x0000_0000_0000_0000
```

## 3. Linker symbols and Rust access
Your linker script exports section boundaries:
- `stext`, `etext`
- `srodata`, `erodata`
- `sdata`, `edata`
- `sbss`, `ebss`
- `kernel_end`

Rust declarations:
```rust
unsafe extern "C" {
    static stext: u8;
    static etext: u8;
    static sbss: u8;
    static ebss: u8;
    static kernel_end: u8;
}
```

## 4. Typed address wrappers
Avoid using raw `usize` everywhere. Use explicit address types:
```rust
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub usize);

pub const PAGE_SIZE: usize = 4096;

pub const fn page_round_down(x: usize) -> usize {
    x & !(PAGE_SIZE - 1)
}
```

## 5. Kernel memory safety invariants
- Never map user pages writable+executable.
- Keep kernel text executable but read-only after setup.
- Enforce guard pages around kernel stacks.
- Flush TLB after page-table changes (`sfence.vma`).

## 6. Bring-up checks
- Verify `.bss` was cleared.
- Verify kernel section addresses match linker expectations.
- Verify first page-table mappings include UART/MMIO and kernel text.

## Checklist
- [ ] I can draw Sv39 VA breakdown from memory.
- [ ] I can explain each linker symbol used at boot.
- [ ] I have typed wrappers for physical/virtual addresses.
- [ ] I know the core mapping invariants to enforce.

Next: [07-kernel-entry.md](07-kernel-entry.md)
