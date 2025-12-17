#!/bin/bash
# Build script for compiling agent_issuance to WebAssembly
#
# Prerequisites:
# - Rust toolchain with wasm32-unknown-unknown target
# - wasm-bindgen-cli
# - wasm-opt (optional, for optimization)
#
# Install prerequisites:
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli
#   cargo install wasm-opt (optional)

set -e

echo "Building agent_issuance for WebAssembly..."

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if wasm32 target is installed
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo -e "${YELLOW}Installing wasm32-unknown-unknown target...${NC}"
    rustup target add wasm32-unknown-unknown
fi

# Check if wasm-bindgen-cli is installed
if ! command -v wasm-bindgen &> /dev/null; then
    echo -e "${RED}Error: wasm-bindgen-cli is not installed${NC}"
    echo "Install it with: cargo install wasm-bindgen-cli"
    exit 1
fi

# Build mode (default: release)
BUILD_MODE="${1:-release}"
OUTPUT_DIR="pkg"

if [ "$BUILD_MODE" = "debug" ]; then
    echo "Building in DEBUG mode..."
    CARGO_FLAGS="--target wasm32-unknown-unknown --features wasm"
    PROFILE_FLAG=""
    TARGET_DIR="../target/wasm32-unknown-unknown/debug"
else
    echo "Building in RELEASE mode..."
    CARGO_FLAGS="--target wasm32-unknown-unknown --release --features wasm"
    PROFILE_FLAG="--release"
    TARGET_DIR="../target/wasm32-unknown-unknown/release"
fi

# Clean previous build (optional)
if [ "$2" = "clean" ]; then
    echo "Cleaning previous build..."
    rm -rf "$OUTPUT_DIR"
    cargo clean --target wasm32-unknown-unknown
fi

# Build the WASM module
echo "Running cargo build..."
# Set RUSTFLAGS for getrandom 0.3 WASM support
RUSTFLAGS="--cfg getrandom_backend=\"wasm_js\"" cargo build $CARGO_FLAGS

# Check if build was successful
if [ ! -f "$TARGET_DIR/agent_issuance.wasm" ]; then
    echo -e "${RED}Error: Build failed - WASM file not found${NC}"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Run wasm-bindgen
echo "Running wasm-bindgen..."
wasm-bindgen "$TARGET_DIR/agent_issuance.wasm" \
    --out-dir "$OUTPUT_DIR" \
    --target web \
    --typescript

# Optimize with wasm-opt if available
if command -v wasm-opt &> /dev/null && [ "$BUILD_MODE" = "release" ]; then
    echo "Optimizing WASM with wasm-opt..."
    wasm-opt -Oz -o "$OUTPUT_DIR/agent_issuance_bg_optimized.wasm" "$OUTPUT_DIR/agent_issuance_bg.wasm"
    mv "$OUTPUT_DIR/agent_issuance_bg_optimized.wasm" "$OUTPUT_DIR/agent_issuance_bg.wasm"
fi

# Get file size
WASM_SIZE=$(du -h "$OUTPUT_DIR/agent_issuance_bg.wasm" | cut -f1)

echo -e "${GREEN}✓ Build successful!${NC}"
echo "Output directory: $OUTPUT_DIR"
echo "WASM size: $WASM_SIZE"
echo ""
echo "Files generated:"
echo "  - agent_issuance_bg.wasm (WebAssembly binary)"
echo "  - agent_issuance.js (JavaScript bindings)"
echo "  - agent_issuance.d.ts (TypeScript definitions)"
echo ""
echo "To use in your web application:"
echo "  import init, { WasmIssuanceAgent } from './pkg/agent_issuance.js';"
echo ""
echo "To test, run: python3 -m http.server 8000"
echo "Then open: http://localhost:8000/www/index.html"
