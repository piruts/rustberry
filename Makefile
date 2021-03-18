## SPDX-License-Identifier: MIT OR Apache-2.0
##
## Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>
## 
## Edited 2021 by Flynn Dreilinger <flynnd@stanford.edu> and Ashish Rao <aprao@stanford.edu>

PROJECT           = rustberry
TARGET            = armv6kz-none-eabi
BIN        		  = $(PROJECT).bin
TEST_BIN		  = test-$(PROJECT).bin
OBJDUMP_BINARY    = arm-none-eabi-objdump
NM_BINARY         = arm-none-eabi-nm
READELF_BINARY	  = arm-none-eabi-readelf
LINKER_FILE       = src/bsp/raspberrypi/link.ld
PROFILE			  = debug

# Export for build.rs
export LINKER_FILE

RUSTFLAGS          = -C link-arg=-T$(LINKER_FILE)
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) #-D warnings -D missing_docs

FEATURES      = bsp_rpiA
COMPILER_ARGS = --target=$(TARGET).json \
    --features $(FEATURES)         \
    -Z build-std=core,alloc 	   \
	-Z panic-abort-tests 

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
TEST_CMD    = cargo test --no-run $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
CHECK_CMD   = cargo check $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary 

ELF = target/$(TARGET)/$(PROFILE)/$(PROJECT)

TEST_ELF = target/$(TARGET)/$(PROFILE)/deps/$(PROJECT)-*[!.]?

.PHONY: all $(ELF) $(TEST_ELF) $(BIN) $(TEST_BIN) doc clippy clean readelf objdump nm check

always_clean_and_format: clean
	cargo fmt

$(ELF):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(TEST_ELF):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(TEST_CMD)
	ln -s target/$(TARGET)/$(PROFILE)/deps/$(PROJECT)-*[!.]? target/$(TARGET)/$(PROFILE)/test-$(PROJECT)

$(BIN): $(ELF)
	@$(OBJCOPY_CMD) $(ELF) $(BIN)

$(TEST_BIN): $(TEST_ELF)
	@$(OBJCOPY_CMD) $(TEST_ELF) $(TEST_BIN)

all: $(BIN)
elf: $(ELF)

test_elf: $(TEST_ELF)

run: always_clean_and_format $(BIN)
	./bin/rpi-run.py -p -t 2 $(BIN)

test: always_clean_and_format $(TEST_BIN)
	./bin/rpi-run.py -p -t 2 $(TEST_BIN)

doc:
	$(call colorecho, "\nGenerating docs")
	@$(DOC_CMD) --document-private-items --open

clippy:
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

clean:
	rm -rf target $(PROJECT).* $(TEST_BIN)

readelf: $(ELF)
	$(call colorecho, "\nLaunching readelf")
	$(READELF_BINARY) --headers $(ELF) > $(PROJECT).readelf

objdump: $(ELF)
	$(call colorecho, "\nLaunching objdump")
	$(OBJDUMP_BINARY) --disassemble --demangle \
                --section .text   \
                --section .rodata \
                --section .got    \
                $(ELF) | rustfilt > $(PROJECT).as

nm: $(ELF)
	$(call colorecho, "\nLaunching nm")
	$(NM_BINARY) --demangle --print-size $(ELF) | sort | rustfilt > $(PROJECT).nm

# For rust-analyzer
check:
	@RUSTFLAGS="$(RUSTFLAGS)" $(CHECK_CMD) --message-format=json
