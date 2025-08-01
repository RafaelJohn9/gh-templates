---
title: "gh-templates license preview"
sidebar_label: "license preview"
---

# Preview License Templates

Preview the content and details of a license template before adding it to your repository.

## Usage

```bash
gh-templates license preview [OPTIONS] <LICENSE>
```

## Arguments

| Argument | Description |
|----------|-------------|
| `<LICENSE>` | License ID (e.g. mit, apache-2.0) |

## Options

| Option | Description |
|--------|-------------|
| `-d, --description` | Show license description |
| `-p, --permissions` | Show permissions granted by the license |
| `-l, --limitations` | Show limitations of the license |
| `-c, --conditions` | Show conditions of the license |
| `-D, --details` | Show all details (description, permissions, limitations, conditions) |
| `-u, --update-cache` | Update the license cache |
| `-h, --help` | Print help |

## Examples

### Preview a License

```bash
gh-templates license preview mit
```

This displays the complete text of the MIT license.

### Preview License with Description

```bash
gh-templates license preview --description mit
```

Shows the MIT license along with its description.

### Preview All License Details

```bash
gh-templates license preview --details apache-2.0
```

Shows the Apache 2.0 license text with description, permissions, limitations, and conditions.

### Preview Specific License Details

```bash
gh-templates license preview --permissions --limitations gpl-3.0
```

Shows the GPL-3.0 license text along with its permissions and limitations.

## Sample Output

```bash
$ gh-templates license preview mit

=== Preview: MIT License ===

MIT License

Copyright (c) [year] [fullname]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Why Preview Licenses?

Previewing licenses is important for:

1. **Understanding Terms**: Know exactly what rights and restrictions apply
2. **Comparing Options**: Review licenses to choose the best fit
3. **Legal Compliance**: Ensure you understand the legal implications
4. **Placeholder Identification**: See what fields need customization

## Common Placeholders

Many license templates include placeholders that need customization:

- `[year]` or `[YEAR]`: Replace with current year
- `[fullname]` or `[NAME]`: Replace with your name or organization
- `[project]` or `[PROJECT]`: Replace with your project name
- `[email]` or `[EMAIL]`: Replace with your contact email

## Understanding License Details

The details options help you understand:

- **Description**: Brief explanation of the license
- **Permissions**: What users can do with your code
- **Limitations**: What users cannot do with your code
- **Conditions**: Requirements users must meet when using your code

## Best Practices

1. **Read Carefully**: Don't just skim; understand the full text
2. **Seek Advice**: For commercial projects, consider legal consultation
3. **Match Project Needs**: Choose licenses that align with your goals
4. **Consider Community**: Some communities prefer specific licenses

## Next Steps

After previewing licenses:

1. **Add the license** if it meets your needs:

   ```bash
   gh-templates license add mit
   ```

2. **Customize placeholders** after adding:
   - Update copyright holder name
   - Set appropriate year(s)
   - Add project-specific information

3. **Explore alternatives** if needed:

   ```bash
   gh-templates license list
   ```

## Related Commands

- [List License Templates](./license-list.md) - See all available licenses
- [Add License Templates](./license-add.md) - Add licenses to your repository
