# Git Templates

This repository provides a collection of reusable Git templates to streamline your workflow across multiple repositories. It includes fully supported **Issue Templates**, **Pull Request Templates**, and more.

## Features

- **Issue Templates**: Standardized templates for bug reports, feature requests, documentation, community collaboration, developer experience feedback, support questions, and tests.
- **Pull Request Templates**: Easily add consistent PR templates to your repositories.
- **Easy Installation**: Quickly set up templates using a provided installation script.
- **Customizable**: All templates can be tailored to fit your project's needs.

## Alpha Release Downloads

You can download the latest beta release binaries for your platform from the `/releases/<tag>` directory. These are currently available as artifacts for different operating systems and architectures:

| Platform                          | Filename                                    | SHA256                                                             |
|------------------------------------|---------------------------------------------|--------------------------------------------------------------------|
| macOS (Apple Silicon)              | `git-templates-aarch64-apple-darwin`        | `86ef1d883c43ed19c94d4911e5ea032a27b7fb5b7cc3a64d493bef03b82a1435`|
| Linux (aarch64)                    | `git-templates-aarch64-unknown-linux-gnu`   | `8a85bb46cbc965d9cb1f187523f00f0690dae6c2ded57c8cab0405fb7659ff8f` |
| macOS (Intel)                      | `git-templates-x86_64-apple-darwin`         | `169e635a170f467ee398947dfc9e76a8a597d1669d01996d0945813fce89d069`|
| Windows (x86_64)                   | `git-templates-x86_64-pc-windows-gnu.exe`   | `e414d145d61abb90861d5e2cfb09859d0d94bc6f0c3367cd9be244d2874b168b`|
| Linux (x86_64, GNU)                | `git-templates-x86_64-unknown-linux-gnu`    | `97d62de5399838e35a43b0ad78a99899314b18bf3b7a695ab15729d3aca2526f`|
| Linux (x86_64, musl)               | `git-templates-x86_64-unknown-linux-musl`   | `48fe89460bc945522c6ba42844cd6418a51420f01eec5b6f7c614fe9d32cbf9c`                                              |

To download, visit the [releases page](https://github.com/rafaeljohn9/git-templates/releases) and select the appropriate binary for your OS and architecture. Verify the SHA256 checksum after downloading.

## Installation

To install the templates for this specific version (`v0.0.14`), download the appropriate binary for your platform from the [v0.0.14 release page](https://github.com/RafaelJohn9/git-templates/releases/tag/v0.0.14).

Below are installation instructions for each supported platform. These commands will download the binary, rename it to `git-templates`, make it executable, and move it to your local `~/.local/bin` (create the directory if it doesn't exist). Adjust the path if you prefer a different location.

### Linux (x86_64, GNU)

```sh
wget https://github.com/RafaelJohn9/git-templates/releases/download/v0.0.14/git-templates-x86_64-unknown-linux-gnu
chmod +x git-templates-x86_64-unknown-linux-gnu
mkdir -p ~/.local/bin
mv git-templates-x86_64-unknown-linux-gnu ~/.local/bin/git-templates
```

### Linux (x86_64, musl)

```sh
wget https://github.com/RafaelJohn9/git-templates/releases/download/v0.0.14/git-templates-x86_64-unknown-linux-musl
chmod +x git-templates-x86_64-unknown-linux-musl
mkdir -p ~/.local/bin
mv git-templates-x86_64-unknown-linux-musl ~/.local/bin/git-templates
```

### Linux (aarch64)

```sh
wget https://github.com/RafaelJohn9/git-templates/releases/download/v0.0.14/git-templates-aarch64-unknown-linux-gnu
chmod +x git-templates-aarch64-unknown-linux-gnu
mkdir -p ~/.local/bin
mv git-templates-aarch64-unknown-linux-gnu ~/.local/bin/git-templates
```

### macOS (Apple Silicon)

```sh
curl -LO https://github.com/RafaelJohn9/git-templates/releases/download/v0.0.14/git-templates-aarch64-apple-darwin
chmod +x git-templates-aarch64-apple-darwin
mkdir -p ~/.local/bin
mv git-templates-aarch64-apple-darwin ~/.local/bin/git-templates
```

### macOS (Intel)

```sh
curl -LO https://github.com/RafaelJohn9/git-templates/releases/download/v0.0.14/git-templates-x86_64-apple-darwin
chmod +x git-templates-x86_64-apple-darwin
mkdir -p ~/.local/bin
mv git-templates-x86_64-apple-darwin ~/.local/bin/git-templates
```

### Windows (x86_64)

Download the binary manually or use PowerShell:

```powershell
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\bin"
Invoke-WebRequest -Uri "https://github.com/RafaelJohn9/git-templates/releases/download/v0.0.14/git-templates-x86_64-pc-windows-gnu.exe" -OutFile "$env:USERPROFILE\bin\git-templates.exe"
```

Make sure `$env:USERPROFILE\bin` is in your `PATH`.

---

## Example Usage

Below are some example commands for using `git-templates` to add templates to your repository:

```sh
# Add an issue template
git-templates add issue bug

# Add a PR template (not yet available)
git-templates add pr feature-request

# Add a community PR template (not yet available)
git-templates add pr community-collaboration

# Add a CI workflow for Python code quality checks (not yet available)
git-templates add ci github python code_quality

# Add a LICENSE file (MIT) (not yet available)
git-templates add license mit

# Add a LICENSE file (Apache 2.0) (not yet available)
git-templates add license apache-2.0

# Add a .gitignore for Rust (not yet available)
git-templates add gitignore Rust

# Add a .gitignore for Node.js (not yet available)
git-templates add gitignore Node

# Add a code of conduct file (not yet available)
git-templates add conduct default

# Add template and specify output location (not yet available)
git-templates add pr bug --output .github/pull_request_template.md

# Force overwrite existing file (not yet available)
git-templates add license mit --force
```

> **Note:** Currently, only `issue` templates are supported. Support for PR, license, CI, .gitignore, and other templates will be added in future releases.

### listing Templates

In the future, you will be able to preview templates before adding them:

```sh
# list templates
git-templates list issue 

# Preview a license (planned)
git-templates preview license mit
```

### Previewing Templates (Planned Feature)

In the future, you will be able to preview templates before adding them:

```sh
# Preview a PR template (planned)
git-templates preview pr feature-request

# Preview a license (planned)
git-templates preview license mit
```
