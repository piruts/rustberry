// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>
// 
// Edited 2021 by Flynn Dreilinger <flynnd@stanford.edu>, Ashish Rao <aprao@stanford.edu>, and Xiluo He <xiluohe@stanford.edu>

#![feature(asm)]
#![feature(alloc_error_handler)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![no_main]
#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod allocator;
mod bsp;
mod cpu;
mod fb;
mod gl;
mod gpio;
mod keyboard;
mod led_test_harness;
mod mailbox;
mod memory;
mod panic_wait;
mod runtime_init;
mod space_invaders;
mod timer;
mod uart;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        space_invaders::run_game();
    }
    led_test_harness::success(); // flashes green led forever
}

// -------------------------------------------------------------------------------------------------
// tests harness start
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    //  allocator::init_heap();
    led_test_harness::start_tests();
    for test in tests {
        test();
    }
    led_test_harness::success(); // flashes green led forever
}

// -------------------------------------------------------------------------------------------------
// tests start end
// -------------------------------------------------------------------------------------------------
