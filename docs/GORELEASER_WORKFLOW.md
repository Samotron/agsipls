# GoReleaser Workflow Diagram

## Release Process Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                      DEVELOPER ACTION                            │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │  Create Git Tag  │
                    │    (v0.1.0)      │
                    └──────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │  Push Tag to     │
                    │     GitHub       │
                    └──────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                  GITHUB ACTIONS TRIGGERED                        │
└─────────────────────────────────────────────────────────────────┘
                              │
          ┌───────────────────┴───────────────────┐
          │                                       │
          ▼                                       ▼
┌─────────────────────┐                 ┌─────────────────────┐
│   BUILD JOB         │                 │   BUILD JOB         │
│   (Matrix x5)       │                 │   (Matrix x5)       │
└─────────────────────┘                 └─────────────────────┘
          │                                       │
          ├─ Linux x86_64                        │
          ├─ Linux ARM64 (cross)                 │
          ├─ macOS x86_64                        │
          ├─ macOS ARM64                         │
          └─ Windows x86_64                      │
          │                                       │
          ▼                                       ▼
┌─────────────────────┐                 ┌─────────────────────┐
│ Compile with Cargo  │                 │  Strip Binaries     │
│  cargo build        │                 │   (Linux/macOS)     │
└─────────────────────┘                 └─────────────────────┘
          │                                       │
          └───────────────────┬───────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │ Upload Artifacts │
                    │  (5 binaries)    │
                    └──────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     RELEASE JOB                                  │
└─────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────┐
│ Download Artifacts  │
│  from Build Job     │
└─────────────────────┘
          │
          ▼
┌─────────────────────┐
│ Organize in dist/   │
│   Structure for     │
│   GoReleaser        │
└─────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                      GORELEASER                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  1. Create Archives (tar.gz/zip)                                │
│  2. Generate Checksums (SHA256)                                 │
│  3. Build Changelog (from commits)                              │
│  4. Create GitHub Release                                       │
│  5. Upload Archives                                             │
│  6. Upload Checksums                                            │
│  7. Create Linux Packages (deb/rpm/apk)                         │
│  8. Upload Packages                                             │
│  9. Update Homebrew Tap (optional)                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    GITHUB RELEASE CREATED                        │
└─────────────────────────────────────────────────────────────────┘
          │
          ├─ agsi_v0.1.0_Linux_x86_64.tar.gz
          ├─ agsi_v0.1.0_Linux_aarch64.tar.gz
          ├─ agsi_v0.1.0_Darwin_x86_64.tar.gz
          ├─ agsi_v0.1.0_Darwin_aarch64.tar.gz
          ├─ agsi_v0.1.0_Windows_x86_64.zip
          ├─ checksums.txt
          ├─ agsi_v0.1.0_amd64.deb
          ├─ agsi-v0.1.0-1.x86_64.rpm
          ├─ agsi_v0.1.0_linux_amd64.apk
          └─ Release Notes (auto-generated)
          │
          ▼
┌─────────────────────────────────────────────────────────────────┐
│                     USERS CAN INSTALL                            │
└─────────────────────────────────────────────────────────────────┘
```

## Component Interactions

```
┌──────────────┐         ┌──────────────┐         ┌──────────────┐
│              │         │              │         │              │
│   GitHub     │────────▶│   GitHub     │────────▶│  GoReleaser  │
│   Tags       │         │   Actions    │         │              │
│              │         │              │         │              │
└──────────────┘         └──────────────┘         └──────────────┘
                                │                        │
                                │                        │
                                ▼                        ▼
                         ┌──────────────┐         ┌──────────────┐
                         │              │         │              │
                         │    Cargo     │         │   Archives   │
                         │  (Rust Build)│         │ & Packages   │
                         │              │         │              │
                         └──────────────┘         └──────────────┘
```

## File Structure in Release Job

```
dist/
├── linux_amd64/
│   └── agsi
├── linux_arm64/
│   └── agsi
├── darwin_amd64/
│   └── agsi
├── darwin_arm64/
│   └── agsi
└── windows_amd64/
    └── agsi.exe

After GoReleaser:

dist/
├── agsi_v0.1.0_Linux_x86_64.tar.gz
├── agsi_v0.1.0_Linux_aarch64.tar.gz
├── agsi_v0.1.0_Darwin_x86_64.tar.gz
├── agsi_v0.1.0_Darwin_aarch64.tar.gz
├── agsi_v0.1.0_Windows_x86_64.zip
├── checksums.txt
├── agsi_v0.1.0_amd64.deb
├── agsi-v0.1.0-1.x86_64.rpm
└── agsi_v0.1.0_linux_amd64.apk
```

## Configuration Files Relationship

```
.goreleaser.yml
    │
    ├─ Defines: Archives format
    ├─ Defines: Changelog rules
    ├─ Defines: Release notes template
    ├─ Defines: Homebrew formula
    └─ Defines: Package metadata
    
.github/workflows/release.yml
    │
    ├─ Triggers: On tag push (v*)
    ├─ Job 1: Build binaries (matrix)
    ├─ Job 2: Run GoReleaser
    └─ Uses: .goreleaser.yml
    
install.sh
    │
    ├─ Downloads: Latest release
    ├─ Extracts: Binary
    └─ Installs: To PATH
```

## Timeline

```
Time    Event
─────────────────────────────────────────────────────────────
0:00    Tag pushed to GitHub
0:01    GitHub Actions triggered
0:02    Build jobs start (parallel)
0:03    - Linux x86_64 compiling
        - Linux ARM64 compiling (cross)
        - macOS x86_64 compiling
        - macOS ARM64 compiling
        - Windows x86_64 compiling
5:00    All builds complete (cached)
5:01    Artifacts uploaded
5:02    Release job starts
5:03    GoReleaser begins packaging
5:05    Archives created
5:06    Checksums generated
5:07    Linux packages built
5:08    GitHub release created
5:09    All assets uploaded
5:10    Homebrew tap updated (optional)
5:11    ✅ RELEASE COMPLETE
```

## Caching Strategy

```
┌─────────────────────────────────────────────────────────┐
│                  First Build (No Cache)                  │
├─────────────────────────────────────────────────────────┤
│  1. Install Rust toolchain             ~2 min           │
│  2. Download dependencies              ~3 min           │
│  3. Compile dependencies               ~5 min           │
│  4. Compile project                    ~2 min           │
│                                                          │
│  Total: ~12 minutes per platform                        │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│              Subsequent Builds (With Cache)              │
├─────────────────────────────────────────────────────────┤
│  1. Restore Rust toolchain             ~10 sec          │
│  2. Restore dependencies               ~10 sec          │
│  3. Restore compiled deps              ~10 sec          │
│  4. Compile project (incremental)      ~2 min           │
│                                                          │
│  Total: ~3 minutes per platform                         │
└─────────────────────────────────────────────────────────┘

Cached Items:
  • ~/.cargo/registry
  • ~/.cargo/git
  • target/
```

## Error Handling

```
                    ┌──────────────┐
                    │  Build Step  │
                    └──────────────┘
                          │
          ┌───────────────┼───────────────┐
          │ Success       │ Failure       │
          ▼               ▼               │
    ┌──────────┐    ┌──────────┐         │
    │ Continue │    │   Stop   │         │
    │   to     │    │  Entire  │         │
    │ Release  │    │  Release │         │
    └──────────┘    └──────────┘         │
                          │               │
                          ▼               │
                    ┌──────────────┐     │
                    │  Show Logs   │     │
                    │  In Actions  │     │
                    └──────────────┘     │
                                         │
    All builds must succeed             │
    before release job starts           │
                                         │
                    ┌────────────────────┘
                    │
                    ▼
            ┌──────────────────┐
            │  Fix & Retry:    │
            │  1. Fix code     │
            │  2. New commit   │
            │  3. Delete tag   │
            │  4. Re-tag       │
            │  5. Push tag     │
            └──────────────────┘
```

## User Installation Paths

```
User Action                 Behind the Scenes
─────────────────────────────────────────────────────────────
curl install.sh | bash  ─▶  Download latest release
                         ─▶  Detect OS/arch
                         ─▶  Extract binary
                         ─▶  Move to PATH
                         
brew install agsi       ─▶  Fetch from Homebrew tap
                         ─▶  Download from GitHub release
                         ─▶  Install to /usr/local/bin
                         
apt install agsi.deb    ─▶  Extract .deb package
                         ─▶  Install to /usr/bin
                         ─▶  Setup man pages
                         
Manual download         ─▶  User downloads .tar.gz
                         ─▶  User extracts
                         ─▶  User moves binary
                         ─▶  User adds to PATH
```
