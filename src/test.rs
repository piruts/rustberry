const GPIO_BASE: u32 = 0x20200000; // leave here to test GPIO module

use crate::cpu;

pub fn success() {
    let gpio = GPIO_BASE as *const u32; //
    let led_on = unsafe { gpio.offset(8) as *mut u32 };
    let led_off = unsafe { gpio.offset(11) as *mut u32 };

    loop {
        unsafe {
            *(led_on) = 1 << 15;
        }
        cpu::sleep(1000000);
        unsafe {
            *(led_off) = 1 << 15;
        }
        cpu::sleep(1000000);
    }
}

pub fn start_tests() {
    let gpio = GPIO_BASE as *const u32; //
    let fsel_3 = unsafe { gpio.offset(3) as *mut u32 };
    let led_off = unsafe { gpio.offset(11) as *mut u32 };

    unsafe {
        *(fsel_3) = 1 << 14;
    }

    unsafe {
        *(led_off) = 1 << 3;
    }
}
