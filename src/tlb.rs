use core::{arch, mem::size_of};

pub const NASID: usize = 256;
pub const PAGE_SHIFT: usize = 12;

extern "C" {
    fn tlb_out(entryhi: usize);
}

//#define GENMASK(h, l) (((~0UL) << (l)) & (~0UL >> (BITS_PER_LONG - 1 - (h))))
fn gen_mask(h: usize, l: usize) -> usize {
    ((!0 as usize) << l) & ((!0 as usize) >> (8*size_of::<usize>() - 1 - h))
}

#[inline(always)]
pub fn tlb_invalidate(va: usize, asid: usize) {
    unsafe {
        tlb_out((va & !gen_mask(PAGE_SHIFT, 0)) | (asid & (NASID - 1)));
    }
}

arch::global_asm!(include_str!("./tlb.S"));

#[inline(always)]
pub fn tlbwr() {
    unsafe {
        arch::asm!("tlbwr");
    }
}
