// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2020-2021 Andre Richter <andre.o.richter@gmail.com>
//
// Edited 2021 by Flynn Dreilinger <flynnd@stanford.edu> and Ashish Rao <aprao@stanford.edu>

//! Processor code.

#[cfg(target_arch = "arm")]
#[path = "_arch/aarch32/cpu.rs"]
mod arch_cpu;

mod boot;

//--------------------------------------------------------------------------------------------------
// Architectural Public Reexports
//--------------------------------------------------------------------------------------------------
pub use arch_cpu::wait_forever;

// Pause execution on the core by doing something small again and again.
pub fn sleep(value: u32) {
    for _ in 1..value {
        unsafe {
            asm!("");
        }
    }
}