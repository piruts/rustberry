const GPIO_BASE: u32 = 0x20200000;
const GPIO_FSEL0: *mut u32 = GPIO_BASE as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;
const GPIO_LEV0: *mut u32 = (GPIO_BASE + 0x34) as *mut u32;

extern "C" {
    pub fn dev_barrier();
}

pub unsafe fn set_function(pin: isize, function: u32) {
    dev_barrier();
    let fsel: *mut u32 = GPIO_FSEL0.offset(pin / 10);
    fsel.write_volatile(fsel.read_volatile() & !(0b111 << ((pin % 10) * 3)));
    fsel.write_volatile(fsel.read_volatile() | (function << ((pin % 10) * 3)));
    dev_barrier();
}

#[allow(unused)]
pub unsafe fn get_function(pin: isize) -> u32 {
    dev_barrier();
    let fsel: *mut u32 = GPIO_FSEL0.offset(pin / 10);
    return (fsel.read_volatile() >> ((pin % 10) * 3)) & 0b111;
}

#[allow(unused)]
pub unsafe fn set_input(pin: isize) { 
    set_function(pin, 0);
}

#[allow(unused)]
pub unsafe fn set_output(pin: isize) {
    set_function(pin, 1);
}

#[allow(unused)]
pub unsafe fn write(pin: isize, value: u32) {
    dev_barrier();
    if value == 0 {
        let clr: *mut u32 = GPIO_CLR0.offset(pin / 32);
        clr.write_volatile(1 << (pin % 32));
    } else if value == 1 {
        let set: *mut u32 = GPIO_SET0.offset(pin / 32);
        set.write_volatile(1 << (pin % 32));
    }
    dev_barrier();
}

#[allow(unused)]
pub unsafe fn read(pin: isize) -> u32 {
    dev_barrier();
    let lev: *mut u32 = GPIO_LEV0.offset(pin / 32);
    return (lev.read_volatile() >> (pin % 32)) & 0b1;
}
