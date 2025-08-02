---
title: "gh-templates pr preview"
sidebar_label: "pr preview"
---

# Preview PR Templates

Preview the content of one or more pull request templates before adding them to your repository.

## Usage

```bash
gh-templates pr preview [TEMPLATES]...
```

## Arguments

| Argument | Description |
|----------|-------------|
| `[TEMPLATES]...` | Names of PR templates to preview |

## Options

| Option | Description |
|--------|-------------|
| `-h, --help` | Print help |

## Examples

### Preview a Single Template

```bash
gh-templates pr preview default
```

This displays the complete content of the default PR template.

### Preview Multiple Templates

```bash
gh-templates pr preview default detailed simple
```

Shows the content of multiple templates for comparison.

### Compare Template Complexity

```bash
gh-templates pr preview simple detailed
```

Compare a minimal template with a comprehensive one.

## Sample Output

```bash
$ gh-templates pr preview default

=== Preview: default ===

## Description

Please include a summary of the changes and the related issue. Please also include relevant motivation and context. List any dependencies that are required for this change.

Fixes # (issue)

## Type of change

Please delete options that are not relevant.

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] This change requires a documentation update

## How Has This Been Tested?

Please describe the tests that you ran to verify your changes. Provide instructions so we can reproduce. Please also list any relevant details for your test configuration

- [ ] Test A
- [ ] Test B

**Test Configuration**:
* Firmware version:
* Hardware:
* Toolchain:
* SDK:

## Checklist:

- [ ] My code follows the style guidelines of this project
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published in downstream modules
```

## Why Preview PR Templates?

Previewing PR templates helps you:

1. **Understand Content**: See what information the template requests
2. **Choose Appropriately**: Select templates that match your workflow
3. **Plan Customization**: Identify sections to modify for your project
4. **Compare Options**: Evaluate different templates side by side

## Template Structure Analysis

When previewing, pay attention to:

### Standard Sections

- **Description**: Summary of changes
- **Type of Change**: Categorization checkboxes
- **Testing**: How changes were verified
- **Checklist**: Pre-submission requirements

### Advanced Sections

- **Breaking Changes**: Impact assessment
- **Dependencies**: Required changes in other systems
- **Screenshots**: Visual documentation
- **Performance Impact**: Performance considerations

## Template Complexity Levels

### Simple Templates

- Minimal sections
- Quick to fill out
- Good for small changes

### Standard Templates

- Balanced information request
- Covers most use cases
- Good default choice

### Detailed Templates

- Comprehensive documentation
- Suitable for major changes
- Thorough review process

## Customization Planning

After previewing, consider customizing:

1. **Project-Specific Sections**: Add relevant fields
2. **Team Workflow**: Match your review process
3. **Tool Integration**: Add links to project tools
4. **Compliance Requirements**: Add regulatory checkboxes

## Best Practices

1. **Preview Before Committing**: Always preview templates before adding
2. **Compare Similar Templates**: Look at different complexity levels
3. **Consider Your Team**: Choose templates that match team preferences
4. **Think About Enforcement**: Consider which sections are required vs. optional

## Common Template Features

Most PR templates include:

| Feature | Purpose |
|---------|---------|
| Description field | Summary of changes |
| Change type checkboxes | Categorize the PR |
| Testing section | Document verification |
| Checklist | Ensure completeness |
| Issue linking | Connect to related issues |

## Next Steps

After previewing templates:

1. **Add the template** if it meets your needs:

   ```bash
   gh-templates pr add default
   ```

2. **Customize after adding** to match your project:
   - Update project-specific sections
   - Modify checklists for your workflow
   - Add team-specific requirements

3. **Try multiple templates** for different scenarios:

   ```bash
   gh-templates pr add feature bugfix hotfix
   ```

## Related Commands

- [List PR Templates](./pr-list.md) - See all available templates
- [Add PR Templates](./pr-add.md) - Add templates to your repository
