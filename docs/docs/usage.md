---
title: "Usage Overview"
sidebar_label: "Usage"
---

# Usage Overview

`gh-templates` provides a consistent interface for managing different types of GitHub templates. Each template type has its own subcommand with common operations: `add`, `list`, and `preview`.

## Basic Command Structure

```bash
gh-templates <subcommand> <action> [options] [arguments]
```

## Available Subcommands

| Subcommand | Description |
|------------|-------------|
| `issue` | Manage GitHub issue templates |
| `pr` | Manage pull request templates |
| `license` | Manage license files |
| `gitignore` | Manage `.gitignore` files |

## Common Actions

All subcommands support these actions:

- **`add`** - Add one or more templates to your repository
- **`list`** - List all available templates
- **`preview`** - Preview template content before adding

## Typical Workflow

### 1. Explore Available Templates

Start by listing what templates are available:

```bash
# See available issue templates
gh-templates issue list

# See available licenses
gh-templates license list

# See available gitignore templates
gh-templates gitignore list
```

### 2. Preview Templates

Before adding a template, preview its content:

```bash
# Preview a bug issue template
gh-templates issue preview bug

# Preview MIT license
gh-templates license preview mit
```

### 3. Add Templates

Add templates to your repository:

```bash
# Add a single template
gh-templates issue add bug

# Add multiple templates
gh-templates gitignore add rust python node

# Add all available templates
gh-templates license add mit
```

## Common Options

### Custom Output Directory

Specify where templates should be saved:

```bash
# Save to custom directory (default = .github/ISSUE_TEMPLATE/)
gh-templates issue add rust --dir .

# Save gitignore to root (default behavior)
gh-templates gitignore add python
```

### Custom File Names

Override default file names:

```bash
# Custom output file name (default = LICENSE)
gh-templates license add mit -o LICENSE.txt

# Multiple templates with custom names
gh-templates issue add bug feature -o bug_report.md -o feature_request.md
```

### Force Overwrite

Overwrite existing files:

```bash
# Force overwrite existing templates
gh-templates pr add default --force
```

## Global Options

These options work with any command:

```bash
# Show version
gh-templates --version

# Show build information
gh-templates --build-info

# Get help for any command
gh-templates --help
gh-templates issue --help
gh-templates issue add --help
```

## Example Workflows

### Setting Up a New Repository

```bash
# Add a comprehensive set of templates
gh-templates gitignore add rust
gh-templates license add mit
gh-templates issue add bug feature
gh-templates pr add default
```

### Updating Existing Templates

```bash
# Preview changes first
gh-templates issue preview rust

# Force update if you like the changes
gh-templates issue add rust --force
```

### Batch Operations

```bash
# Add multiple gitignore templates for a polyglot project
gh-templates gitignore add python javascript rust docker

# Add all available issue templates
gh-templates issue add --all
```

## Tips and Best Practices

1. **Preview First**: Always preview templates before adding them to understand their content
2. **Use Descriptive Names**: When using custom output names, choose descriptive filenames
3. **Organize Templates**: Use custom directories to organize different types of templates
4. **Version Control**: Commit template changes to track modifications over time

## Next Steps

Ready to dive deeper? Explore the detailed command references:

- [Issue Templates](./commands/issue/issue.md)
- [License Templates](./commands/license/license.md)
- [PR Templates](./commands/pr/pr.md)
- [Gitignore Templates](./commands/gitignore/gitignore.md)
