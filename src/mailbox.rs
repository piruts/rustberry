const MAILBOX_BASE: u32 = 0x2000B880;
const MAILBOX_FULL: u32 = 1<<31;
const MAILBOX_EMPTY: u32 = 1<<30;
const MAILBOX_MAXCHANNEL: u32 = 16;
const GPU_NOCACHE: u32 = 0x40000000;

struct MailboxT {
    read: u32,
    padding: [u32; 3],
    peek: u32,
    sender: u32,
    status: u32,
    configuration: u32,
    write: u32,
}

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

pub fn test() {
    let gpio = GPIO_BASE as *const u32;
    let led_on = unsafe { gpio.offset(8) as *mut u32 };

    let config = FbConfigT {
        width: 0,
        height: 0,
        virtual_width: 0,
        virtual_height: 0,
        pitch: 0,
        bit_depth: 0,
        x_offset: 0,
        y_offset: 0,
        framebuffer: 0,
        total_bytes: 0,
    };

    let mailbox_success: bool = mailbox_request(1, (&config as *const _) as u32);

    //if mailbox_success {
        unsafe {
            *(led_on) = 1 << 15;
        }
    //}

    loop {}
}

pub fn mailbox_request(channel: u32, addr: u32) -> bool {
    if !mailbox_write(channel, addr) { return false };
    return mailbox_read(channel) == 0;
}

pub fn mailbox_read(channel: u32) -> u32 {
    if channel >= MAILBOX_MAXCHANNEL { return 1 };
    
    let mailbox = unsafe { &mut *(MAILBOX_BASE as *mut MailboxT) };
    
    unsafe {
        loop {
            loop {
                let stat = unsafe { core::ptr::read_volatile(&mut mailbox.status) };
                if !((stat & MAILBOX_EMPTY) > 0) { break; }
            }
                
            let data: u32 = unsafe { core::ptr::read_volatile(&mut mailbox.read) };
            if (data & 0xF) == channel {
                return data >> 4;
            }
        }
    }
}

pub fn mailbox_write(channel: u32, mut addr: u32) -> bool {
    let mailbox = unsafe { &mut *(MAILBOX_BASE as *mut MailboxT) };
    
    if channel >= MAILBOX_MAXCHANNEL { return false };
    if (addr & 0xF) > 0 { return false };
   
    unsafe {
        loop {
            let stat = unsafe { core::ptr::read_volatile(&mut mailbox.status) };
            if !((stat & MAILBOX_FULL) > 0) { break };
        }
    }

    addr |= GPU_NOCACHE;
    
    unsafe {
        core::ptr::write_volatile(&mut mailbox.write, addr | channel);
    }
    
    return true;
}

