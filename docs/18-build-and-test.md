# xv6-riscv in Rust: Build, Run, and Test

This chapter defines the daily workflow for fast kernel iteration.

## 1. Build commands
Debug build:
```bash
cargo build -p kernel --target riscv64gc-unknown-none-elf
```

Release build:
```bash
cargo build -p kernel --release --target riscv64gc-unknown-none-elf
```

Create raw image (example):
```bash
cargo objcopy -p kernel --target riscv64gc-unknown-none-elf \
  -- -O binary build/kernel.bin
```

## 2. Run on QEMU
```bash
qemu-system-riscv64 \
  -machine virt -nographic -bios default \
  -kernel build/kernel.bin
```

Expected early output:
- boot banner
- memory init complete
- trap init complete
- scheduler started

## 3. Debug with GDB
Start QEMU paused:
```bash
qemu-system-riscv64 -machine virt -nographic -bios default \
  -kernel build/kernel.bin -S -s
```

Attach:
```bash
gdb-multiarch build/kernel.elf \
  -ex "set arch riscv:rv64" \
  -ex "target remote :1234"
```

## 4. Smoke tests you should automate
- Boot reaches scheduler.
- Timer interrupt count increases.
- User `getpid` syscall returns non-zero.
- `exec` launches init program.
- Pipe read/write roundtrip works.

## 5. Suggested `xtask` interface
```bash
cargo run -p xtask -- build
cargo run -p xtask -- run
cargo run -p xtask -- debug
cargo run -p xtask -- test-smoke
```

## 6. Regression workflow
For every change:
1. build
2. run
3. smoke test
4. debug only if failed

Keep loops short.

## Checklist
- [ ] Build and image generation commands are scripted.
- [ ] Run and debug flows are repeatable.
- [ ] Smoke tests cover boot/trap/syscall/process/fs basics.
- [ ] I can reproduce failures quickly with one command.

Next: [19-common-pitfalls.md](19-common-pitfalls.md)
