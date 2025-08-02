---
title: "gh-templates pr"
sidebar_label: "pr"
---

# Pull Request Templates

The `pr` subcommand provides functionality for managing pull request templates. PR templates help standardize pull request descriptions, ensuring consistent information for code reviews and project documentation.

## Usage

```bash
gh-templates pr <COMMAND>
```

## Available Commands

| Command | Description |
|---------|-------------|
| `add` | Add one or more PR templates to the repository |
| `list` | List available PR templates |
| `preview` | Preview a specific PR template |

## Examples

### List Available Templates

```bash
gh-templates pr list
```

### Preview a Template

```bash
gh-templates pr preview default
```

### Add Single Template

```bash
gh-templates pr add default
```

### Add Multiple Templates

```bash
gh-templates pr add default <template2> <template3>
```

## Output Location

By default, PR templates are saved to `.github/` directory in your repository root. GitHub recognizes templates in this location automatically.

Common file names:

- `pull_request_template.md` (default template)
- `PULL_REQUEST_TEMPLATE.md` (alternative naming)
- Templates in `.github/PULL_REQUEST_TEMPLATE/` (multiple templates)

## Template Structure

PR templates typically include sections for:

- **Description**: Summary of changes
- **Type of Change**: Bug fix, feature, documentation, etc.
- **Testing**: How the changes were tested
- **Checklist**: Pre-submission checklist
- **Related Issues**: Links to related issues
- **Screenshots**: Visual changes (for UI updates)

## Next Steps

- [Add PR Templates](./pr-add.md)
- [List PR Templates](./pr-list.md)
- [Preview PR Templates](./pr-preview.md)
