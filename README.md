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
