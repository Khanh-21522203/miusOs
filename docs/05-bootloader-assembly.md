# xv6-riscv in Rust: Bootloader Assembly Deep Dive

This chapter is a practical assembly appendix for Rust kernel startup.

## 1. Minimal startup responsibilities
- Select per-hart stack.
- Preserve boot arguments for Rust (`a0`, `a1`).
- Jump into `rust_main`.
- Park forever if `rust_main` returns unexpectedly.

## 2. Example startup file
`kernel/src/arch/riscv64/start.S`:
```asm
.section .text.entry
.globl _start
_start:
  # a0 = hart_id, a1 = dtb_pa from firmware

  la   t0, boot_stacks_top
  li   t1, 4096
  addi t2, a0, 1
  mul  t1, t1, t2
  sub  sp, t0, t1

  call rust_main

1:
  wfi
  j 1b
```

## 3. Exporting symbols to Rust
In Rust, declare external symbols when needed:
```rust
unsafe extern "C" {
    static boot_stacks_top: u8;
}
```

## 4. Integrating with Rust build
In kernel root:
```rust
core::arch::global_asm!(include_str!("arch/riscv64/start.S"));
```

Rules:
- Keep startup asm tiny.
- Move policy and branching logic into Rust.

## 5. ABI notes you must respect
- Function args use `a0`-`a7`.
- Return address is `ra`.
- Stack must stay 16-byte aligned at call boundaries.

If stack alignment is wrong, Rust code can crash unpredictably.

## 6. Debugging startup assembly
Useful GDB commands:
```gdb
set arch riscv:rv64
target remote :1234
break _start
si
info registers
x/10i $pc
```

## 7. Chapter deliverable
You have a startup file that cleanly hands off to Rust and is easy to debug.

## Checklist
- [ ] `_start` symbol is linked and reachable.
- [ ] Per-hart stack setup is implemented.
- [ ] `call rust_main` executes with valid stack alignment.
- [ ] There is an infinite park loop for unexpected returns.

Next: [06-memory-layout.md](06-memory-layout.md)
