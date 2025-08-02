---
title: "gh-templates issue list"
sidebar_label: "issue list"
---

# List Issue Templates

List all available issue templates that can be added to your repository.

## Usage

```bash
gh-templates issue list
```

## Options

| Option           | Description   |
|------------------|---------------|
| `-h, --help`     | Print help    |

## Examples

### List All Available Templates

```bash
gh-templates issue list
```

This command displays all available issue templates with their filenames and descriptions.

## Sample Output

```
bug - Bug Report Template
chore - Chore Issue Template
community - Report issues or suggestions related to community, collaboration, or project governance.
docs - Report issues or suggest improvements related to documentation, guides, or help content.
dx - Report issues that affect developers' experience
feature - Suggest a new feature or improvement for a project.
refactor - Refactor Issue template for GitHub
support - Ask a question or request support (not for bugs or feature requests)
technical-debt - Technical Debt Issue Template
test - Report issues related to testing or quality assurance.
```

## Understanding Template Names

Template filenames (e.g., `bug.yml`, `feature.yml`) can be used directly with the `add` command:

```bash
# Use any template filename from the list
gh-templates issue add bug
gh-templates issue add feature
```

## Template Categories

Templates are typically organized into categories:

- **General Purpose**: `bug.yml`, `feature.yml`, `chore.yml`, `refactor.yml`, `technical-debt.yml`, `test.yml`
- **Community & Support**: `community.yml`, `support.yml`
- **Documentation & Developer Experience**: `docs.yml`, `dx.yml`

## Next Steps

After reviewing the available templates:

1. **Preview templates** you're interested in:

   ```bash
   gh-templates issue preview bug
   ```

2. **Add templates** to your repository:

   ```bash
   gh-templates issue add bug feature
   ```

> **Info:** When specifying a template name with the `add` command, you can include the `.yml` extension or omit it. Both `gh-templates issue add bug` and `gh-templates issue add bug.yml` are valid.

## Related Commands

- [Preview Issue Templates](./issue-preview.md) - Preview template content
- [Add Issue Templates](./issue-add.md) - Add templates to your repository
