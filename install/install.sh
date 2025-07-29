#!/bin/bash

set -e

REPO="RafaelJohn9/gh-templates"
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="gh-templates"

# Get latest version from GitHub API using only curl and sed/awk
get_latest_version() {
    echo "Fetching latest version..."
    VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest | \
              grep '"tag_name"' | \
              sed -E 's/.*"tag_name": "([^"]+)".*/\1/')
    
    if [ -z "$VERSION" ]; then
        echo "Error: Could not fetch latest version"
        exit 1
    fi
    echo "Latest version: $VERSION"
}

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux*)
            if command -v ldd >/dev/null 2>&1; then
                if ldd --version 2>&1 | grep -q musl; then
                    TARGET="x86_64-unknown-linux-musl"
                else
                    if [ "$ARCH" = "aarch64" ]; then
                        TARGET="aarch64-unknown-linux-gnu"
                    else
                        TARGET="x86_64-unknown-linux-gnu"
                    fi
                fi
            fi
            ;;
        Darwin*)
            if [ "$ARCH" = "arm64" ]; then
                TARGET="aarch64-apple-darwin"
            else
                TARGET="x86_64-apple-darwin"
            fi
            ;;
        *)
            echo "Unsupported OS: $OS"
            exit 1
            ;;
    esac
    echo "Detected platform: $TARGET"
}

# Download the binary
download_binary() {
    URL="https://github.com/$REPO/releases/download/$VERSION/gh-templates-$TARGET"
    echo "Downloading $BINARY_NAME for $TARGET..."
    
    if command -v wget >/dev/null 2>&1; then
        wget -qO "$BINARY_NAME" "$URL"
    elif command -v curl >/dev/null 2>&1; then
        curl -sLo "$BINARY_NAME" "$URL"
    else
        echo "Error: Neither wget nor curl found"
        exit 1
    fi
    
    chmod +x "$BINARY_NAME"
}

# Install the binary
install_binary() {
    mkdir -p "$INSTALL_DIR"
    mv "$BINARY_NAME" "$INSTALL_DIR/"
    echo "Installed $BINARY_NAME to $INSTALL_DIR"
}

# Add to PATH notice
add_to_path_notice() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo -e "\033[1;33mPlease add $INSTALL_DIR to your PATH:\033[0m"
        echo -e "   \033[1;32mexport PATH=\"\$HOME/.local/bin:\$PATH\"\033[0m"
        echo -e "   \033[0;36mYou can add this line to your shell config (~/.bashrc, ~/.zshrc, etc.)\033[0m"
    else
        echo "Installation complete! Run 'gh-templates --help' to get started."
    fi
}

main() {
    get_latest_version
    detect_platform
    download_binary
    install_binary
    add_to_path_notice
}

main