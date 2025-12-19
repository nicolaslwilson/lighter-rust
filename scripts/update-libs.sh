#!/bin/bash
# Script to update lighter-go signing libraries
# Usage: ./scripts/update-libs.sh <version>
# Example: ./scripts/update-libs.sh 0.1.4

set -e

VERSION=${1:-"0.1.3"}
REPO="elliottech/lighter-go"
BASE_URL="https://github.com/${REPO}/releases/download/v${VERSION}"

echo "Updating lighter-go libraries to version ${VERSION}..."

# Create libs directory structure if it doesn't exist
mkdir -p libs/darwin/arm64
mkdir -p libs/linux/amd64
mkdir -p libs/linux/arm64
mkdir -p libs/windows/amd64

# Function to download a file with validation
download_file() {
    local url=$1
    local output=$2
    local temp_file=$(mktemp)
    local http_code
    
    echo "    Trying: ${url}"
    
    # Download to temp file and check HTTP status
    http_code=$(curl -L -s -o "${temp_file}" -w "%{http_code}" "${url}")
    
    if [ "${http_code}" != "200" ]; then
        rm -f "${temp_file}"
        return 1
    fi
    
    # Check if file is HTML (error page) by looking for common HTML markers
    if head -1 "${temp_file}" | grep -qiE "(<!doctype|<html|not found)"; then
        echo "    ERROR: Downloaded file appears to be an HTML error page"
        rm -f "${temp_file}"
        return 1
    fi
    
    # Check if file is too small (likely an error)
    local file_size=$(stat -f%z "${temp_file}" 2>/dev/null || stat -c%s "${temp_file}" 2>/dev/null)
    if [ "${file_size}" -lt 100 ]; then
        echo "    ERROR: Downloaded file is too small (${file_size} bytes), likely an error"
        rm -f "${temp_file}"
        return 1
    fi
    
    # Move temp file to final location
    mv "${temp_file}" "${output}"
    echo "    ✓ Downloaded: $(basename ${output})"
    return 0
}

# Function to try multiple URL patterns
try_download() {
    local output=$1
    shift
    local urls=("$@")
    
    for url in "${urls[@]}"; do
        if download_file "${url}" "${output}"; then
            return 0
        fi
    done
    
    echo "    ERROR: Failed to download $(basename ${output}) from all attempted URLs"
    return 1
}

# Download libraries for each platform
echo "Downloading libraries..."

# Darwin ARM64
echo "  - Darwin ARM64..."
try_download "libs/darwin/arm64/liblighter-signer.dylib" \
    "${BASE_URL}/lighter-signer-darwin-arm64.dylib"

try_download "libs/darwin/arm64/liblighter-signer.h" \
    "${BASE_URL}/lighter-signer-darwin-arm64.h"

# Linux AMD64
echo "  - Linux AMD64..."
try_download "libs/linux/amd64/liblighter-signer.so" \
    "${BASE_URL}/lighter-signer-linux-amd64.so"

try_download "libs/linux/amd64/liblighter-signer.h" \
    "${BASE_URL}/lighter-signer-linux-amd64.h"

# Linux ARM64
echo "  - Linux ARM64..."
try_download "libs/linux/arm64/liblighter-signer.so" \
    "${BASE_URL}/lighter-signer-linux-arm64.so"

try_download "libs/linux/arm64/liblighter-signer.h" \
    "${BASE_URL}/lighter-signer-linux-arm64.h"

# Windows AMD64
echo "  - Windows AMD64..."
try_download "libs/windows/amd64/liblighter-signer.dll" \
    "${BASE_URL}/lighter-signer-windows-amd64.dll"

try_download "libs/windows/amd64/liblighter-signer.h" \
    "${BASE_URL}/lighter-signer-windows-amd64.h"

# Generate new checksums
echo "Generating checksums..."
cd libs
shasum -a 256 darwin/arm64/liblighter-signer.dylib \
           darwin/arm64/liblighter-signer.h \
           linux/amd64/liblighter-signer.h \
           linux/amd64/liblighter-signer.so \
           linux/arm64/liblighter-signer.h \
           linux/arm64/liblighter-signer.so \
           windows/amd64/liblighter-signer.dll \
           windows/amd64/liblighter-signer.h > .checksums.new

# Update .checksums file with header
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
cd ..

# Update README.md
echo "Updating libs/README.md..."
cat > libs/README.md << EOF
## lighter-go signing libaries

https://github.com/elliottech/lighter-go/releases/tag/v${VERSION}
EOF

# Update build.rs comment
echo "Updating build.rs comment..."
sed -i.bak "s/v0\.1\.3/v${VERSION}/g" build.rs
rm -f build.rs.bak

echo ""
echo "✓ Libraries updated to version ${VERSION}"
echo ""
echo "Next steps:"
echo "1. Verify the checksums: cd libs && shasum -a 256 -c .checksums"
echo "2. Test the build: cargo build"
echo "3. If the header files changed, you may need to regenerate bindings"
echo ""
echo "NOTE: If downloads failed, verify the release exists and asset names match:"
echo "      https://github.com/${REPO}/releases/tag/v${VERSION}"
