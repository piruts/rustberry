## SPDX-License-Identifier: MIT OR Apache-2.0
##
## Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>
## 
## Edited 2021 by Flynn Dreilinger <flynnd@stanford.edu> and Ashish Rao <aprao@stanford.edu>

PROJECT           = rustberry
TARGET            = armv6kz-none-eabi
APPLICATION       = $(PROJECT).bin
TEST_CMD		  = test.bin
OBJDUMP_BINARY    = arm-none-eabi-objdump
NM_BINARY         = arm-none-eabi-nm
LINKER_FILE       = src/bsp/raspberrypi/link.ld

# Export for build.rs
export LINKER_FILE

RUSTFLAGS          = -C link-arg=-T$(LINKER_FILE)
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) -D warnings -D missing_docs

FEATURES      = bsp_rpiA
COMPILER_ARGS = --target=$(TARGET).json \
    --features $(FEATURES)         \
    --release                      \
    -Z build-std=core

RUSTC_CMD   	= cargo rustc $(COMPILER_ARGS)
RUST_TEST_CMD	= cargo build --bin test_main $(COMPILER_ARGS)
DOC_CMD     	= cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  	= cargo clippy $(COMPILER_ARGS)
CHECK_CMD   	= cargo check $(COMPILER_ARGS)
OBJCOPY_CMD 	= rust-objcopy \
    --strip-all            \
    -O binary

KERNEL_ELF = target/$(TARGET)/release/$(PROJECT)
KERNEL_ELF_TEST = target/$(TARGET)/release/test

.PHONY: all $(KERNEL_ELF) $(APPLICATION) doc clippy clean readelf objdump nm check

$(KERNEL_ELF):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(KERNEL_ELF_TEST):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_TEST_CMD)

$(APPLICATION): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(APPLICATION)

$(TEST): $(KERNEL_ELF_TEST)
	@$(OBJCOPY_CMD) $(KERNEL_ELF_TEST) $(TEST)

all: $(APPLICATION)
elf: $(KERNEL_ELF)

run: $(KERNEL_BIN)
	./bin/rpi-run.py -p -t 2 $(PROJECT).bin

test: $(TEST)
	rpi-run.py $(PROJECT).bin

doc:
	$(DOC_CMD) --document-private-items --open

clippy:
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

clean:
	rm -rf target $(APPLICATION)

readelf: $(KERNEL_ELF)
	readelf --headers $(KERNEL_ELF)

objdump: $(KERNEL_ELF)
	@$(DOCKER_ELFTOOLS) $(OBJDUMP_BINARY) --disassemble --demangle $(KERNEL_ELF) | rustfilt

nm: $(KERNEL_ELF)
	@$(DOCKER_ELFTOOLS) $(NM_BINARY) --demangle --print-size $(KERNEL_ELF) | sort | rustfilt

# For rust-analyzer
check:
	@RUSTFLAGS="$(RUSTFLAGS)" $(CHECK_CMD) --message-format=json
