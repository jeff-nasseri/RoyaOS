#!/bin/bash

# RoyaOS Installation Script
# This script installs RoyaOS and its dependencies

set -e

echo "=== RoyaOS Installation ==="
echo "Starting installation process..."

# Check for Rust installation
if ! command -v rustc &> /dev/null; then
    echo "Rust not found. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed."
    rustup update
fi

# Check for required dependencies
echo "Checking dependencies..."
DEPS=("git" "build-essential" "pkg-config" "libssl-dev")
for dep in "${DEPS[@]}"; do
    if ! dpkg -l | grep -q "$dep"; then
        MISSING_DEPS="$MISSING_DEPS $dep"
    fi
done

if [ ! -z "$MISSING_DEPS" ]; then
    echo "Installing missing dependencies:$MISSING_DEPS"
    sudo apt update
    sudo apt install -y $MISSING_DEPS
fi

# Clone repository if not already present
if [ ! -d "royaos" ]; then
    echo "Cloning RoyaOS repository..."
    git clone https://github.com/your-username/royaos.git
    cd royaos
else
    echo "RoyaOS repository already exists."
    cd royaos
    git pull
fi

# Build RoyaOS
echo "Building RoyaOS..."
cargo build --release

# Setup configuration
echo "Setting up configuration..."
if [ ! -f "config/config.yaml" ]; then
    cp config/config.example.yaml config/config.yaml
    echo "Created default configuration file. Please edit config/config.yaml with your settings."
fi

# Create necessary directories
echo "Creating system directories..."
mkdir -p data/memory
mkdir -p data/storage
mkdir -p logs

echo "=== Installation Complete ==="
echo "To start RoyaOS, run: cargo run --release"
echo "For more information, see the documentation in the docs/ directory."
