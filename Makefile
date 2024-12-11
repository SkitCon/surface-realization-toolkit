# Variables
CARGO := cargo

# Default target
all: build

build:
	$(CARGO) build --release

# Build the Python library
pylib:
	maturin develop --release

clean:
	cargo clean
