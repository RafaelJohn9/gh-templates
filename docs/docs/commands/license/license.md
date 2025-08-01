---
title: "gh-templates license"
sidebar_label: "license"
---

# License Templates

The `license` subcommand provides functionality for managing license templates. License templates help you quickly add popular open-source licenses to your projects with proper formatting and legal text.

## Usage

```bash
gh-templates license <COMMAND>
```

## Available Commands

| Command | Description |
|---------|-------------|
| `add` | Add one or more license templates to the repository |
| `list` | List available license templates |
| `preview` | Preview a specific license template |

## Examples

### List Available Licenses

```bash
gh-templates license list
```

### Preview a License

```bash
gh-templates license preview mit
```

### Add Single License

```bash
gh-templates license add mit
```

### Add Multiple Licenses

```bash
gh-templates license add mit apache-2.0 -o LICENSE-MIT LICENSE-APACHE-2.0
```

## Popular License Types

Common licenses available include:

- **MIT**: Simple and permissive license
- **Apache-2.0**: Apache License 2.0 with patent protection
- **GPL-3.0**: GNU General Public License v3.0
- **BSD-3-Clause**: 3-clause BSD license
- **ISC**: Internet Systems Consortium license
- **Unlicense**: Public domain dedication

## Output Location

By default, license files are saved to the repository root as `LICENSE` (or with appropriate extensions). This follows standard conventions for license placement.

## License Customization

Many license templates include placeholder fields that may need customization:

- **Copyright holder name**: Your name or organization
- **Year**: Current year or range of years
- **Project name**: Name of your project

## Next Steps

- [Add License Templates](./license-add.md)
- [List License Templates](./license-list.md)
- [Preview License Templates](./license-preview.md)
