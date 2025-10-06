# Installation Section for README

Add this section to your README.md to help users install AGSi:

---

## Installation

AGSi provides multiple installation methods to suit your workflow.

### Quick Install (Recommended)

**Linux & macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/OWNER/REPO/main/install.sh | bash
```

This script automatically:
- Detects your OS and architecture
- Downloads the latest release
- Installs to `/usr/local/bin` or `~/bin`
- Makes the binary executable

### Homebrew

**macOS & Linux:**
```bash
brew install OWNER/tap/agsi
```

### Package Managers

**Debian/Ubuntu:**
```bash
# Download the .deb from releases
wget https://github.com/OWNER/REPO/releases/latest/download/agsi_VERSION_amd64.deb
sudo apt install ./agsi_VERSION_amd64.deb
```

**Fedora/RHEL/CentOS:**
```bash
# Download the .rpm from releases
wget https://github.com/OWNER/REPO/releases/latest/download/agsi-VERSION-1.x86_64.rpm
sudo dnf install agsi-VERSION-1.x86_64.rpm
```

**Alpine:**
```bash
# Download the .apk from releases
wget https://github.com/OWNER/REPO/releases/latest/download/agsi_VERSION_linux_amd64.apk
sudo apk add --allow-untrusted agsi_VERSION_linux_amd64.apk
```

### Manual Installation

1. Download the appropriate archive for your platform from the [releases page](https://github.com/OWNER/REPO/releases/latest):
   - `agsi_VERSION_Linux_x86_64.tar.gz` - Linux (Intel/AMD)
   - `agsi_VERSION_Linux_aarch64.tar.gz` - Linux (ARM64)
   - `agsi_VERSION_Darwin_x86_64.tar.gz` - macOS (Intel)
   - `agsi_VERSION_Darwin_aarch64.tar.gz` - macOS (Apple Silicon)
   - `agsi_VERSION_Windows_x86_64.zip` - Windows

2. Extract the archive:
   ```bash
   # Linux/macOS
   tar -xzf agsi_VERSION_*.tar.gz
   
   # Windows (PowerShell)
   Expand-Archive agsi_VERSION_Windows_x86_64.zip
   ```

3. Move the binary to a directory in your PATH:
   ```bash
   # Linux/macOS
   sudo mv agsi /usr/local/bin/
   
   # Windows
   # Move agsi.exe to C:\Program Files\agsi\ and add to PATH
   ```

4. Verify the installation:
   ```bash
   agsi --version
   ```

### From Source

**Prerequisites:**
- Rust 1.75 or later
- Cargo

```bash
# Clone the repository
git clone https://github.com/OWNER/REPO.git
cd REPO

# Build and install
cargo install --path crates/agsi

# Or build without installing
cargo build --release
# Binary will be at: target/release/agsi
```

### Verification

After installation, verify it's working:

```bash
agsi --version
agsi --help
```

### Updating

**Using install script:**
```bash
curl -sSL https://raw.githubusercontent.com/OWNER/REPO/main/install.sh | bash
```

**Using Homebrew:**
```bash
brew upgrade agsi
```

**Using package managers:**
```bash
# Debian/Ubuntu
sudo apt update && sudo apt upgrade agsi

# Fedora/RHEL
sudo dnf upgrade agsi
```

**Manual update:**
Download the latest version and replace your existing binary.

### Uninstallation

**Install script / Manual:**
```bash
# Find the binary location
which agsi

# Remove it
sudo rm $(which agsi)
```

**Homebrew:**
```bash
brew uninstall agsi
```

**Package managers:**
```bash
# Debian/Ubuntu
sudo apt remove agsi

# Fedora/RHEL
sudo dnf remove agsi

# Alpine
sudo apk del agsi
```

### Supported Platforms

| Platform | Architecture | Status |
|----------|--------------|--------|
| Linux    | x86_64       | ✅     |
| Linux    | ARM64        | ✅     |
| macOS    | Intel        | ✅     |
| macOS    | Apple Silicon| ✅     |
| Windows  | x86_64       | ✅     |

### Checksums

All release artifacts include SHA256 checksums. Download `checksums.txt` from the release page to verify:

```bash
# Download checksums
wget https://github.com/OWNER/REPO/releases/latest/download/checksums.txt

# Verify your download
sha256sum -c checksums.txt 2>&1 | grep agsi_.*$(uname -s | tr '[:upper:]' '[:lower:]')
```

### Troubleshooting

**"command not found" after installation:**
- Ensure the installation directory is in your PATH
- Try `echo $PATH` to see your current PATH
- Add to PATH: `export PATH="/usr/local/bin:$PATH"` (add to ~/.bashrc or ~/.zshrc)

**Permission denied:**
- Make sure the binary is executable: `chmod +x /path/to/agsi`
- Or use sudo for system-wide installation

**"agsipls" can't be opened (macOS):**
- Run: `xattr -d com.apple.quarantine /path/to/agsi`
- Or go to System Preferences → Security & Privacy → Allow

**Architecture mismatch:**
- Ensure you downloaded the correct version for your system
- Check your architecture: `uname -m`
- x86_64 = Intel/AMD 64-bit
- aarch64/arm64 = ARM 64-bit

For more help, see the [troubleshooting guide](docs/TROUBLESHOOTING.md) or [open an issue](https://github.com/OWNER/REPO/issues).

---

## Badges (Optional)

Add these badges to the top of your README:

```markdown
[![Release](https://img.shields.io/github/v/release/OWNER/REPO)](https://github.com/OWNER/REPO/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/OWNER/REPO/total)](https://github.com/OWNER/REPO/releases)
[![License](https://img.shields.io/github/license/OWNER/REPO)](LICENSE)
[![CI](https://github.com/OWNER/REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/OWNER/REPO/actions/workflows/ci.yml)
```

Remember to replace `OWNER` and `REPO` with your actual GitHub username and repository name!
