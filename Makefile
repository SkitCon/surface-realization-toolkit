# Makefile to build and run the FST generator

# Variables
CARGO := cargo
TARGET := generate_fst

# Default target
all: build

# Build the Rust project
build:
	$(CARGO) build --release

# Run the Rust project with arguments
run: build
	./target/release/$(TARGET) input.txt output.fst

# Clean the build files
clean:
	$(CARGO) clean
