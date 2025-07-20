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

Below are installation instructions for each supported platform. These commands will download the binary, rename it to `gh-templates`, make it executable, and move it to your local `~/.local/bin` (create the directory if it doesn't exist). Adjust the path if you prefer a different location.

### Linux (x86_64, GNU)

```sh
wget https://github.com/RafaelJohn9/gh-templates/releases/download/v0.1.2/gh-templates-x86_64-unknown-linux-gnu
chmod +x gh-templates-x86_64-unknown-linux-gnu
mkdir -p ~/.local/bin
mv gh-templates-x86_64-unknown-linux-gnu ~/.local/bin/gh-templates
```

### Linux (x86_64, musl)

```sh
wget https://github.com/RafaelJohn9/gh-templates/releases/download/v0.1.2/gh-templates-x86_64-unknown-linux-musl
chmod +x gh-templates-x86_64-unknown-linux-musl
mkdir -p ~/.local/bin
mv gh-templates-x86_64-unknown-linux-musl ~/.local/bin/gh-templates
```

### Linux (aarch64)

```sh
wget https://github.com/RafaelJohn9/gh-templates/releases/download/v0.1.2/gh-templates-aarch64-unknown-linux-gnu
chmod +x gh-templates-aarch64-unknown-linux-gnu
mkdir -p ~/.local/bin
mv gh-templates-aarch64-unknown-linux-gnu ~/.local/bin/gh-templates
```

### macOS (Apple Silicon)

```sh
curl -LO https://github.com/RafaelJohn9/gh-templates/releases/download/v0.1.2/gh-templates-aarch64-apple-darwin
chmod +x gh-templates-aarch64-apple-darwin
mkdir -p ~/.local/bin
mv gh-templates-aarch64-apple-darwin ~/.local/bin/gh-templates
```

### macOS (Intel)

```sh
curl -LO https://github.com/RafaelJohn9/gh-templates/releases/download/v0.1.2/gh-templates-x86_64-apple-darwin
chmod +x gh-templates-x86_64-apple-darwin
mkdir -p ~/.local/bin
mv gh-templates-x86_64-apple-darwin ~/.local/bin/gh-templates
```

### Windows (x86_64)

Download the binary manually or use PowerShell:

```powershell
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\bin"
Invoke-WebRequest -Uri "https://github.com/RafaelJohn9/gh-templates/releases/download/v0.1.2/gh-templates-x86_64-pc-windows-gnu.exe" -OutFile "$env:USERPROFILE\bin\gh-templates.exe"
```

Make sure `$env:USERPROFILE\bin` is in your `PATH`.

---

## Example Usage

Below are some example commands for using `gh-templates` to add templates to your repository:

> **Note:**  
> It is recommended you use the labels from this repository by running:  
> `gh label clone rafaeljohn9/gh-templates`

### Issue Templates

```sh
# Add an issue template
gh-templates issue add bug

# Add multiple issues
gh-templates issue add bug feature dx

# Preview an issue template
gh-templates issue preview bug

# List available issue templates
gh-templates issue list
```

### Pull Request Templates

```sh
# Add a PR template
gh-templates pr add default

# Preview a PR template
gh-templates pr preview default

# List available PR templates
gh-templates pr list

# Add template and specify output location
gh-templates pr add bug --output .github/pull_request_template.md
```

### License Files

```sh
# Add a LICENSE file (MIT)
gh-templates license add mit

# Add a LICENSE file (Apache 2.0)
gh-templates license add apache-2.0

# Preview a license file
gh-templates license preview mit

# List available license files
gh-templates license list

# Force overwrite existing file
gh-templates license add mit --force
```

### .gitignore Files

```sh
# Add a .gitignore for Rust
gh-templates gitignore add Rust

# Add a .gitignore for Node.js
gh-templates gitignore add Node

# Preview a .gitignore file
gh-templates gitignore preview Rust

# List available .gitignore templates
gh-templates gitignore list
```

### Code of Conduct (Coming Soon)

```sh
# Add a code of conduct file (not yet available)
gh-templates conduct add default
```

> **Note:** Currently, `issue`, `pr`, `license`, and `gitignore` templates are supported. Support for code of conduct and other templates will be added in future releases.
