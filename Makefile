SHELL := /bin/bash

# Use all available CPU cores by default unless jobs are explicitly configured.
ifeq (,$(filter -j% --jobs=% --jobs% --jobserver%,$(MAKEFLAGS)))
	MAKEFLAGS += -j$(shell nproc)
endif

RISCV ?= /opt/riscv/bin/riscv32-unknown-elf-gcc
EXECUTABLE_PATH := target/debug/qrv-emu
INCLUDE := validation/include
RV32IM_TESTS := validation/rv32im-tests
QUADRISPARSE_TESTS := validation/quadrisparse-tests
TEST_BINS := bin
PROGRAM_PATH := validation/programs

BIN ?= 
INSTR ?=
PROGRAM ?=
TEST_PATH ?=
PREFIX ?=
ARGS ?=
SIZE ?=


build:
	cargo build

run:
	RUSTFLAGS=-Awarnings RUST_BACKTRACE=1 cargo run $(BIN) $(ARGS)

build-test:
	@mkdir -p $(TEST_BINS)
	$(RISCV) \
		-march=rv32im -mabi=ilp32 \
		-I $(INCLUDE) \
		-nostdlib -nostartfiles \
		-T $(INCLUDE)/link.ld \
		-o $(TEST_BINS)/$(INSTR)_test.elf \
		$(TEST_PATH)

run-test:
	@if [ -z "$(INSTR)" ]; then \
		echo "INSTR is not set. Usage: make run-test INSTR=instruction_name"; \
		exit 1; \
	fi
	$(eval TEST_PATH := $(firstword $(wildcard $(RV32IM_TESTS)/$(INSTR).S $(QUADRISPARSE_TESTS)/$(INSTR).S)))
	@if [ -z "$(TEST_PATH)" ]; then \
		echo "$(INSTR) not found in $(RV32IM_TESTS) or $(QUADRISPARSE_TESTS)"; \
		exit 1; \
	fi
	@echo "Found: $(TEST_PATH)"
	$(MAKE) build-test TEST_PATH=$(TEST_PATH)
	$(MAKE) run BIN=$(TEST_BINS)/$(INSTR)_test.elf

build-program:
	@mkdir -p $(TEST_BINS)
	$(RISCV) \
		-march=rv32im -mabi=ilp32 \
		-I $(INCLUDE) \
		-o $(TEST_BINS)/$(PROGRAM).elf \
		$(PROGRAM_PATH)/$(PROGRAM)

run-program:
	@if [ -z "$(PROGRAM)" ]; then \
		echo "PROGRAM is not set. Usage: make run-test PROGRAM=c_source_file"; \
		exit 1; \
	fi
	$(MAKE) build-program 
	$(MAKE) run BIN=$(TEST_BINS)/$(PROGRAM).elf

clean-bin:
	rm bin/*

clean:
	cargo clean
