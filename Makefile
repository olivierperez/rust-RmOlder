.PHONY=all

# Root of the project
ROOT = $(dir $(firstword $(MAKEFILE_LIST)))

# Path to rustc executable
RUSTC ?= rustc
RUSTDOC ?= rustdoc

# Flags to pass rustc
RUSTC_FLAGS ?= -O

# Target directory
TARGET ?= $(ROOT)target

# Source directory
SOURCE = src

all: clean tests doc build run
clean:
	mkdir -p $(TARGET)/bin $(TARGET)/test

tests: $(SOURCE) | $(TARGET)
	$(RUSTC) $(SOURCE)/rmolder.rs --test --out-dir $(TARGET)/test
	$(TARGET)/test/rmolder

doc: $(SOURCE)
	$(RUSTDOC) $(SOURCE)/rmolder.rs

build: $(SOURCE)
	$(RUSTC) --out-dir $(TARGET)/bin $(SOURCE)/rmolder.rs
	$(RUSTC) --out-dir $(TARGET)/bin $(SOURCE)/main.rs -L $(TARGET)/bin

run: $(TARGET)
	$(TARGET)/bin/main -d test -a 1000000 --dry

