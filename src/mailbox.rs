const MAILBOX_BASE: u32 = 0x2000B880;
const MAILBOX_FULL: u32 = 1<<31;
const MAILBOX_EMPTY: u32 = 1<<30;
const MAILBOX_MAXCHANNEL: u32 = 16;
const GPU_NOCACHE: u32 = 0x40000000;

#[repr(C)]
struct MailboxT {
    read: u32,
    padding: [u32; 3],
    peek: u32,
    sender: u32,
    status: u32,
    configuration: u32,
    write: u32,
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
                let stat = core::ptr::read_volatile(&mut mailbox.status);
                if !((stat & MAILBOX_EMPTY) > 0) { break; }
            }
                
            let data: u32 = core::ptr::read_volatile(&mut mailbox.read);
            if (data & 0xF) == channel {
                return data >> 4;
            }
        }
    }
}

pub fn mailbox_write(channel: u32, mut addr: u32) -> bool {
    if channel >= MAILBOX_MAXCHANNEL { return false };
    if (addr & 0xF) > 0 { return false };
    let mailbox = unsafe { &mut *(MAILBOX_BASE as *mut MailboxT) };
   
    unsafe {
        loop {
            let stat = core::ptr::read_volatile(&mut mailbox.status);
            if !((stat & MAILBOX_FULL) > 0) { break };
        }
    }

    addr |= GPU_NOCACHE;
    
    unsafe {
        core::ptr::write_volatile(&mut mailbox.write, addr | channel);
    }
    
    return true;
}

