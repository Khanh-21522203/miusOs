# xv6-riscv in Rust: Page Table Implementation

This chapter turns Sv39 theory into Rust code with a safe public API.

## 1. Design goal
Expose safe mapping/unmapping functions while isolating raw pointer walks in `unsafe` internals.

## 2. Core types
```rust
pub type Result<T> = core::result::Result<T, VmError>;

pub struct PageTable {
    root_ppn: usize,
}

#[derive(Debug)]
pub enum VmError {
    NotAligned,
    AlreadyMapped,
    NotMapped,
    OutOfMemory,
}
```

## 3. Public API
```rust
impl PageTable {
    pub fn map(&mut self, va: VirtAddr, pa: PhysAddr, flags: PteFlags) -> Result<()> {
        // validate alignment and flags
        // walk/create levels
        // write leaf PTE
        Ok(())
    }

    pub fn unmap(&mut self, va: VirtAddr) -> Result<PhysAddr> {
        // walk to leaf and clear it
        Ok(PhysAddr(0))
    }

    pub fn translate(&self, va: VirtAddr) -> Option<PhysAddr> {
        None
    }
}
```

## 4. Frame allocator contract
Page-table creation depends on frame allocation:
```rust
pub trait FrameAlloc {
    fn alloc(&mut self) -> Option<PhysAddr>;
    fn free(&mut self, pa: PhysAddr);
}
```

Keep allocator simple first (free-list of 4 KiB frames).

## 5. Unsafe walk boundary
A common pattern:
- `walk_create(vpn)` returns mutable reference to a PTE.
- Only this function dereferences raw table pointers.
- Caller never touches raw PTE addresses directly.

## 6. Enable paging
After building kernel table:
```rust
pub fn activate_kernel_table(root_ppn: usize) {
    arch::riscv64::write_satp_sv39(root_ppn);
    arch::riscv64::sfence_vma_all();
}
```

## 7. Verification scenarios
- Map one page, read back translation.
- Unmap page and confirm translation fails.
- Double map should return `AlreadyMapped`.
- Non-aligned inputs should return `NotAligned`.

## Checklist
- [ ] `map`, `unmap`, and `translate` APIs exist.
- [ ] Alignment and duplicate checks are enforced.
- [ ] `unsafe` is localized to page-table walk internals.
- [ ] Paging activation writes `satp` and fences TLB.

Next: [10-traps-and-syscalls.md](10-traps-and-syscalls.md)
