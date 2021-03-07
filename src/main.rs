#![no_std]
#![no_main] // overwrite the entry point

use core::panic::PanicInfo;

// this function is called on panic.
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}