## SPDX-License-Identifier: MIT OR Apache-2.0
##
## Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

PROJECT           = project
TARGET            = armv6kz-none-eabi
KERNEL_BIN        = kernel.img
OBJDUMP_BINARY    = aarch32-none-elf-objdump
NM_BINARY         = aarch32-none-elf-nm
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

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
CHECK_CMD   = cargo check $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary

KERNEL_ELF = target/$(TARGET)/release/$(PROJECT)

.PHONY: all $(KERNEL_ELF) $(KERNEL_BIN) doc qemu clippy clean readelf objdump nm check

all: $(KERNEL_BIN)

$(KERNEL_ELF):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)

doc:
	$(DOC_CMD) --document-private-items --open

clippy:
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

clean:
	rm -rf target $(KERNEL_BIN)

readelf: $(KERNEL_ELF)
	readelf --headers $(KERNEL_ELF)

objdump: $(KERNEL_ELF)
	@$(DOCKER_ELFTOOLS) $(OBJDUMP_BINARY) --disassemble --demangle $(KERNEL_ELF) | rustfilt

nm: $(KERNEL_ELF)
	@$(DOCKER_ELFTOOLS) $(NM_BINARY) --demangle --print-size $(KERNEL_ELF) | sort | rustfilt

# For rust-analyzer
check:
	@RUSTFLAGS="$(RUSTFLAGS)" $(CHECK_CMD) --message-format=json
