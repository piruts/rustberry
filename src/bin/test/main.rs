#![feature(asm)]
#![feature(format_args_nl)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

use cpu;

fn it_works() {
    assert_eq!(2 + 2, 5);
}

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
#[no_mangle]
pub extern "C" fn main() -> ! {
    it_works();
    cpu::wait_forever();
    // panic!("Stopping here.");
}