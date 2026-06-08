# Bounded Developer Environment (Infra-as-Code)

This folder contains the declarative configuration for establishing a reproducible, portable, and rollback-safe IDE workspace matching the Blueshoes doctrine.

## Structure

*   `extensions.txt`: Canonical list of active IDE extensions.
*   `settings.json`: Redacted/sanitized IDE settings (no hardcoded API keys/secrets).
*   `keybindings.json`: Custom keybinding configurations.
*   `workflows/`: Bounded execution worksheets.
*   `rules/`: System-level guardrails and strict agent behavior limits.
*   `prompts/`: Standard agent prompt envelopes.

## Recovery & Setup

### 1. Restore Extensions
To install all listed extensions into your VSCode-compatible IDE (VSCode / Cursor / Windsurf):

```bash
cat extensions.txt | xargs -L 1 code --install-extension
```

*(For Codium/OpenVSX environments, swap `code` with `codium` or the respective IDE CLI command)*

### 2. Restore User Configurations
Copy `settings.json` and `keybindings.json` to your IDE User profile directory:

*   **macOS**: `~/Library/Application Support/Windsurf/User/` (or `Code/User/` for VSCode)
*   **Linux**: `~/.config/Windsurf/User/` (or `Code/User/`)
