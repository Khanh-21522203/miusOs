# xv6-riscv in Rust: File System Basics

This chapter builds a small inode-based filesystem model similar to xv6.

## 1. Split on-disk vs in-memory types
On-disk types must be stable and packed with explicit layout.

```rust
#[repr(C)]
pub struct SuperBlock {
    pub size: u32,
    pub nblocks: u32,
    pub ninodes: u32,
    pub log_start: u32,
}

#[repr(C)]
pub struct Dinode {
    pub typ: u16,
    pub major: u16,
    pub minor: u16,
    pub nlink: u16,
    pub size: u32,
    pub addrs: [u32; 13],
}
```

In-memory inode can include locks and cache state.

## 2. Essential operations
- inode allocation and lookup
- block mapping (direct + indirect)
- read/write by file offset
- directory lookup and insertion

## 3. Path walk model
`/a/b/c`:
1. Start at root inode.
2. Lookup `a` in root directory.
3. Lookup `b` in inode `a`.
4. Lookup `c` in inode `b`.

Each step acquires/releases locks carefully.

## 4. Crash model for learning kernel
Start with simple write-through behavior or minimal logging.

Document limitation clearly:
- Not power-fail safe without journaling.

## 5. Verification targets
- Create file, write bytes, read back same bytes.
- Create directory and nested file path.
- Unlink decrements link count and frees blocks when needed.

## Checklist
- [ ] On-disk structs use fixed `#[repr(C)]` layout.
- [ ] Inode cache and locking model are defined.
- [ ] Path resolution works for absolute paths.
- [ ] Basic read/write/create/unlink flow is testable.

Next: [17-pipe.md](17-pipe.md)
