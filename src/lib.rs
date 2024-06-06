#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![feature(naked_functions)]
#![no_std]

pub mod cp0;
pub mod gpr;
pub mod tlb;

use core::arch;

#[allow(dead_code)]
pub trait Reg {
    fn read() -> usize;
    fn write(val: usize);
}


