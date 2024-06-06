use core::arch::asm;
use crate::Reg;

macro_rules! DEFINE_GPR {
    ($ident:ident,$str:literal) => {
        #[allow(non_camel_case_types)]
        pub struct $ident;
        impl Reg for $ident {
            #[inline(always)]
            fn read() -> usize {
                let _ret : usize;
                unsafe {
                    asm!(concat!(".set noat\nmove {}, ",$str,"\n.set at\n"),out(reg) _ret,options(nomem,nostack));
                } 
                _ret
            }
            #[inline(always)]
            fn write(val: usize) {
                unsafe {
                    asm!(concat!(".set noat\nmove ",$str,",{}\n.set at\n"),in(reg) val,options(nomem,nostack));
                }
            }
        }
    };
}



DEFINE_GPR!(v0,"$v0");
DEFINE_GPR!(v1,"$v1");
DEFINE_GPR!(a0,"$a0");
DEFINE_GPR!(a1,"$a1");
DEFINE_GPR!(a2,"$a2");
DEFINE_GPR!(a3,"$a3");
DEFINE_GPR!(t0,"$t0");
DEFINE_GPR!(t1,"$t1");
DEFINE_GPR!(t2,"$t2");
DEFINE_GPR!(t3,"$t3");
DEFINE_GPR!(t4,"$t4");
DEFINE_GPR!(t5,"$t5");
DEFINE_GPR!(t6,"$t6");
DEFINE_GPR!(t7,"$t7");
DEFINE_GPR!(s0,"$s0");
DEFINE_GPR!(s1,"$s1");
DEFINE_GPR!(s2,"$s2");
DEFINE_GPR!(s3,"$s3");
DEFINE_GPR!(s4,"$s4");
DEFINE_GPR!(s5,"$s5");
DEFINE_GPR!(s6,"$s6");
DEFINE_GPR!(s7,"$s7");
DEFINE_GPR!(t8,"$t8");
DEFINE_GPR!(t9,"$t9");
DEFINE_GPR!(k0,"$k0");
DEFINE_GPR!(k1,"$k1");
DEFINE_GPR!(gp,"$gp");
DEFINE_GPR!(sp,"$sp");
DEFINE_GPR!(fp,"$fp");
DEFINE_GPR!(ra,"$ra");
