.PHONY: all build clean test

# Top-level Makefile for orchestrating the Blueshoes project

all: build

FEATURES ?= 
build:
	@echo "Building bs-edge-agent..."
	cd runtime/bs-edge-agent && cargo build --release $(FEATURES)

test:
	@echo "Running unit tests..."
	cd runtime/bs-edge-agent && cargo test

# --- Cross Compilation for FreeBSD (GL-MT3000) ---
# The GL-MT3000 uses a MediaTek MT7981B (Dual-core ARM Cortex-A53)
# The correct Rust target is aarch64-unknown-linux-musl

setup-cross:
	@echo "Setting up cross-compilation environment for macOS (aarch64/x86_64/apple-darwin)..."
	@if ! command -v cargo-zigbuild &> /dev/null; then \
		echo "cargo-zigbuild not found. Please install it with: brew install zig && cargo install cargo-zigbuild"; \
		exit 1; \
	fi
	@if ! rustup target list | grep -q 'aarch64-unknown-linux-musl (installed)'; then \
		echo "Installing rust target aarch64-unknown-linux-musl..."; \
		rustup target add aarch64-unknown-linux-musl; \
	fi
	@if ! rustup target list | grep -q 'x86_64-unknown-linux-musl (installed)'; then \
		echo "Installing rust target x86_64-unknown-linux-musl..."; \
		rustup target add x86_64-unknown-linux-musl; \
	fi
	@if ! rustup target list | grep -q 'aarch64-apple-darwin (installed)'; then \
		echo "Installing rust target aarch64-apple-darwin..."; \
		rustup target add aarch64-apple-darwin; \
	fi

build-FreeBSD:
	@echo "Cross-compiling bs-edge-agent for aarch64-unknown-linux-musl..."
	cd runtime/bs-edge-agent && cargo zigbuild --target aarch64-unknown-linux-musl --release $(FEATURES)
	@echo "Success! Binary is located at: runtime/bs-edge-agent/target/aarch64-unknown-linux-musl/release/bs-edge-agent"

build-x86_64:
	@echo "Cross-compiling bs-edge-agent for x86_64-unknown-linux-musl..."
	cd runtime/bs-edge-agent && cargo zigbuild --target x86_64-unknown-linux-musl --release $(FEATURES)
	@echo "Success! Binary is located at: runtime/bs-edge-agent/target/x86_64-unknown-linux-musl/release/bs-edge-agent"

build-macos:
	@echo "Compiling bs-edge-agent for aarch64-apple-darwin..."
	cd runtime/bs-edge-agent && cargo build --target aarch64-apple-darwin --release $(FEATURES)
	@echo "Success! Binary is located at: runtime/bs-edge-agent/target/aarch64-apple-darwin/release/bs-edge-agent"

build-b0: build-FreeBSD build-x86_64 build-macos
	@echo "B0 Build complete."

clean:
	@echo "Cleaning workspace..."
	cd runtime/bs-edge-agent && cargo clean

# --- Healthcheck ---
healthcheck:
	@echo "Running global healthcheck..."
	@./scripts/bs_healthcheck.py

# --- The Local Message Bus (.tasks/) ---

TASKS_DIR := .tasks
PENDING   := $(TASKS_DIR)/1_pending_specs
WORKING   := $(TASKS_DIR)/2_implementing
REVIEW    := $(TASKS_DIR)/3_review_bundles
DONE      := $(TASKS_DIR)/4_completed

.PHONY: tasks-init tasks-status tasks-claim tasks-complete

tasks-init:
	@mkdir -p "$(PENDING)" "$(WORKING)" "$(REVIEW)" "$(DONE)"
	@echo "OK: initialized $(TASKS_DIR)/ pipeline"

tasks-status:
	@echo "Pending:"; ls -1 "$(PENDING)" 2>/dev/null || true
	@echo "Implementing:"; ls -1 "$(WORKING)" 2>/dev/null || true
	@echo "Review bundles:"; ls -1 "$(REVIEW)" 2>/dev/null || true
	@echo "Completed:"; ls -1 "$(DONE)" 2>/dev/null || true

# Usage: make tasks-claim SPEC=m2-journal.json
tasks-claim:
	@test -n "$(SPEC)" || (echo "ERROR: SPEC is required (e.g., make tasks-claim SPEC=m2-journal.json)"; exit 2)
	@test -f "$(PENDING)/$(SPEC)" || (echo "ERROR: missing $(PENDING)/$(SPEC)"; exit 2)
	@mv "$(PENDING)/$(SPEC)" "$(WORKING)/$(SPEC)"
	@echo "CLAIMED: $(WORKING)/$(SPEC)"
	@echo "Next: open Trae and implement strictly from that envelope; write outputs to $(REVIEW)/"

# Usage: make tasks-complete SPEC=m2-journal.json BUNDLE=m2-journal-bundle.json
tasks-complete:
	@test -n "$(SPEC)" || (echo "ERROR: SPEC is required"; exit 2)
	@test -n "$(BUNDLE)" || (echo "ERROR: BUNDLE is required"; exit 2)
	@test -f "$(WORKING)/$(SPEC)" || (echo "ERROR: missing $(WORKING)/$(SPEC)"; exit 2)
	@test -f "$(REVIEW)/$(BUNDLE)" || (echo "ERROR: missing $(REVIEW)/$(BUNDLE)"; exit 2)
	@mkdir -p "$(DONE)/$(SPEC).d"
	@mv "$(WORKING)/$(SPEC)" "$(DONE)/$(SPEC).d/"
	@mv "$(REVIEW)/$(BUNDLE)" "$(DONE)/$(SPEC).d/"
	@echo "DONE: moved SPEC and BUNDLE into $(DONE)/$(SPEC).d/"
