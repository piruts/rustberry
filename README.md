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
cargo install cargo-binutils
rustup component add llvm-tools
rustup component add clippy
rustup component add rust-src
```

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
