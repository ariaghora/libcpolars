ifeq ($(shell uname), Darwin)
	LIB_EXT := dylib
else ifeq ($(OS),Windows_NT)
	LIB_EXT := dll
else
	LIB_EXT := so
endif

INSTALL_DIR := /usr/local/lib/

ifeq ($(OS),Windows_NT)
	INSTALL_DIR := C:\\Windows\\System32\\
endif
 
all:
	@cargo build -r

install:
	@cp target/release/libcpolars.$(LIB_EXT) $(or $(PREFIX)$(INSTALL_DIR), $(INSTALL_DIR))

gen:
	@cbindgen -q --output libcpolars.h