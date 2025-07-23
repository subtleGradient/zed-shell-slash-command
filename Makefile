.PHONY: all clean build install dev-shell

# Default target - builds the extension ready for installation
all: build

# Build the WASM extension
build:
	@echo "Building Zed Shell Command Extension..."
	cargo build --release --target wasm32-wasi
	@echo "Extension built successfully!"
	@echo ""
	@echo "To install in Zed:"
	@echo "1. Open Zed"
	@echo "2. Go to Extensions (Cmd+Shift+X)"
	@echo "3. Click 'Install Dev Extension'"
	@echo "4. Select this directory: $(PWD)"

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/

# Enter the Nix development shell
dev-shell:
	nix develop

# Run tests
test:
	cargo test

# Format code
fmt:
	cargo fmt

# Lint code
lint:
	cargo clippy -- -D warnings

# Check if everything is ready
check: fmt lint test build
	@echo "All checks passed!"

# Watch for changes and rebuild (requires cargo-watch)
watch:
	cargo watch -x "build --release --target wasm32-wasi"