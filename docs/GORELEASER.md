# GoReleaser Integration Guide

This project uses [GoReleaser](https://goreleaser.com/) to automate the release process, even though it's a Rust project. This provides several benefits:

## Features

- **Automated releases** on Git tag push
- **Multi-platform binaries** (Linux, macOS, Windows)
- **Multiple architectures** (x86_64, ARM64)
- **Archive creation** (tar.gz, zip)
- **Checksums** for all artifacts
- **Homebrew tap** support
- **Linux packages** (deb, rpm, apk)
- **Automatic changelog** generation

## How It Works

The release process is split into two jobs:

1. **Build Job**: Compiles Rust binaries for all target platforms using the matrix strategy
2. **Release Job**: Uses GoReleaser to package, create release notes, and publish

This approach combines Rust's excellent cross-compilation with GoReleaser's release management.

## Configuration Files

- `.goreleaser.yml` - GoReleaser configuration
- `.github/workflows/release.yml` - Release workflow
- `.github/workflows/goreleaser-test.yml` - Tests GoReleaser config
- `install.sh` - Installation script for users

## Creating a Release

1. Ensure all changes are committed and pushed
2. Update `CHANGELOG.md` if not using conventional commits
3. Create and push a tag:
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```
4. GitHub Actions will automatically build and release

## Local Testing

Test the GoReleaser configuration locally:

```bash
# Check configuration
goreleaser check

# Build snapshot (doesn't publish)
goreleaser build --snapshot --clean --single-target

# Full release test (doesn't publish)
goreleaser release --snapshot --clean
```

## Homebrew Tap

To enable Homebrew tap publishing:

1. Create a repository named `homebrew-tap` in your GitHub account
2. Add a `HOMEBREW_TAP_GITHUB_TOKEN` secret to your repository settings
3. The formula will be automatically pushed on release

Users can then install with:
```bash
brew install yourusername/tap/agsi
```

## Customization

Edit `.goreleaser.yml` to:
- Add/remove target platforms
- Customize archive contents
- Modify changelog format
- Add Docker image builds
- Configure announcements

## Conventional Commits

For automatic changelog generation, use conventional commit format:

- `feat:` - New features
- `fix:` - Bug fixes
- `perf:` - Performance improvements
- `docs:` - Documentation changes
- `ci:` - CI/CD changes
- `chore:` - Maintenance tasks

Example:
```bash
git commit -m "feat: add validation for material properties"
git commit -m "fix: correct geometry serialization"
```

## Binary Naming Convention

Binaries follow this pattern:
- `agsi_VERSION_PLATFORM_ARCH.tar.gz`
- `agsi_VERSION_PLATFORM_ARCH.zip` (Windows)

Examples:
- `agsi_v0.1.0_linux_x86_64.tar.gz`
- `agsi_v0.1.0_darwin_arm64.tar.gz`
- `agsi_v0.1.0_windows_x86_64.zip`

## Quick Install Script

Users can install with:
```bash
curl -sSL https://raw.githubusercontent.com/OWNER/REPO/main/install.sh | bash
```

The script:
- Detects OS and architecture
- Downloads the latest release
- Extracts to `/usr/local/bin` or `~/bin`
- Makes the binary executable

## Troubleshooting

### Build Fails for Specific Target

Check the build job logs for that target. Common issues:
- Missing cross-compilation tools
- Platform-specific dependencies
- Linker errors

### GoReleaser Config Invalid

Run locally:
```bash
goreleaser check
```

### Release Creation Fails

Ensure:
- Tag is pushed (not just local)
- GitHub token has correct permissions
- All builds completed successfully

## Cross-Compilation Setup

For ARM64 Linux, the workflow uses [cross](https://github.com/cross-rs/cross):

```bash
cargo install cross
cross build --release --target aarch64-unknown-linux-gnu
```

## Permissions

The release workflow needs these GitHub permissions:
- `contents: write` - Create releases
- `packages: write` - Publish packages

These are set in `.github/workflows/release.yml`.
