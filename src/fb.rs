#[path = "mailbox.rs"] mod mailbox;

extern crate lazy_static;
use lazy_static::lazy_static;

#[repr(C, align(16))]
struct FbConfigT {
    width: u32,
    height: u32,
    virtual_width: u32,
    virtual_height: u32,
    pitch: u32,
    bit_depth: u32,
    x_offset: u32,
    y_offset: u32,
    framebuffer: u32,
    total_bytes: u32,
}

const GPIO_BASE: u32 = 0x20200000;

lazy_static! {
    static ref fb_width: u32 = 100;
    static ref fb_height: u32 = 100;
    static ref fb_virtual_height: u32 = 100;
    static ref fb_virtual_width: u32 = 100;
    static ref fb_pitch: u32 = 0;
    static ref fb_bit_depth: u32 = 32;
    static ref fb_x_offset: u32 = 0;
    static ref fb_y_offset: u32 = 0;
    static ref fb_framebuffer: u32 = 0;
    static ref fb_total_bytes: u32 = 0;
}

pub fn test() {
    let gpio = GPIO_BASE as *const u32;
    let led_on = unsafe { gpio.offset(8) as *mut u32 };
    
    // I have no clue what the & is doing here
    //unsafe { *fb_width };

    let config = FbConfigT {
        width: 100,
        height: 100,
        virtual_width: 100,
        virtual_height: 100,
        pitch: 0,
        bit_depth: 32,
        x_offset: 0,
        y_offset: 0,
        framebuffer: 0,
        total_bytes: 0,
    };
    
    let config_addr: u32 = (&config as *const _) as u32;
    let mailbox_success: bool = mailbox::mailbox_request(1, config_addr);

    if mailbox_success {
        unsafe {
            *(led_on) = 1 << 15;
        }
    }

    loop {}
}

