
// Register contexts used in various architectures
//
// These structures all represent a context of one task throughout its
// execution. Each struct is a representation of the architecture's register
// set. When swapping between tasks, these register sets are used to save off
// the current registers into one struct, and load them all from another.
//
// Note that this is only used for context switching, which means that some of
// the registers may go unused. For example, for architectures with
// callee/caller saved registers, the context will only reflect the callee-saved
// registers. This is because the caller saved registers are already stored
// elsewhere on the stack (if it was necessary anyway).
//
// Additionally, there may be fields on various architectures which are unused
// entirely because they only reflect what is theoretically possible for a
// "complete register set" to show, but user-space cannot alter these registers.
// An example of this would be the segment selectors for x86.
//
// These structures/functions are roughly in-sync with the source files inside
// of src/rt/arch/$arch. The only currently used function from those folders is
// the `rust_swap_registers` function, but that's only because for now segmented
// stacks are disabled.

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::{Registers, swap_registers, load_registers, initialize_call_frame};

#[cfg(target_arch = "x86")]
pub use self::x86::{Registers, swap_registers, save_registers, load_registers, initialize_call_frame};

#[cfg(target_arch = "arm")]
pub use self::arm::{Registers, swap_registers, save_registers, load_registers, initialize_call_frame};

#[cfg(any(target_arch = "mips",
          target_arch = "mipsel"))]
pub use self::mips::{Registers, swap_registers, save_registers, load_registers, initialize_call_frame};

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86")]
pub mod x86;

#[cfg(target_arch = "arm")]
pub mod arm;

#[cfg(any(target_arch = "mips",
          target_arch = "mipsel"))]
pub mod mips;

fn align_down(sp: *mut usize) -> *mut usize {
    let sp = (sp as usize) & !(16 - 1);
    sp as *mut usize
}

// ptr::mut_offset is positive isizes only
#[inline]
fn mut_offset<T>(ptr: *mut T, count: isize) -> *mut T {
    // use std::mem::size_of;
    // (ptr as isize + count * (size_of::<T>() as isize)) as *mut T
    unsafe { ptr.offset(count) }
}
