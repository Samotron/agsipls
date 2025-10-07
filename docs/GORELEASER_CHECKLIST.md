# GoReleaser Testing Checklist

Use this checklist to verify the GoReleaser integration is working correctly.

## Pre-Release Checklist

### Local Validation

- [ ] **Validate GoReleaser config**
  ```bash
  goreleaser check
  ```
  Expected: "✓ 1 configuration file(s) validated"

- [ ] **Validate YAML files**
  ```bash
  python3 -c "import yaml; yaml.safe_load(open('.goreleaser.yml'))"
  python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"
  python3 -c "import yaml; yaml.safe_load(open('.github/workflows/goreleaser-test.yml'))"
  ```
  Expected: No errors

- [ ] **Check install script is executable**
  ```bash
  test -x install.sh && echo "✓ install.sh is executable"
  ```

- [ ] **Verify Git remote exists**
  ```bash
  git remote -v | grep origin
  ```
  Expected: origin URL displayed

- [ ] **Ensure working directory is clean**
  ```bash
  git status
  ```
  Expected: Only new/modified files for GoReleaser

### Documentation Review

- [ ] Read `GORELEASER_QUICKREF.md` for quick commands
- [ ] Review `GORELEASER.md` for full integration guide
- [ ] Check `GORELEASER_WORKFLOW.md` for process flow
- [ ] Verify `install.sh` has correct repository URL placeholders

## Test Release (v0.0.1-test)

### Create Test Release

- [ ] **Commit all changes**
  ```bash
  git add .
  git commit -m "feat: integrate GoReleaser for automated releases"
  ```

- [ ] **Create test tag**
  ```bash
  git tag -a v0.0.1-test -m "Test release"
  ```

- [ ] **Push to GitHub**
  ```bash
  git push origin main
  git push origin v0.0.1-test
  ```

### Monitor GitHub Actions

- [ ] **Visit Actions page**
  - URL: `https://github.com/OWNER/REPO/actions`
  - Look for "Release" workflow

- [ ] **Check build job status**
  - [ ] Linux x86_64 build succeeds
  - [ ] Linux ARM64 build succeeds
  - [ ] macOS x86_64 build succeeds
  - [ ] macOS ARM64 build succeeds
  - [ ] Windows x86_64 build succeeds

- [ ] **Check release job status**
  - [ ] Artifacts downloaded successfully
  - [ ] Binaries organized in dist/
  - [ ] GoReleaser runs without errors
  - [ ] Release created on GitHub

### Verify Release Artifacts

- [ ] **Visit release page**
  - URL: `https://github.com/OWNER/REPO/releases/tag/v0.0.1-test`

- [ ] **Check binaries present**
  - [ ] `agsi_v0.0.1-test_Linux_x86_64.tar.gz`
  - [ ] `agsi_v0.0.1-test_Linux_aarch64.tar.gz` (if cross-compilation works)
  - [ ] `agsi_v0.0.1-test_Darwin_x86_64.tar.gz`
  - [ ] `agsi_v0.0.1-test_Darwin_aarch64.tar.gz`
  - [ ] `agsi_v0.0.1-test_Windows_x86_64.zip`

- [ ] **Check packages present**
  - [ ] `agsi_v0.0.1-test_amd64.deb`
  - [ ] `agsi-v0.0.1-test-1.x86_64.rpm`
  - [ ] `agsi_v0.0.1-test_linux_amd64.apk`

- [ ] **Check metadata files**
  - [ ] `checksums.txt` present
  - [ ] Release notes formatted correctly
  - [ ] Changelog included (if commits exist)

### Test Downloads

- [ ] **Download archive for your platform**
  ```bash
  # Linux
  wget https://github.com/OWNER/REPO/releases/download/v0.0.1-test/agsi_v0.0.1-test_Linux_x86_64.tar.gz
  
  # macOS
  curl -LO https://github.com/OWNER/REPO/releases/download/v0.0.1-test/agsi_v0.0.1-test_Darwin_x86_64.tar.gz
  ```

- [ ] **Verify checksum**
  ```bash
  wget https://github.com/OWNER/REPO/releases/download/v0.0.1-test/checksums.txt
  sha256sum -c checksums.txt 2>&1 | grep agsi_.*$(uname -s | tr '[:upper:]' '[:lower:]')
  ```
  Expected: "OK"

- [ ] **Extract and test binary**
  ```bash
  tar -xzf agsi_v0.0.1-test_*.tar.gz
  ./agsi --version
  ./agsi --help
  ```
  Expected: Version and help output

### Test Install Script

- [ ] **Update install.sh with actual repo**
  ```bash
  # Edit install.sh and replace placeholders
  sed -i 's/yourusername/ACTUAL_USER/g' install.sh
  sed -i 's/agsipls/ACTUAL_REPO/g' install.sh
  ```

- [ ] **Test install script locally**
  ```bash
  # In a test directory
  export AGSI_VERSION=v0.0.1-test
  bash install.sh
  ```

- [ ] **Verify installation**
  ```bash
  which agsi
  agsi --version
  ```

### Test Linux Packages (if on Linux)

- [ ] **Test .deb package**
  ```bash
  wget https://github.com/OWNER/REPO/releases/download/v0.0.1-test/agsi_v0.0.1-test_amd64.deb
  sudo dpkg -i agsi_v0.0.1-test_amd64.deb
  agsi --version
  sudo dpkg -r agsi
  ```

- [ ] **Test .rpm package**
  ```bash
  wget https://github.com/OWNER/REPO/releases/download/v0.0.1-test/agsi-v0.0.1-test-1.x86_64.rpm
  sudo rpm -i agsi-v0.0.1-test-1.x86_64.rpm
  agsi --version
  sudo rpm -e agsi
  ```

### Clean Up Test Release

- [ ] **Delete test release**
  ```bash
  gh release delete v0.0.1-test --yes
  ```

- [ ] **Delete test tag**
  ```bash
  git tag -d v0.0.1-test
  git push --delete origin v0.0.1-test
  ```

## Production Release (v0.1.0)

### Preparation

- [ ] All test release checks passed
- [ ] Code is stable and tested
- [ ] Documentation is up to date
- [ ] CHANGELOG.md updated with changes
- [ ] Version bumped in Cargo.toml

### Create Production Release

- [ ] **Create release tag**
  ```bash
  git tag -a v0.1.0 -m "Release v0.1.0"
  git push origin v0.1.0
  ```

- [ ] **Monitor release process**
  - Same checks as test release

- [ ] **Announce release**
  - Share on social media
  - Update documentation
  - Notify users

### Post-Release Validation

- [ ] All artifacts present and downloadable
- [ ] Install script works from main branch
- [ ] Documentation links correct
- [ ] No broken links in release notes

## Optional: Homebrew Tap Setup

### Create Homebrew Tap Repository

- [ ] **Create repository**
  ```bash
  # On GitHub, create repository: homebrew-tap
  ```

- [ ] **Add token to secrets**
  - Go to repository Settings → Secrets
  - Add `HOMEBREW_TAP_GITHUB_TOKEN`

- [ ] **Test Homebrew installation**
  ```bash
  brew tap OWNER/tap
  brew install OWNER/tap/agsi
  agsi --version
  ```

## Troubleshooting Checklist

### Build Failures

- [ ] Check GitHub Actions logs
- [ ] Verify Rust toolchain version
- [ ] Test build locally:
  ```bash
  cargo build --release --target x86_64-unknown-linux-gnu
  ```
- [ ] Check for missing dependencies

### GoReleaser Errors

- [ ] Validate config: `goreleaser check`
- [ ] Check dist/ structure matches expected
- [ ] Verify binary names are correct
- [ ] Check permissions (contents: write)

### Missing Artifacts

- [ ] All build jobs completed successfully
- [ ] Binaries present in artifacts
- [ ] dist/ structure correct before GoReleaser
- [ ] GoReleaser job didn't fail

### Install Script Issues

- [ ] Check URL is correct
- [ ] Verify release exists
- [ ] Test with curl/wget directly
- [ ] Check file permissions

## Performance Benchmarks

Record actual build times:

- [ ] First build time: _____ minutes
- [ ] Cached build time: _____ minutes
- [ ] GoReleaser time: _____ minutes
- [ ] Total release time: _____ minutes

## Success Criteria

All items below must be true:

- [x] GoReleaser config valid
- [x] All YAML files valid
- [x] Install script functional
- [ ] Test release successful
- [ ] All 5 platforms build successfully
- [ ] All 8+ artifacts present in release
- [ ] Checksums valid
- [ ] Binary works when downloaded
- [ ] Install script works end-to-end
- [ ] Release notes formatted correctly
- [ ] No errors in GitHub Actions logs

## Notes

Use this space to record issues encountered and solutions:

```
Date: ___________
Issue: 
Solution:

Date: ___________
Issue:
Solution:
```

---

**Checklist Version**: 1.0  
**Last Updated**: 2025-01-06  
**Status**: Ready for use
