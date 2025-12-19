# Updating Lighter Signer Libraries

This guide explains how to update the lighter-go signing libraries in this project.

## Overview

The lighter-rust SDK depends on platform-specific native libraries from the [lighter-go](https://github.com/elliottech/lighter-go) repository. When a new version is released, you need to:

1. Download the new libraries for all supported platforms
2. Replace the old libraries in the `libs/` directory
3. Update the checksums in `libs/.checksums`
4. Update version references in documentation

## Supported Platforms

- **Darwin ARM64** (macOS Apple Silicon): `libs/darwin/arm64/`
- **Linux AMD64** (x86_64): `libs/linux/amd64/`
- **Linux ARM64** (aarch64): `libs/linux/arm64/`
- **Windows AMD64** (x86_64): `libs/windows/amd64/`

Each platform needs:

- The library file (`.dylib`, `.so`, or `.dll`)
- The header file (`.h`)

## Automated Update (Recommended)

Use the provided script:

```bash
./scripts/update-libs.sh <version>
```

Example:

```bash
./scripts/update-libs.sh 0.1.4
```

**Important Notes:**

- The script attempts to download from common release asset naming patterns
- **Before running**, check the [actual release page](https://github.com/elliottech/lighter-go/releases) to see the exact asset names
- If the script fails to download files (you'll see "ERROR" messages), the asset naming may be different
- The script validates downloads and won't overwrite files with error pages
- If automated download fails, use the manual method below

## Manual Update Steps

### 1. Download Libraries

Go to the [lighter-go releases page](https://github.com/elliottech/lighter-go/releases) and download the assets for the version you want.

For each platform, download:

- The library file (`.dylib`, `.so`, or `.dll`)
- The header file (`.h` - may be shared across platforms)

Place them in the appropriate directories:

```
libs/
├── darwin/arm64/
│   ├── liblighter-signer.dylib
│   └── liblighter-signer.h
├── linux/amd64/
│   ├── liblighter-signer.so
│   └── liblighter-signer.h
├── linux/arm64/
│   ├── liblighter-signer.so
│   └── liblighter-signer.h
└── windows/amd64/
    ├── liblighter-signer.dll
    └── liblighter-signer.h
```

### 2. Generate Checksums

After downloading all files, generate new checksums:

```bash
cd libs
shasum -a 256 \
  darwin/arm64/liblighter-signer.dylib \
  darwin/arm64/liblighter-signer.h \
  linux/amd64/liblighter-signer.h \
  linux/amd64/liblighter-signer.so \
  linux/arm64/liblighter-signer.h \
  linux/arm64/liblighter-signer.so \
  windows/amd64/liblighter-signer.dll \
  windows/amd64/liblighter-signer.h > .checksums.new
```

Then update `.checksums` with the header and new checksums:

```bash
cat > .checksums << 'EOF'
# SHA-256 Checksums for Lighter Signer Libraries
# These checksums verify the integrity of the official signer libraries
# from the Lighter GitHub repository.
#
# To verify all files:
#   cd libs && shasum -a 256 -c .checksums
#
# To verify a single file:
#   shasum -a 256 <file> | grep <expected-hash>
#
# Format: SHA256_HASH  RELATIVE_PATH
EOF
cat .checksums.new >> .checksums
rm .checksums.new
```

### 3. Update Version References

Update the version in these files:

**`libs/README.md`:**

```markdown
## lighter-go signing libaries

https://github.com/elliottech/lighter-go/releases/tag/v<VERSION>
```

**`build.rs` (line 4):**

```rust
// instead of mapping them manually, since we have the header files from v<VERSION>
```

### 4. Verify Checksums

Verify all files match their checksums:

```bash
cd libs
shasum -a 256 -c .checksums
```

### 5. Test the Build

Build the project to ensure everything works:

```bash
cargo build
```

If the header files have changed, `bindgen` will automatically regenerate the Rust bindings. If you encounter compilation errors related to the bindings, you may need to:

1. Clean the build: `cargo clean`
2. Rebuild: `cargo build`

## Troubleshooting

### Header File Changes

If the C header file (`liblighter-signer.h`) has changed, the Rust bindings will be regenerated automatically. However, if there are breaking changes in the API, you may need to update the Rust code that uses these bindings.

### Library Loading Issues

If you encounter runtime errors about libraries not being found:

1. Check that the libraries are in the correct directories
2. Verify the library files have the correct permissions
3. On Linux, ensure the library paths in `build.rs` are correct
4. On macOS, check that the dylib is properly signed (if required)

### Checksum Verification Fails

If checksum verification fails:

1. Re-download the files to ensure they weren't corrupted
2. Verify you downloaded the correct version
3. Check that the file paths in `.checksums` match the actual file locations

## Current Version

The current version is tracked in:

- `libs/README.md`
- `build.rs` (comment on line 4)

Check these files to see what version is currently in use.
