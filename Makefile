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
LINKER_FILE       = src/bsp/raspberrypi/link.ld

# Export for build.rs
export LINKER_FILE

RUSTFLAGS          = -C link-arg=-T$(LINKER_FILE)
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) #-D warnings -D missing_docs

FEATURES      = bsp_rpiA
COMPILER_ARGS = --target=$(TARGET).json \
    --features $(FEATURES)         \
    --release                      \
    -Z build-std=core,alloc

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
TEST_CMD    = cargo test --verbose --no-run --message-format=json $(COMPILER_ARGS)
DOC_CMD     = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
CHECK_CMD   = cargo check $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy \
    --strip-all            \
    -O binary

ELF = target/$(TARGET)/release/$(PROJECT)

TEST_ELF = target/$(TARGET)/release/deps/$(PROJECT)-*[!.]?

.PHONY: all $(ELF) $(TEST_ELF) $(BIN) $(TEST_BIN) doc clippy clean readelf objdump nm check

always_clean_and_format: clean
	cargo fmt

$(ELF):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(TEST_ELF):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(TEST_CMD)
	ln -s target/$(TARGET)/release/deps/$(PROJECT)-*[!.]? target/$(TARGET)/release/test-$(PROJECT)

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
	$(DOC_CMD) --document-private-items --open

clippy:
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

clean:
	rm -rf target $(BIN) $(TEST_BIN)

readelf: $(ELF)
	readelf --headers $(ELF)

objdump: $(ELF)
	@$(DOCKER_ELFTOOLS) $(OBJDUMP_BINARY) --disassemble --demangle $(ELF) | rustfilt

nm: $(ELF)
	@$(DOCKER_ELFTOOLS) $(NM_BINARY) --demangle --print-size $(ELF) | sort | rustfilt

# For rust-analyzer
check:
	@RUSTFLAGS="$(RUSTFLAGS)" $(CHECK_CMD) --message-format=json