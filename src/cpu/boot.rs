// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021 Andre Richter <andre.o.richter@gmail.com>

//! Boot code.

#[cfg(target_arch = "arm")]
#[path = "../_arch/aarch32/cpu/boot.rs"]
mod arch_boot;
