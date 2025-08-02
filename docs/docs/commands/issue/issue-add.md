---
title: "gh-templates issue add"
sidebar_label: "issue add"
---

# Add Issue Templates

Add one or more Issue templates to the repository.

## Usage

```bash
gh-templates issue add [OPTIONS] [TEMPLATE]...
```

## Arguments

| Argument        | Description                                                    |
|-----------------|----------------------------------------------------------------|
| `[TEMPLATE]...` | Template names to add (e.g., rust, python, global/windows)     |

## Options

| Option                        | Description                                                      |
|-------------------------------|------------------------------------------------------------------|
| `--dir <DIR>`                 | Directory to save the issue templates                            |
| `--force`                     | Force overwrite existing issue template files                    |
| `--all`                       | Download all available templates                                 |
| `-o, --output <OUTPUT>...`    | Output file names for the templates (in order of templates)      |
| `-h, --help`                  | Print help                                                       |

## Examples

### Add a Single Template

```bash
gh-templates issue add bug
```

Downloads the bug issue template and saves it to `.github/ISSUE_TEMPLATE/`.

### Add Multiple Templates

```bash
gh-templates issue add bug feature enhancement
```

Downloads multiple templates in one command.

### Custom Output Directory

```bash
gh-templates issue add rust --dir templates/issues/
```

Saves the template to a custom directory.

### Custom File Names

```bash
gh-templates issue add bug feature -o bug_report.md -o feature_request.md
```

Specify custom output file names for each template.

### Force Overwrite

```bash
gh-templates issue add rust --force
```

Overwrites existing template files without prompting.

### Add All Templates

```bash
gh-templates issue add --all
```

Downloads all available issue templates.

### Complex Example

```bash
gh-templates issue add bug feature --dir .github/ISSUE_TEMPLATE/ --force -o bug_report.md -o feature_request.md
```

This command:

- Adds Bug and Feature templates
- Saves to `.github/ISSUE_TEMPLATE/`
- Forces overwrite of existing files
- Uses custom output file names

## Default Behavior

- **Output Directory**: `.github/ISSUE_TEMPLATE/`
- **File Names**: Based on template names (e.g. `bug.md`)
- **Overwrite**: Prompts before overwriting existing files (unless `--force` is used)

## Tips

1. **Preview First**: Use `gh-templates issue preview <template>` to see the content before adding.
2. **Organize Templates**: Use the `--dir` option to organize templates in custom directories.
3. **Batch Operations**: Add multiple related templates in a single command.
4. **Backup Existing**: Consider backing up existing templates before using `--force`.
