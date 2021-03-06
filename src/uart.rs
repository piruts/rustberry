// Author: Flynn Dreilinger <flynnd@stanford.edu>
// based on uart.c by Pat Hanrahan: https://github.com/cs107e/cs107e.github.io/blob/master/cs107e/src/uart.c

use crate::cpu;

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

const MINI_UART_LSR_RX_READY: u32 = 0x00000001;
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
    cpu::dev_barrier();
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
    cpu::dev_barrier();
}

unsafe fn send(byte: u8) {
    cpu::dev_barrier();
    while (*UART).lsr & MINI_UART_LSR_TX_EMPTY == 0 {}
    core::ptr::write_volatile(&mut (*UART).data, byte as u32 & 0xff_u32);
    cpu::dev_barrier();
}

unsafe fn receive() -> u8 {
    while !has_char() {}
    (*UART).data as u8
}

pub unsafe fn flush() {
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
// Use send/receive to send/receive raw byte, no conversion

unsafe fn get_char() -> u8 {
    let mut character = receive();
    if character == b'\r' {
        character = b'\n'; // convert CR to newline
    }
    character
}

pub unsafe fn put_u8(character: u8) {
    // force initialize if not yet done
    // this fallback is special case for uart_putchar as
    // without it, all output (print/assert) can fail and no
    // clear indication because of self-referential nature of problem
    if !INITIALIZED {
        init();
    }
    cpu::dev_barrier();
    send(character);
    cpu::dev_barrier();
}

#[test_case]
fn test_put_u8() {
    // say hello
    unsafe {
        put_u8(b'h');
        put_u8(b'e');
        put_u8(b'l');
        put_u8(b'l');
        put_u8(b'o');
        put_u8(b' ');
        put_u8(b'f');
        put_u8(b'r');
        put_u8(b'o');
        put_u8(b'm');
        put_u8(b' ');
        put_u8(b'r');
        put_u8(b'u');
        put_u8(b's');
        put_u8(b't');
        put_u8(b' ');
        put_u8(b':');
        put_u8(b')');
    }
}

/*
pub unsafe fn put_utf8_char(character: char) {
    cpu::dev_barrier();
    if !INITIALIZED {
        init();
    }
    for i in (0..4).rev() {
        put_u8((character as u32 >> (i * 2)) as u8 & 0xFF as u8);
    }
    cpu::dev_barrier();
}

#[test_case]
fn test_put_utf8_char() {
    // say hello
    unsafe {
        cpu::dev_barrier();
        put_utf8_char('h');
        put_utf8_char('e'); // TODO this test breaks something and does not panic, not sure what
                            // is going on here. Will fix in subsequent PR
        cpu::dev_barrier();
    }
}

pub unsafe fn launch() {
    put_u8(0xF0);
    put_u8(0x9F);
    put_u8(0x9A);
    put_u8(0x80);
}
*/

/*
pub unsafe fn put_string(str: str) {
    for c in str.chars() {
        put_u8(c as u8);
    }
}
*/
