#!/bin/bash

apt update
apt install net-tools curl -yq

# Check if Rust is already installed
if command -v rustc > /dev/null 2>&1; then
    echo "Rust is already installed."
    exit 0
fi

# Run the installation script for Rust and Cargo without user prompts
echo "Installing Rust and Cargo..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Update the current shell environment to use Rust and Cargo immediately
source $HOME/.cargo/env

# Confirm installation
echo "Rust and Cargo have been installed successfully."
rustc --version
cargo --version

exit 0