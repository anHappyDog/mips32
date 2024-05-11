use core::arch::asm;
use crate::Reg;



macro_rules! DEFINE_CP0 {
    ($ident:ident,$no:literal) => {
        #[allow(non_camel_case_types)]
        pub struct $ident;
        impl Reg for $ident {
            #[inline(never)]
            fn read() -> usize {
                let _ret : usize;
                unsafe {
                    asm!(concat!("mfc0 $t0, ",$no),out("$8") _ret,options(nomem,nostack));
                } 
                _ret
            }
            #[inline(never)]
            fn write(val: usize) {
                unsafe {
                    asm!(concat!("mtc0 $t0, ",$no),in("$8") val,options(nomem,nostack));
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
                    asm!(concat!("mfc0 $8, ",$no,", ",$str),out("$8") _ret);
                } 
                _ret
            }
            fn write(val: usize) {
                unsafe {
                    asm!(concat!("mtc0 $8, ",$no,", ",$str),in("$8") val);
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
