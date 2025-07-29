# Installation Scripts

These scripts provide automated installation for `gh-templates` across different platforms.

## Quick Install

### Linux/macOS

```bash
curl -sSL https://raw.githubusercontent.com/RafaelJohn9/gh-templates/main/install/install.sh | bash
```

### Windows (PowerShell)

```powershell
iwr -useb https://raw.githubusercontent.com/RafaelJohn9/gh-templates/main/install/install.ps1 | iex
```

## What These Scripts Do

- Automatically detect your operating system and architecture
- Fetch the latest version from GitHub Releases
- Download the appropriate binary
- Install to a standard location (`~/.local/bin` on Linux/macOS, `~/bin` on Windows)
- Make the binary executable

## Manual Installation

If you prefer to run the scripts locally:

```bash
# Linux/macOS
curl -sSL https://raw.githubusercontent.com/RafaelJohn9/gh-templates/main/install/install.sh -o install.sh
chmod +x install.sh
./install.sh

# Windows (PowerShell)
curl -sSL https://raw.githubusercontent.com/RafaelJohn9/gh-templates/main/install/install.ps1 -o install.ps1
./install.ps1
```

## Requirements

- **Linux/macOS**: `curl` or `wget` (one of these is usually pre-installed)
- **Windows**: PowerShell 5.1+ (comes with Windows)

After installation, make sure the install directory is in your `PATH` environment variable.
