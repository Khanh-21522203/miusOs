# xv6-riscv in Rust: `exec` System Call

This chapter implements `exec`: replace current process image while keeping the same PID.

## 1. `exec` behavior
Before:
- PID is stable.
- old user text/data/stack are active.

After `exec(path, argv)` succeeds:
- PID unchanged.
- Address space replaced by new program.
- User PC jumps to new entry point.
- User stack rebuilt with new args.

## 2. Safe Rust-oriented flow
1. Validate and copy `path`/`argv` from user memory.
2. Read ELF file through VFS.
3. Validate ELF headers.
4. Build new page table and map segments.
5. Build new user stack and argument block.
6. Swap current task memory context atomically.
7. Free old address space.

## 3. Function shape
```rust
pub fn exec(path: &UserCStr, argv: &UserArgv) -> Result<(), ExecError> {
    let kpath = copy_path_from_user(path)?;
    let kargv = copy_argv_from_user(argv)?;

    let image = fs::load_elf(&kpath)?;
    let (new_pt, entry, user_sp) = loader::build_user_image(&image, &kargv)?;

    proc::replace_current_address_space(new_pt, entry, user_sp)?;
    Ok(())
}
```

## 4. Security and correctness checks
- Segment virtual addresses must be canonical and page-aligned.
- Segment size in file must not exceed in-memory bounds.
- User stack guard page should remain unmapped.
- Argument copying must enforce max total size.

## 5. Failure behavior
If any step fails:
- Keep current process image unchanged.
- Return error code in syscall return register.

No partial commit.

## 6. Verification ideas
- `exec` a valid binary and confirm PID unchanged.
- `exec` invalid ELF and confirm process continues old image.
- Large argv should fail cleanly without memory corruption.

## Checklist
- [ ] `exec` uses two-phase commit (build then swap).
- [ ] User pointers are validated and copied into kernel buffers.
- [ ] ELF validation is strict.
- [ ] Failure paths do not leak frames or corrupt current process.

Next: [14-scheduler.md](14-scheduler.md)
