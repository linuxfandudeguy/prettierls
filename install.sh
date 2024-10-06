#!/bin/bash

# Function to check if Rust is installed
check_rust_installed() {
    if command -v rustc &> /dev/null; then
        echo "Rust is already installed."
        return 0  # Rust is installed
    else
        return 1  # Rust is not installed
    fi
}

# Function to install Rust
install_rust() {
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    echo "Rust installation complete. Please restart your terminal or run 'source $HOME/.cargo/env'."
}

# Check if Rust is installed
if ! check_rust_installed; then
    read -p "Rust is not installed. Would you like to install it? (y/n): " install_rust_choice
    if [[ "$install_rust_choice" =~ ^[Yy]$ ]]; then
        install_rust
    else
        echo "You need Rust installed to proceed. Exiting."
        exit 1
    fi
fi

# Prompt for downloading prettierls binary
read -p "Do you want to install the prettierls binary? (y/n): " install_prettierls_choice
if [[ "$install_prettierls_choice" =~ ^[Yy]$ ]]; then
    # Download prettierls binary
    echo "Downloading prettierls binary..."
    curl -LO https://linuxfandudeguy.github.io/prettierls/bin/prettierls

    # Make the binary executable
    chmod +x prettierls

    # Move the binary to a directory in PATH
    sudo mv prettierls /usr/local/bin/

    echo "Installation complete! You can now use 'prettierls' from the command line."
else
    echo "Skipping the installation of prettierls."
fi
