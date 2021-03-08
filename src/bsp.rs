// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//! Conditional reexporting of Board Support Packages.

#[cfg(feature = "bsp_rpiA")]
mod raspberrypi;

#[cfg(feature = "bsp_rpiA")]
pub use raspberrypi::*;
