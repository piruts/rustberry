#![no_std]
#![feature(asm)]
#![feature(global_asm)]
#![no_main]

global_asm!(include_str!("boot/boot.s"));


use core::panic::PanicInfo;

const GPIO_BASE: u32 = 0x20200000;
fn sleep(value: u32) {
    for _ in 1..value {
        unsafe { asm!(""); }
    }
}

// this function is called on panic.
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn main() {
    let gpio = GPIO_BASE as *const u32;
    let led_on = unsafe { gpio.offset(8) as *mut u32 };
    let led_off = unsafe { gpio.offset(11) as *mut u32 };

    loop {
        unsafe { *(led_on) = 1 << 15; }
        sleep(500000);
        unsafe { *(led_off) = 1 << 15; }
        sleep(500000);
    }
}
