.PHONY: all clean build install dev-shell

# Default target - builds the extension ready for installation
all: build

# Build the WASM extension
build:
	@echo "Building Zed Shell Command Extension..."
	nix develop -c cargo build --release --target wasm32-wasip1
	@echo "Extension built successfully!"
	@echo ""
	@echo "To install in Zed:"
	@echo "1. Open Zed"
	@echo "2. Go to Extensions (Cmd+Shift+X)"
	@echo "3. Click 'Install Dev Extension'"
	@echo "4. Select this directory: $(PWD)"

# Clean build artifacts
clean:
	nix develop -c cargo clean
	rm -rf target/

# Enter the Nix development shell
dev-shell:
	nix develop

# Run tests
test:
	nix develop -c cargo test

# Format code
fmt:
	nix develop -c cargo fmt

# Lint code
lint:
	nix develop -c cargo clippy -- -D warnings

# Check if everything is ready
check: fmt lint test build
	@echo "All checks passed!"

# Watch for changes and rebuild (requires cargo-watch)
watch:
	nix develop -c cargo watch -x "build --release --target wasm32-wasip1"