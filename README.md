# Libcpolars

Libcpolars is a small layer that wraps the functionalities of [Polars](https://www.pola.rs/) with a C-FFI-friendly API.

## Installation

To install, ensure that you have the Rust compiler and `make`. Then follow these steps:

1. Open the terminal and run `make`.
2. After running `make`, run `make install` (or `sudo make install`) to install the shared library. By default, it will be installed in `/usr/local/lib/`. However, you can specify an alternative path by using the `PREFIX=/your/alternative/path/` command.

## Usage

This library is designed to be used by other languages that support C-FFI but do not have official support for Polars.
