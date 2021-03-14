// Author: Flynn Dreilinger <flynnd@stanford.edu>
// based on uart.c by Pat Hanrahan

// AUX bits
const AUX_ENABLES: u32 = 0x20215004;
const AUX_ENABLE: u32 = 0x00000001;

// Mini UART
const MINI_UART_BASE: u32 = 0x20215040;

const MINI_UART_IIR_RX_FIFO_CLEAR: u32 = 0x00000002;
const MINI_UART_IIR_TX_FIFO_CLEAR: u32 = 0x00000004;
const MINI_UART_IIR_RX_FIFO_ENABLE: u32 = 0x00000080;
const MINI_UART_IIR_TX_FIFO_ENABLE: u32 = 0x00000040;

const MINI_UART_LCR_8BIT: u32 = 0x00000003;

// const MINI_UART_LSR_RX_READY: u32 = 0x00000001;
// const MINI_UART_LSR_TX_READY: u32 = 0x00000010;
const MINI_UART_LSR_TX_EMPTY: u32 = 0x00000020;

const MINI_UART_CNTL_TX_ENABLE: u32 = 0x00000002;
const MINI_UART_CNTL_RX_ENABLE: u32 = 0x00000001;

const GPIO_BASE: u32 = 0x20200000; // leave here to test GPIO module

#[repr(C)]
pub struct Uart {
    pub data: u32,
    pub ier: u32,
    pub iir: u32,
    pub lcr: u32,
    pub mcr: u32,
    pub lsr: u32,
    pub msr: u32,
    pub scratch: u32,
    pub cntl: u32,
    pub stat: u32,
    pub baud: u32,
}
static mut UART: *mut Uart = MINI_UART_BASE as u32 as *mut Uart as *mut Uart;

// let mailbox = unsafe { &mut *(MAILBOX_BASE as *mut MailboxT) };

static mut INITIALIZED: bool = false;

/* Key detail from the Broadcom Peripherals data sheet p.10
*
* GPIO pins should be set up first the before enabling the UART.
* The UART core is build to emulate 16550 behaviour.
* So when it is enabled any data at the inputs will immediately be received .
* If the UART1_RX line is low (because the GPIO pins have not been set-up yet)
* that will be seen as a start bit and the UART will start receiving 0x00-characters.
* [...] The result will be that the FIFO is full and overflowing in no time flat.
*/
pub unsafe fn init() {
    let gpio = GPIO_BASE as *const u32;
    let fsel_1 = gpio.offset(1) as *mut u32;

    // configure tx (14) and rx (15) as alt fn 5
    {
        *(fsel_1) = 1001 << 10;
    }

    // must enable mini-uart before accessing registers
    let aux: *mut u32 = AUX_ENABLES as u32 as *mut u32;
    *aux |= AUX_ENABLE as u32;

    core::ptr::write_volatile(&mut (*UART).ier, 0_u32); // wait for char
    core::ptr::write_volatile(&mut (*UART).cntl, 0_u32);
    core::ptr::write_volatile(&mut (*UART).lcr, MINI_UART_LCR_8BIT as u32);
    core::ptr::write_volatile(&mut (*UART).mcr, 0_u32);
    core::ptr::write_volatile(&mut (*UART).ier, 0_u32);
    core::ptr::write_volatile(
        &mut (*UART).iir,
        (MINI_UART_IIR_RX_FIFO_CLEAR
            | MINI_UART_IIR_RX_FIFO_ENABLE
            | MINI_UART_IIR_TX_FIFO_CLEAR
            | MINI_UART_IIR_TX_FIFO_ENABLE) as u32,
    );
    // baud rate ((250,000,000/115200)/8)-1 = 270
    core::ptr::write_volatile(&mut (*UART).baud, 270_u32);
    core::ptr::write_volatile(
        &mut (*UART).cntl,
        (MINI_UART_CNTL_TX_ENABLE | MINI_UART_CNTL_RX_ENABLE) as u32,
    );
    INITIALIZED = true;
}

unsafe fn send(byte: u8) {
    while (*UART).lsr & MINI_UART_LSR_TX_EMPTY == 0 {}
    core::ptr::write_volatile(&mut (*UART).data, byte as u32 & 0xff_u32);
}

pub unsafe fn put_char(character: u8) -> u8 {
    // force initialize if not yet done
    // this fallback is special case for uart_putchar as
    // without it, all output (print/assert) can fail and no
    // clear indication because of self-referential nature of problem
    if !INITIALIZED {
        init();
    }

    // convert newline to CR LF sequence by inserting CR
    if character == b'\n' {
        send(b'\r');
    }
    send(character);
    character
}

#[test_case]
fn test_put_char() {
    unsafe {
        put_char(0xF0);
        put_char(0x9F);
        put_char(0x9A);
        put_char(0x80);
    }
}

/*
unsafe fn recieve() -> u8 {
    while !has_char() {}
    (*UART).data as u8
}
 */

/*
unsafe fn flush() {
    while (*UART).lsr & MINI_UART_LSR_TX_EMPTY as u32 == 0 {}
}

unsafe fn has_char() -> bool {
    (*UART).lsr & MINI_UART_LSR_RX_READY == 1
}

// RE: line endings
// canonical use is '\n' newline as sole line terminator (both read/write)
// but connected terminal may expect to receive a CR-LF sequence from Pi
// and may send a CR to Pi for return/enter key. get_char and put_char
// internally convert chars, client can simply send/receive newline
// Use send/recieve to send/receive raw byte, no conversion

unsafe fn get_char() -> u8 {
    let mut character = recieve();
    if character == b'\r' {
        character = b'\n'; // convert CR to newline
    }
    character
}
 */

/*

pub unsafe fn put_string(str: *const u8) -> u32 {
    let mut n: u32 = 0;
    while *str.offset(n as isize) != 0 {
        put_char(*str.offset(n as isize) as u8);
        n += 1
    }
    n
}

 */
