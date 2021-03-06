// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>
//
// Edited 2021 by Flynn Dreilinger <flynnd@stanford.edu>

//! Rust runtime initialization code.

use crate::{allocator, bsp, cpu, memory, uart};

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

/// Zero out the .bss section.
///
/// # Safety
///
/// - Must only be called pre `kernel_init()`.
#[inline(always)]
unsafe fn zero_bss() {
    memory::zero_volatile(bsp::memory::bss_range_inclusive());
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Equivalent to `crt0` or `c0` code in C/C++ world. Clears the `bss` section, then jumps to kernel
/// init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
#[no_mangle]
pub unsafe fn runtime_init() -> ! {
    zero_bss();
    uart::init();
    allocator::init();

    #[cfg(test)]
    crate::test_main();

    crate::main();
}
