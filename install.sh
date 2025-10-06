#!/usr/bin/env bash
# AGSi installer script
# Usage: curl -sSL https://raw.githubusercontent.com/OWNER/REPO/main/install.sh | bash

set -e

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        os="linux"
        ;;
    Darwin*)
        os="darwin"
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64)
        arch="x86_64"
        ;;
    aarch64|arm64)
        arch="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Get latest release version
REPO="${GITHUB_REPOSITORY:-yourusername/agsipls}"
VERSION="${AGSI_VERSION:-latest}"

if [ "$VERSION" = "latest" ]; then
    VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
fi

if [ -z "$VERSION" ]; then
    echo "Failed to determine latest version"
    exit 1
fi

echo "Installing AGSi $VERSION for $os-$arch..."

# Construct download URL
FILENAME="agsipls_${VERSION}_${os}_${arch}.tar.gz"
URL="https://github.com/$REPO/releases/download/$VERSION/$FILENAME"

echo "Downloading from: $URL"

# Download and extract
TMPDIR=$(mktemp -d)
cd "$TMPDIR"

if command -v curl > /dev/null; then
    curl -fsSL "$URL" -o "$FILENAME"
elif command -v wget > /dev/null; then
    wget -q "$URL" -O "$FILENAME"
else
    echo "Neither curl nor wget found. Please install one of them."
    exit 1
fi

tar -xzf "$FILENAME"

# Install to /usr/local/bin or ~/bin
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/bin"
    mkdir -p "$INSTALL_DIR"
fi

mv agsipls "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/agsipls"

# Cleanup
cd - > /dev/null
rm -rf "$TMPDIR"

echo "AGSi installed successfully to $INSTALL_DIR/agsipls"
echo ""
echo "Run 'agsipls --version' to verify the installation"

# Check if install directory is in PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo ""
    echo "⚠️  Warning: $INSTALL_DIR is not in your PATH"
    echo "Add the following to your ~/.bashrc, ~/.zshrc, or equivalent:"
    echo ""
    echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
    echo ""
fi
