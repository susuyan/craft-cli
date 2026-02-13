#!/bin/bash
# Release script for craft-cli

set -e

VERSION="0.1.1"
REPO="susuyan/craft-cli"

echo "Building release binaries..."

# Build for macOS ARM64 (Apple Silicon)
cargo build --release --target aarch64-apple-darwin 2>/dev/null || echo "Skipping ARM64 build (requires target)"

# Build for macOS x86_64
cargo build --release --target x86_64-apple-darwin 2>/dev/null || echo "Skipping x86_64 build (requires target)"

# Build for current platform
cargo build --release

echo "Creating release archive..."
cd target/release

# Create archive for current platform
tar czf "craft-cli-${VERSION}-$(uname -m)-apple-darwin.tar.gz" craft-cli

echo "Release archive created: craft-cli-${VERSION}-$(uname -m)-apple-darwin.tar.gz"
echo ""
echo "Next steps:"
echo "1. Create GitHub release: gh release create v${VERSION}"
echo "2. Upload the archive to the release"
echo "3. Update Homebrew formula with new URL and SHA256"
