.section .text ,"ax",%progbits
.globl tlb_out;
.align 2;
.type tlb_out,@function;
.ent tlb_out;
tlb_out:
.set noreorder
    mfc0    $t0, $10
    mtc0    $a0, $10
    nop
    tlbp
    nop
    mfc0    $t1, $0
.set reorder
    bltz    $t1,NO_SUCH_ENTRY
.set noreorder
    mtc0    $zero, $10
    mtc0    $zero, $2
    mtc0    $zero, $3
    nop
    tlbwi
.set reorder
NO_SUCH_ENTRY:
    mtc0    $t0,$10
    j       $ra
.end tlb_out