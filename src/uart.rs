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

const MINI_UART_LSR_RX_READY: u32 = 0x00000001;
// const MINI_UART_LSR_TX_READY: u32 = 0x00000010;
const MINI_UART_LSR_TX_EMPTY: u32 = 0x00000020;

const MINI_UART_CNTL_TX_ENABLE: u32 = 0x00000002;
// const MINI_UART_CNTL_RX_ENABLE: u32 = 0x00000001;

const GPIO_BASE: u32 = 0x20200000; // leave here to test GPIO module

#[repr(C)]
pub struct UART {
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
static mut UART: *mut UART = MINI_UART_BASE as u32 as *mut UART as *mut UART;

/* Key detail from the Broadcom Peripherals data sheet p.10
*
* GPIO pins should be set up first the before enabling the UART.
* The UART core is build to emulate 16550 behaviour.
* So when it is enabled any data at the inputs will immediately be received .
* If the UART1_RX line is low (because the GPIO pins have not been set-up yet)
* that will be seen as a start bit and the UART will start receiving 0x00-characters.
* [...] The result will be that the FIFO is full and overflowing in no time flat.
*/
#[no_mangle]
pub unsafe extern "C" fn uart_init() {
    let gpio = GPIO_BASE as *const u32;
    let fsel_1 = gpio.offset(1) as *mut u32;

    // configure tx (14) as alt fn 45
    {
        *(fsel_1) = 1 << 11;
    }

    // configure rx (15) as alt fn 5
    {
        *(fsel_1) = 1 << 14;
    }

    // must enable mini-uart before accessing registers
    let aux: *mut u32 = AUX_ENABLES as u32 as *mut u32;
    *aux |= AUX_ENABLE as u32;

    core::ptr::write_volatile(&mut (*UART).ier as *mut u32, 0 as u32); // wait for char
    core::ptr::write_volatile(&mut (*UART).cntl as *mut u32, 0 as u32);
    core::ptr::write_volatile(&mut (*UART).lcr as *mut u32, MINI_UART_LCR_8BIT as u32);
    core::ptr::write_volatile(&mut (*UART).mcr as *mut u32, 0 as u32);
    core::ptr::write_volatile(&mut (*UART).ier as *mut u32, 0 as u32);
    core::ptr::write_volatile(
        &mut (*UART).iir as *mut u32,
        (MINI_UART_IIR_RX_FIFO_CLEAR
            | MINI_UART_IIR_RX_FIFO_ENABLE
            | MINI_UART_IIR_TX_FIFO_CLEAR
            | MINI_UART_IIR_TX_FIFO_ENABLE) as u32,
    );
    // baud rate ((250,000,000/115200)/8)-1 = 270
    core::ptr::write_volatile(&mut (*UART).baud as *mut u32, 270 as u32);
    core::ptr::write_volatile(
        &mut (*UART).cntl as *mut u32,
        (MINI_UART_CNTL_TX_ENABLE | MINI_UART_CNTL_TX_ENABLE) as u32,
    );
}

#[no_mangle]
unsafe fn recieve() -> u8 {
    while has_char() == false {}
    return (*UART).data as u8 & 0xff;
}

#[no_mangle]
unsafe fn send(byte: u8) {
    while (*UART).lsr & MINI_UART_LSR_TX_EMPTY as u32 == 0 {}
    core::ptr::write_volatile(&mut (*UART).data as *mut u32, byte as u32 & 0xff as u32);
}

#[no_mangle]
unsafe fn flush() {
    while (*UART).lsr & MINI_UART_LSR_TX_EMPTY as u32 == 0 {}
}

#[no_mangle]
unsafe fn has_char() -> bool {
    (*UART).lsr & MINI_UART_LSR_RX_READY != 0
}

#[no_mangle]
unsafe fn get_char() -> u8 {
    let mut character = recieve();
    if character as char == '\r' {
        character = 0xa;
    }
    character
}

#[no_mangle]
pub unsafe extern "C" fn put_char(ch: u8) -> u8 {
    // force initialize if not yet done
    // this fallback is special case for uart_putchar as
    // without it, all output (print/assert) can fail and no
    // clear indication because of self-referential nature of problem
    if ch as char == '\n' {
        send('\r' as u8);
    }
    send(ch);
    ch
}

#[no_mangle]
pub unsafe extern "C" fn put_string(str: *const u8) -> u32 {
    let mut n: u32 = 0;
    while *str.offset(n as isize) != 0 {
        put_char(*str.offset(n as isize) as u8);
        n += 1
    }
    return n;
}
