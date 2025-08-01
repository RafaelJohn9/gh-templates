---
title: "Installation"
sidebar_label: "Installation"
---

# Installation

`gh-templates` is a Rust-based CLI tool that can be installed through various methods.

## Installation Methods

### Homebrew (macOS/Linux)

```bash
brew install RafaelJohn9/tap/gh-templates
```

### PyPI

```bash
pip install gh-templates
```

### npm

```bash
npm install -g gh-templates
```

### Cargo

```bash
cargo install gh-templates
```

This will download, compile, and install the latest version of `gh-templates` to your Cargo bin directory (usually `~/.cargo/bin`).

### Quick Install (directly from github releases)

Install `gh-templates` automatically with a single command:

**Linux/macOS:**

```bash
curl -sSL https://raw.githubusercontent.com/RafaelJohn9/gh-templates/main/install/install.sh | bash
```

**Windows (PowerShell):**

```powershell
iwr -useb https://raw.githubusercontent.com/RafaelJohn9/gh-templates/main/install/install.ps1 | iex
```

✅ The installer automatically:

- Detects your OS and architecture
- Downloads the latest version
- Installs to the appropriate location (`~/.local/bin` on Linux/macOS, `~/bin` on Windows)
- Makes the binary executable

> **Note:** Ensure your install directory is in your `PATH`. On Linux/macOS, you may need to add `export PATH="$HOME/.local/bin:$PATH"` to your shell profile.

---

## Manual Installation

If you prefer to install manually, download the appropriate binary for your platform from the [GitHub Releases](https://github.com/RafaelJohn9/gh-templates/releases) page:

<details>
<summary>Linux (x86_64, GNU)</summary>

```bash
wget https://github.com/RafaelJohn9/gh-templates/releases/latest/download/gh-templates-x86_64-unknown-linux-gnu
chmod +x gh-templates-x86_64-unknown-linux-gnu
mkdir -p ~/.local/bin
mv gh-templates-x86_64-unknown-linux-gnu ~/.local/bin/gh-templates
```

</details>

<details>
<summary>Linux (x86_64, musl)</summary>

```bash
wget https://github.com/RafaelJohn9/gh-templates/releases/latest/download/gh-templates-x86_64-unknown-linux-musl
chmod +x gh-templates-x86_64-unknown-linux-musl
mkdir -p ~/.local/bin
mv gh-templates-x86_64-unknown-linux-musl ~/.local/bin/gh-templates
```

</details>

<details>
<summary>Linux (aarch64)</summary>

```bash
wget https://github.com/RafaelJohn9/gh-templates/releases/latest/download/gh-templates-aarch64-unknown-linux-gnu
chmod +x gh-templates-aarch64-unknown-linux-gnu
mkdir -p ~/.local/bin
mv gh-templates-aarch64-unknown-linux-gnu ~/.local/bin/gh-templates
```

</details>

<details>
<summary>macOS (Apple Silicon)</summary>

```bash
curl -LO https://github.com/RafaelJohn9/gh-templates/releases/latest/download/gh-templates-aarch64-apple-darwin
chmod +x gh-templates-aarch64-apple-darwin
mkdir -p ~/.local/bin
mv gh-templates-aarch64-apple-darwin ~/.local/bin/gh-templates
```

</details>

<details>
<summary>macOS (Intel)</summary>

```bash
curl -LO https://github.com/RafaelJohn9/gh-templates/releases/latest/download/gh-templates-x86_64-apple-darwin
chmod +x gh-templates-x86_64-apple-darwin
mkdir -p ~/.local/bin
mv gh-templates-x86_64-apple-darwin ~/.local/bin/gh-templates
```

</details>

<details>
<summary>Windows (x86_64)</summary>

```powershell
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\bin"
Invoke-WebRequest -Uri "https://github.com/RafaelJohn9/gh-templates/releases/latest/download/gh-templates-x86_64-pc-windows-gnu.exe" -OutFile "$env:USERPROFILE\bin\gh-templates.exe"
```

> **Note:** Make sure `$env:USERPROFILE\bin` is in your `PATH` environment variable.
</details>

---

## From Source

Clone the repository and build from source:

```bash
git clone https://github.com/RafaelJohn9/gh-templates.git
cd gh-templates
cargo install --path .
```

---

## Verify Installation

After installation, verify that `gh-templates` is working correctly:

```bash
gh-templates --version
```

You should see output similar to:

```
gh-templates 0.1.0
```

After installation, verify that `gh-templates` is working correctly:

```bash
gh-templates --version
```

You should see output similar to:

```
gh-templates 0.1.0
```

## Build Information

To see detailed build information about your installation:

```bash
gh-templates --build-info
```

This displays compilation details and other build metadata.

The `--force` flag ensures the existing installation is overwritten with the new version.

## Next Steps

Now that you have `gh-templates` installed, check out the [Usage Guide](./usage.md) to learn how to use it effectively.
