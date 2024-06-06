use core::arch::asm;
use crate::Reg;

pub const ST_CU0: usize = 1 << 28;
pub const ST_IM: usize = 1 << 8;
pub const ST_IE: usize = 1 << 0;
pub const ST_BEV: usize = 1 << 22;
pub const ST_EXL: usize = 1 << 1;
pub const ST_UM: usize = 1 << 4;
pub const ST_ERL: usize = 1 << 2;
pub const ST_KSU: usize = 3 << 3;
pub const ST_KSU_USER: usize = 1 << 3;
pub const ST_KSU_SUPER: usize = 2 << 3;
pub const ST_KSU_KERNEL: usize = 0 << 3;
pub const ST_UX: usize = 1 << 0;
pub const ST_UX_SHIFT: usize = 0;
pub const ST_UX_MASK: usize = 1 << 0;
pub const ST_SX: usize = 1 << 1;

pub const ST_IM7: usize = 1 << 15;
pub const ST_IM6: usize = 1 << 14;
pub const ST_IM5: usize = 1 << 13;
pub const ST_IM4: usize = 1 << 12;
pub const ST_IM3: usize = 1 << 11;
pub const ST_IM2: usize = 1 << 10;
pub const ST_IM1: usize = 1 << 9;
pub const ST_IM0: usize = 1 << 8;


macro_rules! DEFINE_CP0 {
    ($ident:ident,$no:literal) => {
        #[allow(non_camel_case_types)]
        pub struct $ident;
        impl Reg for $ident {
            #[inline(never)]
            fn read() -> usize {
                let _ret : usize;
                unsafe {
                    asm!(concat!(".set noat\nmfc0 {}, ",$no,"\n.set at\n"),out(reg) _ret,options(nomem,nostack));
                } 
                _ret
            }
            #[inline(never)]
            fn write(val: usize) {
                unsafe {
                    asm!(concat!(".set noat\nmtc0 {}, ",$no,"\n.set at\n"),in(reg) val,options(nomem,nostack));
                }
            }
        }
    };
    ($ident:ident,$no:literal,$str:literal) => {
        #[allow(non_camel_case_types)]
        pub struct $ident;
        impl Reg for $ident {
            fn read() -> usize {
                let _ret : usize;
                unsafe {
                    asm!(concat!(".set noat\nmfc0 {}, ",$no,", ",$str,"\n.set at\n"),out(reg) _ret);
                } 
                _ret
            }
            fn write(val: usize) {
                unsafe {
                    asm!(concat!(".set noat\nmtc0 {}, ",$no,", ",$str,"\n.set at\n"),in(reg) val);
                }
            }
        }
    };
}


DEFINE_CP0!(sr,"$12");
DEFINE_CP0!(cause,"$13");
DEFINE_CP0!(epc,"$14");
DEFINE_CP0!(count,"$9");
DEFINE_CP0!(compare,"$11");
DEFINE_CP0!(badvaddr,"$8");
DEFINE_CP0!(context,"$4");
DEFINE_CP0!(entryhi,"$10");
DEFINE_CP0!(entrylo0,"$2");
DEFINE_CP0!(entrylo1,"$3");
DEFINE_CP0!(index,"$0");
DEFINE_CP0!(random,"$1");
DEFINE_CP0!(wired,"$6");
DEFINE_CP0!(prid,"$15");
DEFINE_CP0!(config,"$16");
DEFINE_CP0!(config1,"$16","1");
DEFINE_CP0!(config2,"$16","2");
DEFINE_CP0!(config3,"$16","3");
DEFINE_CP0!(ebase,"$15","1");
DEFINE_CP0!(intctl,"$12","1");
DEFINE_CP0!(srsctl,"$12","2");
DEFINE_CP0!(srsmap,"$12","3");
DEFINE_CP0!(cacheerr,"$27");
DEFINE_CP0!(ecc,"$26");
DEFINE_CP0!(errorepc,"$30");
DEFINE_CP0!(taglo,"$28");
DEFINE_CP0!(taghi,"$29");
DEFINE_CP0!(datalo,"$28","1");
DEFINE_CP0!(datahi,"$29","1");
DEFINE_CP0!(debug,"$23");
DEFINE_CP0!(depc,"$24","3");
DEFINE_CP0!(desave,"$31");
DEFINE_CP0!(watchlo,"$18");
DEFINE_CP0!(watchhi,"$19");
DEFINE_CP0!(perfctl,"$25");
DEFINE_CP0!(perfcnt,"$25","1");
DEFINE_CP0!(lladdr,"$17");
DEFINE_CP0!(hwrena,"$7","0");
