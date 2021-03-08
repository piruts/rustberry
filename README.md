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

### Install Bin Utils and LLVM Tools
```bash
cargo install cargo-binutils
rustup component add llvm-tools
```

## Build

```bash
make run
```
Generate docs with

```
make doc
```

Run linter
```
make clippy
```
