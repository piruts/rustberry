// Author: Xiluo He <xiluohe@stanford.edu>
use crate::cpu;

const GPIO_BASE: u32 = 0x20200000;
const GPIO_FSEL0: *mut u32 = GPIO_BASE as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;
const GPIO_LEV0: *mut u32 = (GPIO_BASE + 0x34) as *mut u32;

pub unsafe fn set_function(pin: isize, function: u32) {
    cpu::dev_barrier();
    let fsel: *mut u32 = GPIO_FSEL0.offset(pin / 10);
    fsel.write_volatile(fsel.read_volatile() & !(0b111 << ((pin % 10) * 3)));
    fsel.write_volatile(fsel.read_volatile() | (function << ((pin % 10) * 3)));
    cpu::dev_barrier();
}

pub unsafe fn get_function(pin: isize) -> u32 {
    cpu::dev_barrier();
    let fsel: *mut u32 = GPIO_FSEL0.offset(pin / 10);
    return (fsel.read_volatile() >> ((pin % 10) * 3)) & 0b111;
}

pub unsafe fn set_input(pin: isize) {
    set_function(pin, 0);
}

pub unsafe fn set_output(pin: isize) {
    set_function(pin, 1);
}

pub unsafe fn write(pin: isize, value: u32) {
    cpu::dev_barrier();
    if value == 0 {
        let clr: *mut u32 = GPIO_CLR0.offset(pin / 32);
        clr.write_volatile(1 << (pin % 32));
    } else if value == 1 {
        let set: *mut u32 = GPIO_SET0.offset(pin / 32);
        set.write_volatile(1 << (pin % 32));
    }
    cpu::dev_barrier();
}

pub unsafe fn read(pin: isize) -> u32 {
    cpu::dev_barrier();
    let lev: *mut u32 = GPIO_LEV0.offset(pin / 32);
    return (lev.read_volatile() >> (pin % 32)) & 0b1;
}

const GPPUD: *mut u32 = (GPIO_BASE + 0x94) as *mut u32;
const GPPUDCLK: *mut u32 = (GPIO_BASE + 0x98) as *mut u32;

pub unsafe fn set_pud(pin: isize, pud: u32) {
    let gppudclk: *mut u32 = GPPUDCLK.offset(pin / 32);

    GPPUD.write_volatile(pud);
    for _x in 0..150 {}

    gppudclk.write_volatile(1 << (pin % 32));
    for _x in 0..150 {}

    gppudclk.write_volatile(0);
}

#[allow(unused)]
pub unsafe fn set_pullup(pin: isize) {
    set_pud(pin, 2);
}

#[allow(unused)]
pub unsafe fn set_pulldown(pin: isize) {
    set_pud(pin, 1);
}

#[allow(unused)]
pub unsafe fn set_pullnone(pin: isize) {
    set_pud(pin, 0);
}

#[test_case]
pub fn test() {
    unsafe {
        set_output(16);
        set_output(17);
        set_input(18);
        write(16, 1);
        write(17, 0);
        assert_eq!(read(16), 1);
        assert_eq!(get_function(16), 1);
        assert_eq!(read(17), 0);
        assert_eq!(get_function(17), 1);
        assert_eq!(get_function(18), 0);
        write(16, 0);
        assert_eq!(read(16), 0);
    }
}
