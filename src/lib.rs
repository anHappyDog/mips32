#![cfg_attr(target_arch="mips", feature(asm_experimental_arch))]
#![no_std]

pub mod cp0;
pub mod gpr;


#[allow(dead_code)]
pub trait Reg {
    fn read() -> usize;
    fn write(val: usize);
}


