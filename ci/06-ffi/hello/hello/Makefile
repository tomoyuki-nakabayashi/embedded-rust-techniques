PREFIX ?= .
LIB_DIR ?= $(PREFIX)/lib

TARGET_PATH := target/thumbv7m-none-eabi/debug
RUST_TARGET := librustlib.a
RUST_LIB := librust.a
RUST_FILES := $(shell find src/ -type f -name "*.rs") Cargo.toml
C_BINDINGS := rustlib.h

all: $(LIB_DIR)/$(RUST_LIB) $(LIB_DIR)/$(C_BINDINGS)

$(LIB_DIR)/$(RUST_LIB): $(RUST_FILES)
	mkdir -p $(LIB_DIR)
	cargo build
	cp $(TARGET_PATH)/$(RUST_TARGET) $(LIB_DIR)/$(RUST_LIB)
	cargo objcopy -- --weaken $(LIB_DIR)/$(RUST_LIB)

$(LIB_DIR)/$(C_BINDINGS): $(RUST_FILES)
	cbindgen src/lib.rs -l c -o $(LIB_DIR)/$(C_BINDINGS)

clean:
	rm -rf $(LIB_DIR)/$(RUST_LIB) $(LIB_DIR)/$(C_BINDINGS)
	cargo clean
