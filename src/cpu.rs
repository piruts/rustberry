// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2020-2021 Andre Richter <andre.o.richter@gmail.com>

//! Processor code.

#[cfg(target_arch = "arm")]
#[path = "_arch/aarch32/cpu.rs"]
mod arch_cpu;

mod boot;

//--------------------------------------------------------------------------------------------------
// Architectural Public Reexports
//--------------------------------------------------------------------------------------------------
pub use arch_cpu::wait_forever;
