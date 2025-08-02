---
title: "Introduction"
sidebar_label: "Introduction"
---

# gh-templates

**Scaffold GitHub templates easily**

`gh-templates` is a powerful command-line tool that helps you manage GitHub templates for issues, pull requests, licenses, and `.gitignore` files. Whether you're setting up a new repository or standardizing your existing projects, `gh-templates` streamlines the process of adding professional templates to your GitHub repositories.

## What can gh-templates do?

- **Issue Templates**: Add structured issue templates to improve bug reports and feature requests
- **Pull Request Templates**: Create standardized PR templates for better code review workflows  
- **License Templates**: Easily add popular open-source licenses to your projects
- **Gitignore Templates**: Generate language-specific `.gitignore` files to keep your repos clean

## Key Features

- **Fast Setup**: Add templates with a single command
- **Pre-built Templates**: Access a curated collection of templates for popular languages and frameworks
- **Flexible Output**: Customize file names and output directories
- **Preview Mode**: Preview templates before adding them to your repository
- **Force Overwrite**: Update existing templates when needed

## Quick Start

```bash
# Install gh-templates
cargo install gh-templates

# List available issue templates
gh-templates issue list

# Add a bug issue template
gh-templates issue add bug

# Add multiple templates at once
gh-templates license add mit apache-2.0
```

## Getting Help

Use the built-in help system to explore all available commands:

```bash
# General help
gh-templates --help

# Help for specific subcommands
gh-templates issue --help
gh-templates license add --help
```

Ready to get started? Check out the [Installation Guide](./installation.md) or dive into the [Usage Overview](./usage.md).
