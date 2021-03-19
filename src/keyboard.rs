use crate::gpio;
use crate::timer;
use crate::uart;

pub struct Ps2DeviceT {
    clock: u32,
    data: u32,
}

enum Ps2Codes {
    PS2_KEY_NONE = 0,
    PS2_CODE_RELEASE = 0xF0,
    PS2_CODE_EXTENDED = 0xE0,
    PS2_KEY_SHIFT = 0x90,
    PS2_KEY_ALT, // values assigned in increasing sequence from here
    PS2_KEY_CTRL,
    PS2_KEY_CAPS_LOCK,
    PS2_KEY_ENTER,
    PS2_KEY_ESC,
    PS2_KEY_F1,
    PS2_KEY_F2,
    PS2_KEY_F3,
    PS2_KEY_F4,
    PS2_KEY_F5,
    PS2_KEY_F6,
    PS2_KEY_F7,
    PS2_KEY_F8,
    PS2_KEY_F9,
    PS2_KEY_F10,
    PS2_KEY_F11,
    PS2_KEY_F12,
    PS2_KEY_NUM_LOCK,
    PS2_KEY_HOME,
    PS2_KEY_PAGE_UP,
    PS2_KEY_PAGE_DOWN,
    PS2_KEY_INSERT,
    PS2_KEY_DELETE,
    PS2_KEY_END,
    PS2_KEY_SCROLL_LOCK,
    PS2_KEY_ARROW_UP,
    PS2_KEY_ARROW_DOWN,
    PS2_KEY_ARROW_LEFT,
    PS2_KEY_ARROW_RIGHT,
}

const ps2_keys: &[char] = &[
    /* scan code */
    /* 00 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 01 */ { Ps2Codes::PS2_KEY_F9 as u8 as char },
    /* 02 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 03 */ { Ps2Codes::PS2_KEY_F5 as u8 as char },
    /* 04 */ { Ps2Codes::PS2_KEY_F3 as u8 as char },
    /* 05 */ { Ps2Codes::PS2_KEY_F1 as u8 as char },
    /* 06 */ { Ps2Codes::PS2_KEY_F2 as u8 as char },
    /* 07 */ { Ps2Codes::PS2_KEY_F12 as u8 as char },
    /* 08 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 09 */ { Ps2Codes::PS2_KEY_F10 as u8 as char },
    /* 0A */ { Ps2Codes::PS2_KEY_F8 as u8 as char },
    /* 0B */ { Ps2Codes::PS2_KEY_F6 as u8 as char },
    /* 0C */ { Ps2Codes::PS2_KEY_F4 as u8 as char },
    /* 0D */ { '\t' },
    /* 0E */ { '`' },
    /* 0F */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 10 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 11 */ { Ps2Codes::PS2_KEY_ALT as u8 as char },
    /* 12 */ { Ps2Codes::PS2_KEY_SHIFT as u8 as char },
    /* 13 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 14 */ { Ps2Codes::PS2_KEY_CTRL as u8 as char },
    /* 15 */ { 'q' },
    /* 16 */ { '1' },
    /* 17 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 18 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 19 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 1A */ { 'z' },
    /* 1B */ { 's' },
    /* 1C */ { 'a' },
    /* 1D */ { 'w' },
    /* 1E */ { '2' },
    /* 1F */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 20 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 21 */ { 'c' },
    /* 22 */ { 'x' },
    /* 23 */ { 'd' },
    /* 24 */ { 'e' },
    /* 25 */ { '4' },
    /* 26 */ { '3' },
    /* 27 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 28 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 29 */ { ' ' },
    /* 2A */ { 'v' },
    /* 2B */ { 'f' },
    /* 2C */ { 't' },
    /* 2D */ { 'r' },
    /* 2E */ { '5' },
    /* 2F */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 30 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 31 */ { 'n' },
    /* 32 */ { 'b' },
    /* 33 */ { 'h' },
    /* 34 */ { 'g' },
    /* 35 */ { 'y' },
    /* 36 */ { '6' },
    /* 37 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 38 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 39 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 3A */ { 'm' },
    /* 3B */ { 'j' },
    /* 3C */ { 'u' },
    /* 3D */ { '7' },
    /* 3E */ { '8' },
    /* 3F */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 40 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 41 */ { ',' },
    /* 42 */ { 'k' },
    /* 43 */ { 'i' },
    /* 44 */ { 'o' },
    /* 45 */ { '0' },
    /* 46 */ { '9' },
    /* 47 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 48 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 49 */ { '.' },
    /* 4A */ { '/' },
    /* 4B */ { 'l' },
    /* 4C */ { ';' },
    /* 4D */ { 'p' },
    /* 4E */ { '-' },
    /* 4F */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 50 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 51 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 52 */ { '\'' },
    /* 53 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 54 */ { '[' },
    /* 55 */ { '=' },
    /* 56 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 57 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 58 */ { Ps2Codes::PS2_KEY_CAPS_LOCK as u8 as char },
    /* 59 */ { Ps2Codes::PS2_KEY_SHIFT as u8 as char },
    /* 5A */ { '\n' },
    /* 5B */ { ']' },
    /* 5C */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 5D */ { '\\' },
    /* 5E */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 5F */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 60 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 61 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 62 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 63 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 64 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 65 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 66 */ { '' },
    /* 67 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 68 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 69 */ { Ps2Codes::PS2_KEY_END as u8 as char },
    /* 6A */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 6B */ { Ps2Codes::PS2_KEY_ARROW_LEFT as u8 as char },
    /* 6C */ { Ps2Codes::PS2_KEY_HOME as u8 as char },
    /* 6D */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 6E */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 6F */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 70 */ { Ps2Codes::PS2_KEY_INSERT as u8 as char },
    /* 71 */ { Ps2Codes::PS2_KEY_DELETE as u8 as char },
    /* 72 */ { Ps2Codes::PS2_KEY_ARROW_DOWN as u8 as char },
    /* 73 */ { '5' },
    /* 74 */ { Ps2Codes::PS2_KEY_ARROW_RIGHT as u8 as char },
    /* 75 */ { Ps2Codes::PS2_KEY_ARROW_UP as u8 as char },
    /* 76 */ { Ps2Codes::PS2_KEY_ESC as u8 as char },
    /* 77 */ { Ps2Codes::PS2_KEY_NUM_LOCK as u8 as char },
    /* 78 */ { Ps2Codes::PS2_KEY_F11 as u8 as char },
    /* 79 */ { '+' },
    /* 7A */ { Ps2Codes::PS2_KEY_PAGE_DOWN as u8 as char },
    /* 7B */ { '-' },
    /* 7C */ { '*' },
    /* 7D */ { Ps2Codes::PS2_KEY_PAGE_UP as u8 as char },
    /* 7E */ { Ps2Codes::PS2_KEY_SCROLL_LOCK as u8 as char },
    /* 7F */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 80 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 81 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 82 */ { Ps2Codes::PS2_KEY_NONE as u8 as char },
    /* 83 */ { Ps2Codes::PS2_KEY_F7 as u8 as char },
];

#[derive(Copy, Clone)]
pub struct KeyActionT {
    what: u32,
    keycode: u32,
}

pub struct KeyEventT {
    action: KeyActionT,
    key: char,
}

static mut dev: Ps2DeviceT = Ps2DeviceT { clock: 3, data: 4 };

pub unsafe fn init() {
    /*
        dev = Ps2DeviceT {
            clock: clock_gpio as u32,
            data: data_gpio as u32,
        };
    */
    gpio::set_input(dev.clock as isize);
    gpio::set_pullup(dev.clock as isize);

    gpio::set_input(dev.data as isize);
    gpio::set_pullup(dev.data as isize);
}

static mut timeout: u32 = 0;

unsafe fn wait_for_falling_clock_edge() {
    let start: u32 = timer::get_ticks();
    while gpio::read(dev.clock as u8 as isize) == 0 {
        if timer::get_ticks() - start > 100000 {
            timeout = 1;
            break;
        }
    }
    while gpio::read(dev.clock as u8 as isize) == 1 {
        if timer::get_ticks() - start > 100000 {
            timeout = 1;
            break;
        } else if timeout == 1 {
            break;
        }
    }
}

unsafe fn read_bit() -> u32 {
    wait_for_falling_clock_edge();
    if timeout == 1 {
        return 1;
    }
    return gpio::read(dev.data as u8 as isize);
}

pub unsafe fn read_scancode() -> u32 {
    let mut scancode: u32 = 0;
    let mut paritycheck: u32 = 0;
    loop {
        while read_bit() == 1 {
            if timeout == 1 {
                return 0;
            }
        }

        for i in 0..8 {
            let bit: u32 = read_bit();
            scancode |= bit << i;
            paritycheck += bit;
        }

        let paritybit: u32 = read_bit();
        paritycheck += paritybit;
        let stopbit: u32 = read_bit();

        if paritycheck % 2 != 1 || stopbit != 1 {
            scancode = 0;
            paritycheck = 0;
            continue;
        }
        break;
    }
    return scancode;
}

pub unsafe fn read_sequence() -> KeyActionT {
    let mut action = KeyActionT {
        what: 0,
        keycode: 0,
    };

    let mut keycode: u32 = read_scancode();
    if timeout == 1 {
        return action;
    }

    if keycode == 0xE0 {
        keycode = read_scancode();
    }

    if keycode == 0xF0 {
        action.what = 1;
        action.keycode = read_scancode();
    } else {
        action.what = 0;
        action.keycode = keycode;
    }
    return action;
}

pub unsafe fn read_event() -> KeyEventT {
    let temp = KeyActionT {
        what: 0,
        keycode: 0,
    };

    let mut event = KeyEventT {
        action: temp,
        key: '\0',
    };

    loop {
        let mut action: KeyActionT = read_sequence();
        if timeout == 1 {
            return event;
        }

        if action.keycode != 0x12
            && action.keycode != 0x59
            && action.keycode != 0x11
            && action.keycode != 0x14
            && action.keycode != 0x58
            && action.keycode != 0x7e
            && action.keycode != 0x77
        {
            event.action = action;
            event.key = ps2_keys[action.keycode as usize];
            break;
        }
    }

    return event;
}

pub unsafe fn read_next() -> char {
    let mut keyevent: KeyEventT = read_event();
    if timeout == 1 {
        timeout = 0;
    }
    loop {
        if keyevent.action.what == 0 {
            break;
        }
        keyevent = read_event();
    }

    return keyevent.key;
}

#[test_case]
pub fn test() {
    unsafe {
        let mut inputchar: char = read_next();
        while inputchar != '`' {
            uart::put_u8(inputchar as u8);
            inputchar = read_next();
        }
    }
}
