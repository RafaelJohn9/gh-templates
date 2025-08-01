---
title: "gh-templates license add"
sidebar_label: "license add"
---

# Add License Templates

Add one or more license templates to your repository.

## Usage

```bash
gh-templates license add [OPTIONS] [LICENSE]...
```

## Arguments

| Argument | Description |
|----------|-------------|
| `[LICENSE]...` | License IDs to add (e.g., mit, apache-2.0, gpl-3.0) |

## Options

| Option | Description |
|--------|-------------|
| `--dir <DIR>` | Directory to save the license templates |
| `--force` | Force overwrite existing license template files |
| `--all` | Download all available license templates |
| `-i, --interactive` | Interactive mode for filling placeholders |
| `--update-cache` | Update the license templates cache |
| `--param [<KEY=VALUE>...]` | Additional parameters for license placeholders (key=value format) |
| `-o, --output <OUTPUT>...` | Output file names for the templates (in order of templates) |
| `-h, --help` | Print help |

## Examples

### Add a Single License

```bash
gh-templates license add mit
```

This downloads the MIT license and saves it as `LICENSE` in the repository root.

### Interactive License Creation

```bash
gh-templates license add mit  -i
```

Prompts for placeholder values interactively:

```
Enter value for 'copyright holders': <YOUR NAME>
Enter value for 'year': <2025>
✓ Filled 2 out of 2 placeholder(s).
⚠ Please carefully review the license text above for any missed or incorrect placeholders.
✓ LICENSE - has been added.
```

### License with Parameters

```bash
gh-templates license add mit --force --dir . --param copyright-holders="YOUR NAME" --param year="2025"
```

Provides placeholder values directly through parameters.

### Add Multiple Licenses

```bash
gh-templates license add mit apache-2.0 -o LICENSE-MIT LICENSE-APACHE-2.0
```

Downloads multiple license templates (useful for dual-licensing).

### Custom Output Directory

```bash
gh-templates license add mit --dir legal/
```

Saves the license to a custom directory.

### Custom File Names

```bash
gh-templates license add mit apache-2.0 -o LICENSE.MIT -o LICENSE.APACHE
```

Specify custom output file names for each license.

### Force Overwrite

```bash
gh-templates license add mit --force
```

Overwrites existing license files without prompting.

### Add All Licenses

```bash
gh-templates license add --all
```

Downloads all available license templates (not recommended).

### Complex Example

```bash
gh-templates license add mit apache-2.0 --dir licenses/ --force -o MIT.txt -o APACHE-2.0.txt
```

This command:

- Adds MIT and Apache 2.0 licenses
- Saves to `licenses/` directory
- Forces overwrite of existing files
- Uses custom file names with `.txt` extension

## Default Behavior

- **Output Directory**: Repository root (`.`)
- **File Names**: `LICENSE` for single license, or license-specific names for multiple
- **Overwrite**: Prompts before overwriting existing files (unless `--force` is used)

## License Customization

Licenses often contain placeholders that need to be filled:

1. You can use `--interactive` mode to be prompted for each placeholder
2. You can use `--param key=value` to provide values for placeholders
3. Common placeholders include:
    - `copyright-holders`: Your name or organization
    - `year`: Current year or appropriate range
    - `project-name`: Your project name

Example of customized MIT license:

```
Copyright (c) 2025 YOUR NAME

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software")...
```

## Tips

1. **Choose Carefully**: License choice affects how others can use your code
2. **Single License**: Most projects use one primary license
3. **Dual Licensing**: Some projects offer multiple license options
4. **Consult Legal**: For commercial projects, consider legal consultation
5. **Placement**: Keep license files in the repository root for visibility

## Common License Choices

| License | Use Case |
|---------|----------|
| MIT | Simple, permissive open-source |
| Apache-2.0 | Open-source with patent protection |
| GPL-3.0 | Copyleft, requires derivatives to be open-source |
| BSD-3-Clause | Permissive with attribution requirement |
