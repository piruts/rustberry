#[path = "mailbox.rs"] mod mailbox;

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

static mut FB_CONFIG: FbConfigT = FbConfigT {
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

pub fn fb_init(u32 width, u32 height, u32 depth_in_bytes, )

pub fn test() {
    let gpio = GPIO_BASE as *const u32;
    let led_on = unsafe { gpio.offset(8) as *mut u32 };
    
    let config_addr: u32 = unsafe { (&FB_CONFIG as *const _) as u32 };
    let mailbox_success: bool = mailbox::mailbox_request(1, config_addr);

    if mailbox_success {
        unsafe {
            *(led_on) = 1 << 15;
        }
    }

    loop {}
}

