use alloc::boxed::Box;

use crate::gpio;
use crate::timer;
use crate::uart;

pub struct Ps2DeviceT {
    clock: u32,
    data: u32,
}

//static mut timeout: u32 = 0;

pub unsafe fn ps2_new(clock_gpio: u8, data_gpio: u8) -> Box<Ps2DeviceT> {
    let dev = Ps2DeviceT {
        clock: clock_gpio as u32,
        data: data_gpio as u32,
    };

    gpio::set_input(dev.clock as isize);
    gpio::set_pullup(dev.clock as isize);

    gpio::set_input(dev.data as isize);
    gpio::set_pullup(dev.data as isize);

    let dev: Box<Ps2DeviceT> = Box::new(dev);

    return dev;
}

unsafe fn wait_for_falling_clock_edge(dev: &Box<Ps2DeviceT>) {
    while gpio::read((*dev).clock as u8 as isize) == 0 {}
    while gpio::read((*dev).clock as u8 as isize) == 1 {}
}

unsafe fn read_bit(dev: &Box<Ps2DeviceT>) -> u32 {
    //let static prevTick: mut u32 = timer::get_ticks;
    wait_for_falling_clock_edge(&dev);
    //let curTick = timer::get_ticks;

    //if prevTick != 0 && curTick - prevTick > 3000 {
    //    timeout = 1;
    //}
    //prevTick = timer::get_ticks;
    //let temp = dev;
    return gpio::read((*dev).data as u8 as isize);
}

pub unsafe fn ps2_read(dev: &Box<Ps2DeviceT>) -> u32 {
    let mut scancode: u32 = 0;
    let mut paritycheck: u32 = 0;

    loop {
        while read_bit(&dev) == 1 {}

        for i in 0..8 {
            let bit: u32 = read_bit(&dev);
            scancode |= bit << i;
            paritycheck += bit;
        }

        let paritybit: u32 = read_bit(&dev);
        paritycheck += paritybit;
        let stopbit: u32 = read_bit(&dev);

        if paritycheck % 2 != 1 || stopbit != 1 {
            // || timeout == 1
            scancode = 0;
            paritycheck = 0;
            //            timeout = 0;
            continue;
        }

        break;
    }
    return scancode;
}

#[test_case]
pub fn test() {
    unsafe {
        uart::put_u8(0x30);
    }
    let dev: Box<Ps2DeviceT> = unsafe { ps2_new(3, 4) };

    let scancode: u32 = unsafe { ps2_read(&dev) };
}
/*
static ps2_device_t *dev;

void keyboard_init(unsigned int clock_gpio, unsigned int data_gpio)
{
    dev = ps2_new(clock_gpio, data_gpio);
}

//This function returns the keycode for a given keyboard action
unsigned char keyboard_read_scancode(void)
{
    return ps2_read(dev);
}
*/
