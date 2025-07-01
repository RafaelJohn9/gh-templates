# Gh Templates

This repository provides a collection of reusable Git templates to streamline your workflow across multiple repositories. It includes fully supported **Issue Templates**, **Pull Request Templates**, and more.

## Contributing

We welcome contributions from everyone! Don’t worry if you’re not familiar with Rust—the project is primarily composed of YAML files (for workflows), Markdown files (for templates and documentation), and license files. Rust is only used for the CLI interface. You can help by opening issues, suggesting new GitHub templates, or sharing feedback to improve the project. Your ideas and participation are valued, and we’re happy to support you as you get involved.

## Features

- **Issue Templates**: Standardized templates for bug reports, feature requests, documentation, community collaboration, developer experience feedback, support questions, and tests.
- **Pull Request Templates**: Easily add consistent PR templates to your repositories.
- **Easy Installation**: Quickly set up templates using a provided installation script.
- **Customizable**: All templates can be tailored to fit your project's needs.

## Alpha Release Downloads

You can download the latest alpha release binaries for your platform from the `/releases/<tag>` directory. These are currently available as artifacts for different operating systems and architectures:

| Platform                          | Filename                                    |
|------------------------------------|---------------------------------------------|
| macOS (Apple Silicon)              | `gh-templates-aarch64-apple-darwin`        |
| Linux (aarch64)                    | `gh-templates-aarch64-unknown-linux-gnu`   |
| macOS (Intel)                      | `gh-templates-x86_64-apple-darwin`         |
| Windows (x86_64)                   | `gh-templates-x86_64-pc-windows-gnu.exe`   |
| Linux (x86_64, GNU)                | `gh-templates-x86_64-unknown-linux-gnu`    |
| Linux (x86_64, musl)               | `gh-templates-x86_64-unknown-linux-musl`   |

To download, visit the [releases page](https://github.com/rafaeljohn9/gh-templates/releases) and select the appropriate binary for your OS and architecture. Verify the SHA256 checksum after downloading.

## Installation

To install the latest version (`v0.0.38`), download the appropriate binary for your platform from the [v0.0.38 release page](https://github.com/RafaelJohn9/gh-templates/releases/tag/v0.0.43).

Below are installation instructions for each supported platform. These commands will download the binary, rename it to `gh-templates`, make it executable, and move it to your local `~/.local/bin` (create the directory if it doesn't exist). Adjust the path if you prefer a different location.

### Linux (x86_64, GNU)

```sh
wget https://github.com/RafaelJohn9/gh-templates/releases/download/v0.0.43/gh-templates-x86_64-unknown-linux-gnu
chmod +x gh-templates-x86_64-unknown-linux-gnu
mkdir -p ~/.local/bin
mv gh-templates-x86_64-unknown-linux-gnu ~/.local/bin/gh-templates
```

### Linux (x86_64, musl)

```sh
wget https://github.com/RafaelJohn9/gh-templates/releases/download/v0.0.43/gh-templates-x86_64-unknown-linux-musl
chmod +x gh-templates-x86_64-unknown-linux-musl
mkdir -p ~/.local/bin
mv gh-templates-x86_64-unknown-linux-musl ~/.local/bin/gh-templates
```

### Linux (aarch64)

```sh
wget https://github.com/RafaelJohn9/gh-templates/releases/download/v0.0.43/gh-templates-aarch64-unknown-linux-gnu
chmod +x gh-templates-aarch64-unknown-linux-gnu
mkdir -p ~/.local/bin
mv gh-templates-aarch64-unknown-linux-gnu ~/.local/bin/gh-templates
```

### macOS (Apple Silicon)

```sh
curl -LO https://github.com/RafaelJohn9/gh-templates/releases/download/v0.0.43/gh-templates-aarch64-apple-darwin
chmod +x gh-templates-aarch64-apple-darwin
mkdir -p ~/.local/bin
mv gh-templates-aarch64-apple-darwin ~/.local/bin/gh-templates
```

### macOS (Intel)

```sh
curl -LO https://github.com/RafaelJohn9/gh-templates/releases/download/v0.0.43/gh-templates-x86_64-apple-darwin
chmod +x gh-templates-x86_64-apple-darwin
mkdir -p ~/.local/bin
mv gh-templates-x86_64-apple-darwin ~/.local/bin/gh-templates
```

### Windows (x86_64)

Download the binary manually or use PowerShell:

```powershell
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\bin"
Invoke-WebRequest -Uri "https://github.com/RafaelJohn9/gh-templates/releases/download/v0.0.43/gh-templates-x86_64-pc-windows-gnu.exe" -OutFile "$env:USERPROFILE\bin\gh-templates.exe"
```

Make sure `$env:USERPROFILE\bin` is in your `PATH`.

---

## Example Usage

Below are some example commands for using `gh-templates` to add templates to your repository:

> **Note:**  
> It is recommended you use the labels from this repository by running:  
> `gh label clone rafaeljohn9/gh-templates`

```sh
# Add an issue template
gh-templates add issue bug

# Add a PR template (not yet available)
gh-templates add pr feature-request

# Add a community PR template (not yet available)
gh-templates add pr community-collaboration

# Add a CI workflow for Python code quality checks (not yet available)
gh-templates add ci github python code_quality

# Add a LICENSE file (MIT) (not yet available)
gh-templates add license mit

# Add a LICENSE file (Apache 2.0) (not yet available)
gh-templates add license apache-2.0

# Add a .gitignore for Rust (not yet available)
gh-templates add gitignore Rust

# Add a .gitignore for Node.js (not yet available)
gh-templates add gitignore Node

# Add a code of conduct file (not yet available)
gh-templates add conduct default

# Add template and specify output location (not yet available)
gh-templates add pr bug --output .github/pull_request_template.md

# Force overwrite existing file (not yet available)
gh-templates add license mit --force
```

> **Note:** Currently, only `issue` templates are supported. Support for PR, license, CI, .gitignore, and other templates will be added in future releases.

### Listing Templates

In the future, you will be able to preview templates before adding them:

```sh
# list templates
gh-templates list issue 

# Preview a license (planned)
gh-templates preview license mit
```

### Previewing Templates

In the future, you will be able to preview templates before adding them:

```sh
# Preview a PR template (planned)
gh-templates preview pr feature-request

# Preview a license (planned)
gh-templates preview license mit
```
