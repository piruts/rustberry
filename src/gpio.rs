const GPIO_BASE: u32 = 0x20200000;
const GPIO_FSEL0: *mut u32 = GPIO_BASE as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;
const GPIO_LEV0: *mut u32 = (GPIO_BASE + 0x34) as *mut u32;

pub unsafe fn set_function(pin: u32, function: u32) {
    let fsel: *mut u32 = (GPIO_FSEL0 as u32 + (pin / 10) * 4) as *mut u32;

    core::ptr::write_volatile(fsel, core::ptr::read_volatile(fsel) & !(0b111 << ((pin % 10) * 3)));
    core::ptr::write_volatile(fsel, core::ptr::read_volatile(fsel) | (function << ((pin % 10) * 3)));
}

#[allow(unused)]
pub unsafe fn get_function(pin: u32) -> u32 {
    let fsel: *mut u32 = (GPIO_FSEL0 as u32 + (pin / 10) * 4) as *mut u32;
    return (core::ptr::read_volatile(fsel) >> ((pin % 10) * 3)) & 0b111;
}

#[allow(unused)]
pub unsafe fn set_input(pin: u32) { 
    set_function(pin, 0);
}

#[allow(unused)]
pub unsafe fn set_output(pin: u32) {
    set_function(pin, 1);
}

#[allow(unused)]
pub unsafe fn write(pin: u32, value: u32) {
    if value == 0 {
        let clr: *mut u32 = (GPIO_CLR0 as u32 + (pin / 32) * 4) as *mut u32;
        core::ptr::write_volatile(clr, 1 << (pin % 32));
    } else if value == 1 {
        let set: *mut u32 = (GPIO_SET0 as u32 + (pin / 32) * 4) as *mut u32;
        core::ptr::write_volatile(set, 1 << (pin % 32));
    }
}

pub unsafe fn read(pin: u32) -> u32 {
    let lev: *mut u32 = (GPIO_LEV0 as u32 + (pin / 32) * 4) as *mut u32;
    return (core::ptr::read_volatile(lev) >> (pin % 32)) & 0b1;
}
