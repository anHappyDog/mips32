use core::arch;

#[inline(always)]
pub fn tlb_invalidate(va: usize, asid: usize) {

}

#[inline(always)]
pub fn tlbwr() {
    unsafe {
        arch::asm!("tlbwr");
    }
}
