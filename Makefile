ERROR ?= "\e[31m"
WARN ?= "\e[33m"
NORMAL ?= "\e[32m"
RESET ?= "\e[0m"

APPS := $(shell find apps -maxdepth 1 -mindepth 1 -type d | sed 's|apps/||')
ELF_PATH := bin

# default mode: relaease
# default target: riscv64
# default lib: glibc
MODE ?= release
TARGET ?= riscv64gc-unknown-none-elf
TARGET_DIR := $(shell pwd)/target/$(TARGET)/$(MODE)
CARGO_ARGS := --target $(TARGET) --release

export TESTCASES ?= ltp

all: build

build:
	@echo -e $(NORMAL)"Building apps..."$(RESET)
	@rm -rf $(ELF_PATH)
	@mkdir -p $(ELF_PATH)
	@cd apps && $(foreach dir, $(APPS), (cd $(dir) && cargo build $(CARGO_ARGS) && cd ..);)
# special compile for run_tests
	@cd apps/run_tests && cargo build --features "$(TESTCASES)" $(CARGO_ARGS)
	@echo -e $(NORMAL)"Apps build finished:"$(RESET)
	@$(foreach dir, $(APPS), echo -e "\t"$(NORMAL)$(dir)$(RESET);)
	@$(foreach dir, $(APPS), cp $(TARGET_DIR)/$(dir) $(ELF_PATH);)

vendor:
	@cargo clean
	@rm -rf vendor
	@cargo vendor

asm:
	@echo -e $(NORMAL)"Generating User Assembly..."$(RESET)
	@$(foreach dir, $(APPS), (cd apps/$(dir) && cargo objdump $(CARGO_ARGS) --quiet -- -d > $(ROOT)/log/$(dir).asm);)
	@echo -e $(NORMAL)"Assembly saved to:"$(RESET)
	@$(foreach dir, $(APPS), (echo -e $(NORMAL)"\t"$(ROOT)/log/$(dir).asm$(RESET));)

.PHONY: all build vendor
