// from https://docs.rs/embedded-hal/0.2.4/embedded_hal/

extern crate embedded-hal as hal;
extern crate nb;

// device crate
extern crate stm32f30x;

use stm32f30x::USART1;

/// A serial interface
// NOTE generic over the USART peripheral
pub struct Serial<USART> { usart: USART }

// convenience type alias
pub type Serial1 = Serial<USART1>;

/// Serial interface error
pub enum Error {
    /// Buffer overrun
    Overrun,
    // omitted: other error variants
}

impl hal::serial::Read<u8> for Serial<USART1> {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        // read the status register
        let isr = self.usart.isr.read();

        if isr.ore().bit_is_set() {
            // Error: Buffer overrun
            Err(nb::Error::Other(Error::Overrun))
        }
        // omitted: checks for other errors
        else if isr.rxne().bit_is_set() {
            // Data available: read the data register
            Ok(self.usart.rdr.read().bits() as u8)
        } else {
            // No data available yet
            Err(nb::Error::WouldBlock)
        }
    }
}

impl hal::serial::Write<u8> for Serial<USART1> {
    type Error = Error;

    fn write(&mut self, byte: u8) -> nb::Result<(), Error> {
        // Similar to the `read` implementation
    }

    fn flush(&mut self) -> nb::Result<(), Error> {
        // Similar to the `read` implementation
    }
}