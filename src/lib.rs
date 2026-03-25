#![no_std]

pub mod arch;
pub mod drivers;
pub mod fs;
pub mod mm;
pub mod proc;
pub mod trap;

pub fn kernel_probe() -> usize {
    42
}
