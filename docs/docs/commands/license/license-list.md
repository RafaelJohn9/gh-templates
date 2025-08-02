---
title: "gh-templates license list"
sidebar_label: "license list"
---

# List License Templates

List all available license templates that can be added to your repository.

## Usage

```bash
gh-templates license list [OPTIONS] [ARGS]...
```

## Options

| Option | Description |
|--------|-------------|
| `-p, --popular` | Show only popular/common licenses |
| `-n, --non-software` | Show non-software licenses |
| `-s, --search <SEARCH>` | Search for licenses containing this term, can contain wildcard |
| `--include-deprecated` | Show deprecated licenses as well |
| `--update-cache` | Update the license cache before listing |
| `--osi-approved` | Show only OSI-approved licenses |
| `--fsf-libre` | Show only FSF libre-approved licenses |
| `-h, --help` | Print help |

## Examples

### List All Available Licenses

```bash
gh-templates license list
```

This command displays all available license templates with their names and descriptions.

### List Only Popular Licenses

```bash
gh-templates license list --popular
```

### Search for Specific Licenses

```bash
gh-templates license list --search mit
```

### List OSI-approved Licenses

```bash
gh-templates license list --osi-approved
```

## Sample Output

```
Available License Templates:

mit                   MIT License - Simple and permissive
apache-2.0           Apache License 2.0 - Permissive with patent protection
gpl-3.0              GNU General Public License v3.0 - Strong copyleft
bsd-3-clause         3-Clause BSD License - Permissive with attribution
bsd-2-clause         2-Clause BSD License - Simplified BSD
isc                  ISC License - Simplified permissive license
mpl-2.0              Mozilla Public License 2.0 - Weak copyleft
lgpl-3.0             GNU Lesser General Public License v3.0
unlicense            The Unlicense - Public domain dedication
cc0-1.0              Creative Commons Zero v1.0 Universal
```

## Understanding License Names

License names shown in the list can be used directly with the `add` command:

```bash
# Use any license name from the list
gh-templates license add mit
gh-templates license add apache-2.0
gh-templates license add gpl-3.0
```

## License Categories

Licenses are typically categorized by their restrictions:

### Permissive Licenses

- **MIT**: Most permissive, allows almost anything
- **Apache-2.0**: Permissive with patent protection
- **BSD-3-Clause**: Permissive with attribution requirement
- **ISC**: Simple and permissive

### Copyleft Licenses

- **GPL-3.0**: Strong copyleft, derivatives must be open-source
- **LGPL-3.0**: Lesser GPL, allows linking with proprietary code
- **MPL-2.0**: Weak copyleft, file-level copyleft

### Public Domain

- **Unlicense**: Releases code to public domain
- **CC0-1.0**: Creative Commons public domain dedication

## Choosing a License

Consider these factors when selecting a license:

1. **Project Type**: Open-source library vs. application
2. **Commercial Use**: Whether you want to allow commercial use
3. **Attribution**: Whether you require attribution
4. **Copyleft**: Whether derivatives must remain open-source
5. **Patent Rights**: Whether you need patent protection

## Quick Reference

| Need | Recommended License |
|------|-------------------|
| Maximum freedom | MIT |
| Patent protection | Apache-2.0 |
| Keep derivatives open | GPL-3.0 |
| Public domain | Unlicense |
| Simple permissive | ISC |

## Next Steps

After reviewing the available licenses:

1. **Preview licenses** you're considering:

   ```bash
   gh-templates license preview mit
   ```

2. **Add a license** to your repository:

   ```bash
   gh-templates license add mit
   ```

3. **Compare licenses** by previewing multiple options:

   ```bash
   gh-templates license preview mit apache-2.0
   ```

## Related Commands

- [Preview License Templates](./license-preview.md) - Preview license content
- [Add License Templates](./license-add.md) - Add licenses to your repository
