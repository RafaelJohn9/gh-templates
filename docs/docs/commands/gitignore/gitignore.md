---
title: "gh-templates gitignore"
sidebar_label: "gitignore"
---

# Gitignore Templates

The `gitignore` subcommand provides functionality for managing `.gitignore` templates. Gitignore templates help you quickly set up language and framework-specific ignore patterns to keep your repositories clean and focused.

The available templates are indexed from [https://github.com/github/gitignore/](https://github.com/github/gitignore/), ensuring up-to-date and comprehensive coverage for a wide range of languages, frameworks, and tools.

## Usage

```bash
gh-templates gitignore <COMMAND>
```

## Available Commands

| Command | Description |
|---------|-------------|
| `add` | Add one or more gitignore templates to the repository |
| `list` | List available gitignore templates |
| `preview` | Preview a specific gitignore template |

## Examples

### List Available Templates

```bash
gh-templates gitignore list
```

### Preview a Template

```bash
gh-templates gitignore preview rust
```

### Add Single Template

```bash
gh-templates gitignore add python
```

### Add Multiple Templates

```bash
gh-templates gitignore add rust node python
```

## Template Categories

Gitignore templates are available for:
All templates available in [github/gitignore](https://github.com/github/gitignore/) can be used, including those in nested directories such as `Global` and `community`. You can reference templates by their name, or for community templates, use either the `community-<template>` or `community/<template>` format.

For example:

- `community-openssl` or `community/OpenSSL` (for `community/OpenSSL.gitignore`)
- `Global/macOS` (for `Global/macOS.gitignore`)

This allows you to access a wide range of language, framework, tool, operating system, and community-contributed gitignore templates.

## Output Location

By default, gitignore files are saved to the repository root as `.gitignore`. This is the standard location where Git expects to find ignore patterns.

You can customize the output location by using the `--dir` flag:

```bash
gh-templates gitignore add python --dir subfolder/
```

This saves the `.gitignore` file to the specified directory instead of the repository root.

## Combining Templates

You can combine multiple gitignore templates:

```bash
gh-templates gitignore add rust docker vscode
```

This creates a comprehensive `.gitignore` file with patterns for Rust development using Docker and VSCode.

## Template Structure

Gitignore templates typically include:

- **File patterns**: Specific files to ignore
- **Directory patterns**: Entire directories to exclude
- **Extension patterns**: Files with specific extensions
- **Comments**: Explanations for ignore patterns

## Next Steps

- [Add Gitignore Templates](./gitignore-add.md)
- [List Gitignore Templates](./gitignore-list.md)
- [Preview Gitignore Templates](./gitignore-preview.md)
