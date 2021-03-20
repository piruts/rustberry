// Author: Xiluo He <xiluohe@stanford.edu>
const TIME: *mut u32 = 0x20003004 as *mut u32;

pub unsafe fn get_ticks() -> u32 {
    return TIME.read_volatile();
}

pub unsafe fn delay_us(usecs: u32) {
    let start: u32 = get_ticks();
    while get_ticks() - start < usecs {}
}

pub unsafe fn delay_ms(msecs: u32) {
    delay_us(1000 * msecs);
}

pub unsafe fn delay(secs: u32) {
    delay_us(1000000 * secs);
}
