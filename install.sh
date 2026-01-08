#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  PPM (Python Project Manager) Installer${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Check OS
OS_TYPE=$(uname -s)
echo -e "${YELLOW}Detected OS: $OS_TYPE${NC}\n"

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
echo -e "${BLUE}Checking prerequisites...${NC}"

# Check Python
if command_exists python3; then
    PYTHON_VERSION=$(python3 --version 2>&1 | awk '{print $2}')
    echo -e "${GREEN}✓ Python 3 found: $PYTHON_VERSION${NC}"
else
    echo -e "${RED}✗ Python 3 is required but not installed${NC}"
    echo "   Install Python 3 from https://www.python.org/"
    exit 1
fi

# Check Git
if command_exists git; then
    GIT_VERSION=$(git --version | awk '{print $3}')
    echo -e "${GREEN}✓ Git found: $GIT_VERSION${NC}"
else
    echo -e "${RED}✗ Git is required but not installed${NC}"
    exit 1
fi

# Check Rust
if command_exists cargo; then
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    echo -e "${GREEN}✓ Rust found: $RUST_VERSION${NC}"
else
    echo -e "${YELLOW}⚠ Rust is required but not installed${NC}"
    echo -e "${YELLOW}Installing Rust...${NC}\n"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo -e "${GREEN}✓ Rust installed successfully${NC}"
fi

echo ""

# Clone or navigate to repo
REPO_DIR="python-project-manager"

if [ ! -d "$REPO_DIR" ]; then
    echo -e "${BLUE}Cloning repository...${NC}"
    git clone https://github.com/Sumangal44/python-project-manager.git
    cd "$REPO_DIR"
else
    echo -e "${BLUE}Repository already exists, updating...${NC}"
    cd "$REPO_DIR"
    git pull origin master
fi

echo ""

# Build the project
echo -e "${BLUE}Building PPM (this may take a few moments)...${NC}"
cargo build --release

echo ""

# Install globally
echo -e "${BLUE}Installing PPM globally...${NC}"

BINARY_PATH="target/release/ppm"
INSTALL_DIR="/usr/local/bin"

if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}✗ Build failed: Binary not found at $BINARY_PATH${NC}"
    exit 1
fi

# Try to install with sudo if needed
if [ -w "$INSTALL_DIR" ]; then
    cp "$BINARY_PATH" "$INSTALL_DIR/ppm"
    echo -e "${GREEN}✓ Installed to $INSTALL_DIR/ppm${NC}"
else
    echo -e "${YELLOW}Requires sudo to install to $INSTALL_DIR${NC}"
    sudo cp "$BINARY_PATH" "$INSTALL_DIR/ppm"
    echo -e "${GREEN}✓ Installed to $INSTALL_DIR/ppm${NC}"
fi

# Make executable
chmod +x "$INSTALL_DIR/ppm"

echo ""

# Verify installation
echo -e "${BLUE}Verifying installation...${NC}"
if command_exists ppm; then
    PPM_VERSION=$(ppm --version)
    echo -e "${GREEN}✓ PPM installed successfully${NC}"
    echo -e "${GREEN}  Version: $PPM_VERSION${NC}"
else
    echo -e "${YELLOW}⚠ PPM not found in PATH${NC}"
    echo -e "${YELLOW}  Try adding /usr/local/bin to your PATH or restart your terminal${NC}"
fi

echo ""

# Show quick start guide
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}Installation Complete!${NC}"
echo -e "${BLUE}========================================${NC}\n"

echo -e "${YELLOW}Quick Start:${NC}"
echo -e "  ${BLUE}Create new project:${NC}"
echo -e "    ppm new my_project -v 1.0.0 -d 'My project' -g"
echo ""
echo -e "  ${BLUE}Initialize existing project:${NC}"
echo -e "    cd my_project && ppm init my_project"
echo ""
echo -e "  ${BLUE}View all commands:${NC}"
echo -e "    ppm --help"
echo ""

echo -e "${BLUE}Documentation:${NC}"
echo -e "  https://github.com/Sumangal44/python-project-manager"
echo ""
