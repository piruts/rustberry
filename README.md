[![Build Status](https://staging.travis-ci.com/piruts/rustberry.svg?branch=main)](https://staging.travis-ci.com/piruts/rustberry)
# Rustberry Pi

## Setup

### Install Rustup

#### Macos
```
brew install rustup
```

#### Windows, Linux

visit: https://rustup.rs/#


### Configure the project to use `Nightly` Rust Channel

```
rustup override set nightly
```

### Install Dependencies
```bash
cargo install cargo-binutils rustfilt
rustup component add llvm-tools clippy rust-src
```

## Setup Hardware
Things you need:
- Raspberry Pi A
- CP2102 Breakout
- PS/2 Keyboard
- HDMI Cable
- HDMI Display
- Jumper cables

Before plugging anything in, connect 
### Keyboard

## Build

**Compile and run on hardware**

```bash
make run
```
**Generate docs**

```
make doc
```

**Run linter**
```
make clippy
```
