# Quick Reference: GoReleaser Setup

## Creating a Release

```bash
# 1. Tag the release
git tag -a v0.1.0 -m "Release v0.1.0"

# 2. Push the tag
git push origin v0.1.0

# 3. Wait for GitHub Actions to complete
# Visit: https://github.com/OWNER/REPO/actions
```

## Files Overview

| File | Purpose |
|------|---------|
| `.goreleaser.yml` | Main configuration |
| `.github/workflows/release.yml` | Release automation |
| `.github/workflows/goreleaser-test.yml` | Config validation |
| `install.sh` | User installation script |
| `GORELEASER.md` | Full documentation |

## Testing Locally

```bash
# Check config is valid
goreleaser check

# Build for current platform
goreleaser build --snapshot --clean --single-target

# Simulate full release (no publish)
goreleaser release --snapshot --clean
```

## User Installation Methods

### Quick Install (Linux/macOS)
```bash
curl -sSL https://raw.githubusercontent.com/OWNER/REPO/main/install.sh | bash
```

### Homebrew (after setup)
```bash
brew install OWNER/tap/agsi
```

### Manual Download
```bash
# Download from https://github.com/OWNER/REPO/releases/latest
tar -xzf agsi_VERSION_OS_ARCH.tar.gz
sudo mv agsi /usr/local/bin/
```

### Package Managers
```bash
# Debian/Ubuntu
sudo dpkg -i agsi_VERSION_amd64.deb

# Red Hat/Fedora
sudo rpm -i agsi-VERSION-1.x86_64.rpm

# Alpine
sudo apk add --allow-untrusted agsi_VERSION_linux_amd64.apk
```

## Release Artifacts

Each release includes:
- `agsi_VERSION_Linux_x86_64.tar.gz`
- `agsi_VERSION_Linux_aarch64.tar.gz`
- `agsi_VERSION_Darwin_x86_64.tar.gz`
- `agsi_VERSION_Darwin_aarch64.tar.gz`
- `agsi_VERSION_Windows_x86_64.zip`
- `checksums.txt`
- `agsi_VERSION_amd64.deb`
- `agsi-VERSION-1.x86_64.rpm`
- `agsi_VERSION_linux_amd64.apk`
- Source code (zip & tar.gz)

## Conventional Commits

Use these prefixes for automatic changelog generation:

- `feat:` → New Features
- `fix:` → Bug Fixes
- `perf:` → Performance Improvements
- `docs:` → Documentation
- `ci:`, `chore:`, `test:` → Filtered out

Example:
```bash
git commit -m "feat: add material validation"
git commit -m "fix: correct geometry serialization"
```

## Troubleshooting

### Build fails
```bash
# Check workflow logs
gh run list
gh run view RUN_ID --log-failed

# Test locally
cargo build --release --target TARGET
```

### GoReleaser config invalid
```bash
goreleaser check
```

### Missing artifacts
Check that all binaries are in `dist/` structure:
```
dist/
  linux_amd64/agsi
  linux_arm64/agsi
  darwin_amd64/agsi
  darwin_arm64/agsi
  windows_amd64/agsi.exe
```

## Environment Variables

| Variable | Required | Purpose |
|----------|----------|---------|
| `GITHUB_TOKEN` | Yes | Auto-provided by Actions |
| `HOMEBREW_TAP_GITHUB_TOKEN` | No | Homebrew tap updates |
| `GITHUB_REPOSITORY` | Yes | Auto-provided by Actions |
| `GITHUB_REPOSITORY_OWNER` | Yes | Auto-provided by Actions |

## Permissions Required

In `.github/workflows/release.yml`:
```yaml
permissions:
  contents: write    # Create releases
  packages: write    # Publish packages
```

## Workflow Triggers

```yaml
on:
  push:
    tags:
      - 'v*'  # Any tag starting with 'v'
```

## Common Commands

```bash
# List releases
gh release list

# View latest release
gh release view --web

# Download asset
gh release download v0.1.0 -p 'agsi_*_Linux_x86_64.tar.gz'

# Delete release (testing)
gh release delete v0.1.0-test --yes
git push --delete origin v0.1.0-test
```

## Quick Start Checklist

- [ ] Files created (`.goreleaser.yml`, workflows, `install.sh`)
- [ ] GoReleaser config valid (`goreleaser check`)
- [ ] YAML files valid (no syntax errors)
- [ ] Git remote configured
- [ ] Test with `v0.0.1-test` tag
- [ ] Verify artifacts in release
- [ ] Test install script
- [ ] Update README with install instructions
- [ ] Create official `v0.1.0` release

## Support

- Issue: Check GitHub Actions logs
- Docs: https://goreleaser.com/
- Examples: `.goreleaser.yml` comments
- Full guide: `GORELEASER.md`
