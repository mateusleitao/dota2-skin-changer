#!/usr/bin/env bash
set -euo pipefail

echo "=== Dota 2 Skin Changer — Development Setup ==="

# Check Rust
if ! command -v cargo &>/dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi
echo "Rust: $(rustc --version)"

# Check Node
if ! command -v node &>/dev/null; then
    echo "ERROR: Node.js not found. Install Node.js 20+ first."
    exit 1
fi
echo "Node: $(node --version)"

# Check protoc
if ! command -v protoc &>/dev/null; then
    echo "Installing protoc..."
    if [[ "$OSTYPE" == "darwin"* ]]; then
        brew install protobuf
    else
        echo "ERROR: protoc not found. Install protobuf compiler."
        exit 1
    fi
fi
echo "protoc: $(protoc --version)"

# Install npm dependencies
echo "Installing npm dependencies..."
npm install

# Build Rust workspace
echo "Building Rust workspace..."
cargo build --workspace --lib

# Run tests
echo "Running Rust tests..."
cargo test --workspace

echo "Running frontend tests..."
npx vitest run

echo ""
echo "=== Setup complete! ==="
echo "  cargo test --workspace    # Rust tests"
echo "  npm test                  # Frontend tests"
echo "  npm run dev               # Dev server"
