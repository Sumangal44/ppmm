#!/bin/bash
# One-liner installer for PPM
# Usage: bash <(curl -s https://raw.githubusercontent.com/Sumangal44/python-project-manager/master/quick-install.sh)

set -e

echo "🚀 Installing PPM (Python Project Manager)..."

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "📦 Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Clone and build
git clone https://github.com/Sumangal44/python-project-manager.git
cd python-project-manager
cargo build --release

# Install
sudo cp target/release/ppm /usr/local/bin/
chmod +x /usr/local/bin/ppm

echo "✅ PPM installed successfully!"
echo "📝 Try: ppm --help"
