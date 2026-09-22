mod multiboot1;
use core::arch::global_asm;

global_asm!(
    include_str!("entry.S"),
    options(att_syntax),
); // asm because it sets stack ptr
