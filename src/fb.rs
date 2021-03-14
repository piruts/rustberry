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


pub const FB_SINGLEBUFFER: u32 = 0;
pub const FB_DOUBLEBUFFER: u32 = 1;

//const GPIO_BASE: u32 = 0x20200000;

static mut FB: FbConfigT = FbConfigT {
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

static mut BUFMODE: u32 = FB_SINGLEBUFFER;

use mailbox::MAILBOX_FRAMEBUFFER;
use mailbox::mailbox_request;

pub unsafe fn fb_init(width: u32, height: u32, 
               depth_in_bytes: u32, mode: u32) -> bool {
    BUFMODE = mode;
    
    FB.width = width;
    FB.height = height;
    FB.virtual_width = width;
    FB.virtual_height = 
        if mode == FB_SINGLEBUFFER {height} else {2*height};
    
    FB.bit_depth = 8 * depth_in_bytes;
    FB.x_offset = 0;
    FB.y_offset = 0;

    // GPU fills these values in
    FB.pitch = 0;
    FB.framebuffer = 0;
    FB.total_bytes = 0;
    
    let config_addr: u32 = (&FB as *const _) as u32;
    return mailbox_request(MAILBOX_FRAMEBUFFER, config_addr);
}

pub unsafe fn fb_swap_buffer() -> bool {
    if BUFMODE == FB_SINGLEBUFFER { return true };
    FB.y_offset = (FB.y_offset + FB.height) % (2*FB.height);
    
    let config_addr: u32 = (&FB as *const _) as u32;
    return mailbox_request(MAILBOX_FRAMEBUFFER, config_addr);
}

pub unsafe fn fb_get_draw_buffer() -> u32 {
    if BUFMODE == FB_SINGLEBUFFER { return FB.framebuffer; }
    let row_offset: u32 = FB.y_offset;
    return FB.framebuffer + row_offset*fb_get_pitch();
}

pub unsafe fn fb_get_width() -> u32 {
    return FB.width;
}

pub unsafe fn fb_get_height() -> u32 {
    return FB.height;
}

pub unsafe fn fb_get_depth() -> u32 {
    return FB.bit_depth / 8;
}

pub unsafe fn fb_get_pitch() -> u32 {
    return FB.pitch;
}

/*pub fn test() {
     
    unsafe {
        fb_init(100, 100, 4, FB_DOUBLEBUFFER);
        
        assert!(fb_swap_buffer())
    }
}*/
