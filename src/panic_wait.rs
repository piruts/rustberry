// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>
//
// Edited 2021 by Flynn Dreilinger <flynnd@stanford.edu> and Ashish Rao <aprao@stanford.edu>

//! A panic handler that infinitely waits.

use crate::{cpu, println};
use core::panic::PanicInfo;

const GPIO_BASE: u32 = 0x20200000;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(args) = info.message() {
        println!("\nKernel panic: {}", args);
    } else {
        println!("\nKernel panic!");
    }

    let gpio = GPIO_BASE as *const u32;
    let fsel_3 = unsafe { gpio.offset(3) as *mut u32 };
    let set_1 = unsafe { gpio.offset(8) as *mut u32 };
    let clr_1 = unsafe { gpio.offset(11) as *mut u32 };

    unsafe {
        *(fsel_3) = 1 << 15;
    }

    loop {
        unsafe {
            *(set_1) = 1 << 3;
        }
        cpu::sleep(1000000);
        unsafe {
            *(clr_1) = 1 << 3;
        }
        cpu::sleep(1000000);
    }
}
