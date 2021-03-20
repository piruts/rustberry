[![Build Status](https://staging.travis-ci.com/piruts/rustberry.svg?branch=main)](https://staging.travis-ci.com/piruts/rustberry)

# Rustberry Pi

## Setup

### Install Rustup

#### Macos

```sh
brew install rustup
```

#### Windows, Linux

visit: https://rustup.rs/#

### Configure the project to use `Nightly` Rust Channel

```sh
rustup override set nightly
```

### Install Dependencies

```sh
cargo install cargo-binutils rustfilt
rustup component add llvm-tools clippy rust-src
```

## Setup Hardware

Things you need:

- Raspberry Pi A
- CP2102 USB to UART Bridge
- PS/2 Keyboard
- HDMI Cable
- HDMI Display
- Jumper Wires
- Header Pin

### Wiring

| CP2102 | Raspberry Pi       |
|--------|--------------------|
| DTR    | Run*               |
| RXI    | GPIO 14 (UART TXD) |
| TXO    | GPIO 15 (UART RXD) |
| VCC    | 5V                 |
| GND    | GND                |

**you need to solder a pin on the Raspberry Pi*

Take a look at [Mini-DIN connectors](https://en.wikipedia.org/wiki/Mini-DIN_connector)

| PS/2 Keyboard | Raspberry Pi |
|---------------|--------------|
| DATA (1)      | GPIO 4       |
| VCC (4)       | 5V           |
| CLK (5)       | GPIO 3       |
| GND (3)       | GND          |

## Build and run

```sh
make run
```

---

## Individual Contributions

Flynn:

- Implemented UART module
- Implemented test framework
- Implemented Alloc Module
- Defined custom target for rust/llvm compilation

Ashish:

- Implemented mailbox and framebuffer modules
- Implemented support for embedded-graphics crate (see src/gl.rs)
- Worked on final game
- Animation with double buffering
  - Logic for firing beams and checking for collision with enemy ships
  - Movement of enemies and player

Xiluo:

- Implemented GPIO module and part of GPIO extra
- Implemented timer module
- Implemented ps2 module and keyboard support

## Resources Used

- In order to figure out how to run Rust on the Raspberry Pi, we referred to a Raspberry Pi ¾ project. https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials. Our Makefile was based on the one given under lesson 03_hacky_hello_world in this repository, and we included the linker file and based some of our modules off of modules in their codebase. All files used have their license and original author listed at the top.
- We used the embedded-graphics library for drawing rectangles and triangles: https://crates.io/crates/embedded-graphics. We implemented the low-level functions necessary to use this library on Raspberry Pi ourselves (mailbox and framebuffer).
- To learn more about and understand Rust, we referred to the Rust book and the embedded Rust book: https://doc.rust-lang.org/book/, https://docs.rust-embedded.org/book/
- We had help from Akshay Srivatsan and used Dawson Engler’s dev_barrier() code from CS140E