#!/bin/bash

apt update
apt install net-tools curl -yq

# Check if Rust is already installed
if command -v rustc > /dev/null 2>&1; then
    echo "Rust is already installed."
    exit 0
fi

# Run the installation script for Rust and Cargo without user prompts
log.info "Installing Rust and Cargo..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Update the current shell environment to use Rust and Cargo immediately
source $HOME/.cargo/env

# Confirm installation
log.done "Rust and Cargo have been installed successfully."
log.sub $(rustc --version)
log.sub $(cargo --version)
