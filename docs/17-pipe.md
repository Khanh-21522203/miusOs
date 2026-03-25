# xv6-riscv in Rust: Pipes and IPC

This chapter implements anonymous pipes for process-to-process byte streams.

## 1. Pipe model
- Bounded ring buffer.
- One read end and one write end (conceptually).
- Reader blocks when empty.
- Writer blocks when full.
- Reader sees EOF when all writers are closed.

## 2. Rust structure sketch
```rust
pub const PIPE_SIZE: usize = 512;

pub struct Pipe {
    buf: [u8; PIPE_SIZE],
    r: usize,
    w: usize,
    read_open: bool,
    write_open: bool,
}
```

Wrap with lock + wait queues/sleep channels.

## 3. Read semantics
- If buffer has data: return immediately.
- If empty and write end open: sleep.
- If empty and write end closed: return `0` (EOF).

## 4. Write semantics
- If buffer full and read end open: sleep.
- If read end closed: return broken-pipe error.
- Otherwise copy bytes and wake readers.

## 5. Integration with file descriptors
`pipe()` syscall:
- allocates one `Pipe` object
- returns two FDs (`read_fd`, `write_fd`) in caller table
- both FDs reference same underlying pipe state

## Checklist
- [ ] Ring-buffer indices wrap correctly.
- [ ] Blocking/wakeup behavior is correct for empty/full buffer.
- [ ] EOF and broken-pipe behavior are correct.
- [ ] `pipe()` installs two FDs pointing to one pipe object.

Next: [18-build-and-test.md](18-build-and-test.md)
