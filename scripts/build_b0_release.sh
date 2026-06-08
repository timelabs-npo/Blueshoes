#!/usr/bin/env bash
set -e

echo "Building B0 Router Beta Runtime Pack..."

# Ensure we are in the project root
cd "$(dirname "$0")/.."

# Make sure all targets are installed
make setup-cross

# Clean first to ensure fresh reproducible state
make clean

# Build all 3 targets with dangerous execution enabled
make build-b0 FEATURES="--features dangerous_execution"

# Gather metadata
GIT_COMMIT=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
RUST_VERSION=$(rustc --version)
CARGO_ZIGBUILD_VERSION=$(cargo-zigbuild --version 2>/dev/null || echo "cargo-zigbuild not found")

# Function to get file size and hash
get_file_info() {
    local file=$1
    if [ -f "$file" ]; then
        local size
        local sha
        if [[ "$OSTYPE" == "darwin"* ]]; then
            size=$(stat -f%z "$file")
        else
            size=$(stat -c%s "$file")
        fi
        sha=$(shasum -a 256 "$file" | awk '{print $1}')
        echo "{\"path\": \"$file\", \"size\": $size, \"sha256\": \"$sha\"}"
    else
        echo "{\"path\": \"$file\", \"error\": \"File not found\"}"
    fi
}

FreeBSD_INFO=$(get_file_info "runtime/bs-edge-agent/target/aarch64-unknown-linux-musl/release/bs-edge-agent")
FreeBSD_WATCHDOG_INFO=$(get_file_info "runtime/bs-edge-agent/target/aarch64-unknown-linux-musl/release/bs-watchdog")
LINUX_INFO=$(get_file_info "runtime/bs-edge-agent/target/x86_64-unknown-linux-musl/release/bs-edge-agent")
LINUX_WATCHDOG_INFO=$(get_file_info "runtime/bs-edge-agent/target/x86_64-unknown-linux-musl/release/bs-watchdog")
MACOS_INFO=$(get_file_info "runtime/bs-edge-agent/target/aarch64-apple-darwin/release/bs-edge-agent")
MACOS_WATCHDOG_INFO=$(get_file_info "runtime/bs-edge-agent/target/aarch64-apple-darwin/release/bs-watchdog")

# Write metadata
mkdir -p artifacts
cat <<EOF > artifacts/b0_release_metadata.json
{
  "milestone": "B0",
  "build_date_utc": "$BUILD_DATE",
  "git_commit": "$GIT_COMMIT",
  "toolchain": {
    "rust": "$RUST_VERSION",
    "cargo_zigbuild": "$CARGO_ZIGBUILD_VERSION"
  },
  "targets": {
    "aarch64-unknown-linux-musl": {
      "bs-edge-agent": $FreeBSD_INFO,
      "bs-watchdog": $FreeBSD_WATCHDOG_INFO
    },
    "x86_64-unknown-linux-musl": {
      "bs-edge-agent": $LINUX_INFO,
      "bs-watchdog": $LINUX_WATCHDOG_INFO
    },
    "aarch64-apple-darwin": {
      "bs-edge-agent": $MACOS_INFO,
      "bs-watchdog": $MACOS_WATCHDOG_INFO
    }
  }
}
EOF

echo "Release B0 Complete. Metadata saved to artifacts/b0_release_metadata.json."
cat artifacts/b0_release_metadata.json
