#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]

/*
const GPIO_BASE: u32 = 0x20200000 as *mut u32;
const GPIO_FSEL0: u32 = GPIO_BASE;
const GPIO_SET0: u32 = GPIO_BASE + 0x1c;
const GPIO_CLR0: u32 = GPIO_BASE + 0x28;
const GPIO_LEV0: u32 = GPIO_BASE + 0x34;
*/

const GPIO_BASE: u32 = 0x20200000;
const GPIO_FSEL0: *mut u32 = GPIO_BASE as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;
const GPIO_LEV0: *mut u32 = (GPIO_BASE + 0x34) as *mut u32;

pub unsafe fn gpio_init() {
    // no initialization required for this peripheral
}

pub unsafe fn gpio_set_function(pin: u32, function: u32) {
    let FSEL: *mut u32 = (GPIO_FSEL0 as u32 + (pin / 10)) as *mut u32;

    core::ptr::write_volatile(FSEL, core::ptr::read_volatile(FSEL) & !(0b111 << ((pin % 10) * 3)));
    
    core::ptr::write_volatile(FSEL, core::ptr::read_volatile(FSEL) | (function << ((pin % 10) * 3)));
    
    //FSEL.write_volatile(*FSEL & !(0b111 << ((pin % 10) * 3)));
    //FSEL.write_volatile(*FSEL | (function << ((pin % 10) * 3)));    
    
    //*FSEL = *FSEL & !(0b111 << ((pin % 10) * 3));
    //*FSEL = *FSEL | (function << ((pin % 10) * 3));
}

pub unsafe fn gpio_get_function(pin: u32) -> u32 {
    let FSEL: *mut u32 = (GPIO_FSEL0 as u32 + (pin / 10)) as *mut u32;
    //return (FSEL.read_volatile() >> ((pin % 10) * 3)) & 0b111;
    //return (*FSEL >> ((pin % 10) * 3)) & 0b111;
    return (core::ptr::read_volatile(FSEL) >> ((pin % 10) * 3)) & 0b111;
}

pub unsafe fn gpio_set_input(pin: u32) { 
    gpio_set_function(pin, 0);
}

pub unsafe fn gpio_set_output(pin: u32) { 
    gpio_set_function(pin, 1);
}

pub unsafe fn gpio_write(pin: u32, value: u32) {
    if value == 0 {
        let CLR: *mut u32 = (GPIO_CLR0 as u32 + (pin / 32)) as *mut u32;
        //*CLR = 1 << (pin % 32);
        //CLR.write_volatile(1 << (pin % 32));
        core::ptr::write_volatile(CLR, 1 << (pin % 32));
    } else if value == 1 {
        let SET: *mut u32 = (GPIO_SET0 as u32 + (pin / 32)) as *mut u32;
        //*SET = 1 << (pin % 32);
        //SET.write_volatile(1 << (pin % 32));
        core::ptr::write_volatile(SET, 1 << (pin % 32));
    }
}

pub unsafe fn gpio_read(pin: u32) -> u32 {
    let LEV: *mut u32 = (GPIO_LEV0 as u32 + (pin / 32)) as *mut u32;
    //return (LEV.read_volatile() >> (pin % 32)) & 0b1;
    //return (*LEV >> (pin % 32)) & 0b1;
    return (core::ptr::read_volatile(LEV) >> (pin % 32)) & 0b1;
}
