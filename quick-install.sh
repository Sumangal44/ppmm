#!/bin/bash
# One-liner installer for PPMM
# Usage: bash <(curl -s https://raw.githubusercontent.com/Sumangal44/ppmm/master/quick-install.sh)

set -e

echo "🚀 Installing PPMM (Python Project Manager)..."

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Clone and build
git clone https://github.com/Sumangal44/ppmm.git
cd ppmm
cargo build --release

# Install
sudo cp target/release/ppmm /usr/local/bin/
chmod +x /usr/local/bin/ppmm

echo "✅ PPMM installed successfully!"
echo "📝 Try: ppmm --help"
