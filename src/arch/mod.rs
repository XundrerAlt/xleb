#[cfg(target_arch = "x86")]
mod i386;
#[cfg(target_arch = "x86")]
pub(crate) use i386::*;
