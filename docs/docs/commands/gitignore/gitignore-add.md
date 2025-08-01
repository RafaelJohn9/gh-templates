---
title: "gh-templates gitignore add"
sidebar_label: "gitignore add"
---

# Add Gitignore Templates

Add one or more gitignore templates to your repository.

## Usage

```bash
gh-templates gitignore add [OPTIONS] [TEMPLATE]...
```

## Arguments

| Argument        | Description                                                      |
|-----------------|------------------------------------------------------------------|
| `[TEMPLATE]...` | Gitignore template names to add (e.g., rust, python, global/windows) |

## Options

| Option                        | Description                                                                 |
|-------------------------------|-----------------------------------------------------------------------------|
| `--dir <DIR>`                 | Directory to save the `.gitignore` file                                     |
| `--force`                     | Force overwrite existing `.gitignore` file                                  |
| `--all`                       | Download all available templates                                            |
| `-a, --append`                | Append to the existing `.gitignore` file instead of overwriting             |
| `--update-cache`              | Update the gitignore template cache                                         |
| `-n, --use-remote-name`       | Use the remote template file name as the output file name                   |
| `-o, --output <FILENAME>...`  | Output file name(s) (default: `.gitignore`)                                 |
| `-h, --help`                  | Print help                                                                 |

## Examples

### Add a Single Template

```bash
gh-templates gitignore add rust
```

Downloads the Rust gitignore template and saves it as `.gitignore` in the repository root.

### Add Multiple Templates

```bash
gh-templates gitignore add rust docker vscode
```

Combines multiple gitignore templates into a single comprehensive file.

### Custom Output Directory

```bash
gh-templates gitignore add python --dir backend/
```

Saves the gitignore file to a custom directory (useful for monorepos).

### Custom File Names

```bash
gh-templates gitignore add rust python -o .gitignore.rust -o .gitignore.python
```

Creates separate gitignore files for each template.

### Force Overwrite

```bash
gh-templates gitignore add node --force
```

Overwrites existing `.gitignore` file without prompting.

### Add All Templates

```bash
gh-templates gitignore add --all
```

Downloads and combines all available gitignore templates (creates a very comprehensive file ~ Not Recommended).

### Append to Existing File

```bash
gh-templates gitignore add python --append
```

Appends the Python template to the existing `.gitignore` file.

### Use Remote Template Name

```bash
gh-templates gitignore add rust -n
```

Saves the template using its remote file name (e.g., `Rust.gitignore`).

### Update Template Cache

```bash
gh-templates gitignore add --update-cache
```

Updates the local cache of available gitignore templates.

### Complex Example

```bash
gh-templates gitignore add rust docker windows --dir . --force -o .gitignore
```

This command:

- Adds Rust, Docker, and Windows gitignore patterns
- Saves to repository root directory
- Forces overwrite of existing file
- Uses explicit output filename

## Default Behavior

- **Output Directory**: Repository root (`.`)
- **File Names**: `.gitignore` (single file containing all patterns)
- **Overwrite**: Prompts before overwriting existing files (unless `--force` is used)
- **Append**: By default, overwrites unless `--append` is specified
- **Combination**: Multiple templates are merged into one file

## Template Combination

When adding multiple templates, `gh-templates` intelligently combines them:

1. **Categorization**: Groups patterns by category with comments

## Best Practices for Combining

### Language + OS + Editor

```bash
gh-templates gitignore add python windows vscode
```

### Full Stack Project

```bash
gh-templates gitignore add javascript python docker
```

### Polyglot Repository

```bash
gh-templates gitignore add rust go python java
```

## Monorepo Strategy

For monorepos with multiple projects:

```bash
# Root-level common ignores
gh-templates gitignore add windows macos linux

# Language-specific ignores in subdirectories
gh-templates gitignore add rust --dir rust-service/
gh-templates gitignore add node --dir web-frontend/
gh-templates gitignore add python --dir python-api/
```

## Updating Gitignore

To update an existing `.gitignore`:

1. **Preview changes** first:

   ```bash
   gh-templates gitignore preview rust
   ```

2. **Backup existing** file:

   ```bash
   cp .gitignore .gitignore.backup
   ```

3. **Update with force**:

   ```bash
   gh-templates gitignore add rust --force
   ```

## Common Combinations

| Project Type      | Recommended Templates         |
|-------------------|-----------------------------|
| Rust desktop app  | `rust windows macos linux`  |
| Python web app    | `python docker vscode`      |
| Node.js project   | `node windows macos`        |
| Full-stack app    | `javascript python docker`  |
| Mobile app        | `java kotlin swift xcode`   |
| Data science      | `python jupyter r`          |

## Troubleshooting

### File Already Exists

Without `--force`, you'll be prompted to overwrite. Options:

- Use `--force` to overwrite
- Use `--append` to add to the existing file
- Use different output name with `-o`
- Manually merge the content

### Patterns Not Working

If ignore patterns aren't working:

1. Ensure file is named `.gitignore`
2. Check file is in repository root or appropriate subdirectory
3. Commit the `.gitignore` file
4. Use `git rm --cached <file>` to untrack already-tracked files

## Tips

1. **Start Early**: Add `.gitignore` before committing code
2. **Be Specific**: Choose templates that match your actual tech stack
3. **Review Content**: Preview templates to understand what they ignore
4. **Combine Wisely**: Don't add unnecessary templates that create noise
5. **Update Regularly**: Refresh gitignore as your project evolves
