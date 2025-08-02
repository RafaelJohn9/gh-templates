---
title: "Commands Overview"
sidebar_label: "Overview"
---

# Commands Overview

`gh-templates` provides four main subcommands for managing different types of GitHub templates. Each subcommand follows a consistent pattern with `add`, `list`, and `preview` actions.

## Main Commands

### Issue Templates (`issue`)

Manage GitHub issue templates to standardize bug reports and feature requests.

```bash
gh-templates issue <action> [options] [templates...]
```

**Common Actions:**

- `add` - Add issue templates to `.github/ISSUE_TEMPLATE/`
- `list` - Show available issue templates
- `preview` - Preview template content

**Example:**

```bash
gh-templates issue add rust python --dir templates/
```

[→ Full Issue Documentation](./issue/issue.md)

---

### License Templates (`license`)

Add popular open-source licenses to your repository.

```bash
gh-templates license <action> [options] [licenses...]
```

**Common Actions:**

- `add` - Add license files to your repository
- `list` - Show available licenses
- `preview` - Preview license text

**Example:**

```bash
gh-templates license add mit apache-2.0 -o LICENSE
```

[→ Full License Documentation](./license/license.md)

---

### Pull Request Templates (`pr`)

Create standardized pull request templates for better code review workflows.

```bash
gh-templates pr <action> [options] [templates...]
```

**Common Actions:**

- `add` - Add PR templates to `.github/`
- `list` - Show available PR templates  
- `preview` - Preview template content

**Example:**

```bash
gh-templates pr add default detailed --force
```

[→ Full PR Documentation](./pr/pr.md)

---

### Gitignore Templates (`gitignore`)

Generate language and framework-specific `.gitignore` files.

```bash
gh-templates gitignore <action> [options] [templates...]
```

**Common Actions:**

- `add` - Add `.gitignore` files to repository root
- `list` - Show available gitignore templates
- `preview` - Preview gitignore content

**Example:**

```bash
gh-templates gitignore add rust node python
```

[→ Full Gitignore Documentation](./gitignore/gitignore.md)

## Universal Options

These options work across all commands:

| Option | Description |
|--------|-------------|
| `--help`, `-h` | Show help information |
| `--version`, `-V` | Display version number |
| `--build-info` | Show detailed build information |

## Common Patterns

### List Available Templates

```bash
gh-templates issue list
gh-templates license list  
gh-templates pr list
gh-templates gitignore list
```

### Preview Before Adding

```bash
gh-templates issue preview rust
gh-templates license preview mit
gh-templates pr preview default
gh-templates gitignore preview python
```

### Add Multiple Templates

```bash
gh-templates issue add bug feature enhancement
gh-templates gitignore add python javascript docker
```

### Force Overwrite Existing Files

```bash
gh-templates license add mit --force
gh-templates pr add default --force
```

## Quick Reference

| Task | Command |
|------|---------|
| Add Rust issue template | `gh-templates issue add rust` |
| Add MIT license | `gh-templates license add mit` |
| Add default PR template | `gh-templates pr add default` |
| Add Python gitignore | `gh-templates gitignore add python` |
| Preview any template | `gh-templates <type> preview <name>` |
| List all templates | `gh-templates <type> list` |
| Custom output location | `gh-templates <type> add <name> --dir path/` |
| Custom file name | `gh-templates <type> add <name> -o filename` |

## Getting Help

For detailed help on any command:

```bash
# General help
gh-templates --help

# Subcommand help
gh-templates issue --help

# Action-specific help  
gh-templates issue add --help
```
