# xv6-riscv in Rust: File Descriptors and I/O

This chapter defines a Unix-like file descriptor layer in Rust.

## 1. Core idea
A file descriptor is an index into a per-process table referencing an open object (inode, pipe, device).

## 2. Rust abstraction
```rust
pub trait FileOps: Send + Sync {
    fn read(&self, dst: &mut [u8]) -> Result<usize, IoError>;
    fn write(&self, src: &[u8]) -> Result<usize, IoError>;
    fn close(&self) -> Result<(), IoError> { Ok(()) }
}

pub enum FileRef {
    Inode(alloc::sync::Arc<dyn FileOps>),
    Pipe(alloc::sync::Arc<dyn FileOps>),
    Device(alloc::sync::Arc<dyn FileOps>),
}
```

## 3. Per-process FD table
```rust
pub const NOFILE: usize = 32;

pub struct FdTable {
    slots: [Option<FileRef>; NOFILE],
}
```

Rules:
- `open` allocates lowest free fd.
- `dup` increases shared reference count.
- `close` drops one reference.

## 4. Syscall boundary checks
- Validate fd range before access.
- Validate user buffers for read/write.
- Return proper error codes for bad fd and permissions.

## 5. Concurrency notes
Use lock around FD table mutation. File objects themselves may have internal locks.

## Checklist
- [ ] Trait-based I/O abstraction exists.
- [ ] `open/close/read/write/dup` semantics are defined.
- [ ] Per-process FD table enforces bounds and ownership.
- [ ] Syscall layer validates user pointers and fd values.

Next: [16-filesystem.md](16-filesystem.md)
