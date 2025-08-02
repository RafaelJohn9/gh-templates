---
title: "gh-templates pr list"
sidebar_label: "pr list"
---

# List PR Templates

List all available pull request templates that can be added to your repository.

## Usage

```bash
gh-templates pr list
```

## Options

| Option | Description |
|--------|-------------|
| `-h, --help` | Print help |

## Examples

### List All Available Templates

```bash
gh-templates pr list
```

This command displays all available PR templates with their names and descriptions.

## Sample Output

```
Available PR Templates:

default               Standard pull request template with common sections
detailed              Comprehensive PR template with extensive documentation
simple                Minimal PR template for quick changes
feature               Template specifically designed for new features
bugfix                Template for bug fix pull requests
hotfix                Template for urgent production fixes
security              Template for security-related changes
documentation         Template for documentation updates
refactor              Template for code refactoring PRs
performance           Template for performance improvements
```

## Understanding Template Names

Template names shown in the list can be used directly with the `add` command:

```bash
# Use any template name from the list
gh-templates pr add default
gh-templates pr add feature
gh-templates pr add bugfix
```

## Template Categories

PR templates are typically organized by purpose:

### General Purpose

- **default**: Standard template suitable for most PRs
- **simple**: Minimal template for straightforward changes
- **detailed**: Comprehensive template for complex changes

### Change Type Specific

- **feature**: New functionality additions
- **bugfix**: Bug fixes and patches
- **hotfix**: Urgent production fixes
- **refactor**: Code restructuring without behavior changes
- **performance**: Performance optimizations

### Domain Specific

- **security**: Security-related changes
- **documentation**: Documentation updates
- **ci/cd**: Build and deployment changes

## Choosing the Right Template

Consider these factors when selecting PR templates:

| Use Case | Recommended Template |
|----------|-------------------|
| General development | `default` |
| Quick fixes | `simple` |
| Major features | `detailed` or `feature` |
| Bug fixes | `bugfix` |
| Emergency fixes | `hotfix` |
| Code cleanup | `refactor` |
| Docs updates | `documentation` |

## Multiple Template Strategy

Many projects use multiple templates:

```bash
# Add templates for different change types
gh-templates pr add feature bugfix hotfix --dir .github/PULL_REQUEST_TEMPLATE/
```

This creates a dropdown in GitHub where contributors can select the appropriate template.

## Template Selection Workflow

With multiple templates:

1. **Contributor** creates a new PR using template as a parameter (`https://github.com/owner/repo/compare/main...branch?template=feature.md`)
2. **Template** pre-fills PR description
3. **Contributor** completes template sections

## Next Steps

After reviewing the available templates:

1. **Preview templates** you're interested in:

   ```bash
   gh-templates pr preview default
   ```

2. **Add templates** to your repository:

   ```bash
   gh-templates pr add default feature bugfix
   ```

3. **Compare templates** by previewing multiple options:

   ```bash
   gh-templates pr preview simple detailed
   ```

## Best Practices

1. **Start with One**: Begin with a single `default` template
2. **Evolve Gradually**: Add more specific templates as needs arise
3. **Team Consensus**: Ensure the team agrees on template content
4. **Regular Review**: Update templates based on team feedback
5. **Clear Naming**: Use descriptive names for multiple templates

## Related Commands

- [Preview PR Templates](./pr-preview.md) - Preview template content
- [Add PR Templates](./pr-add.md) - Add templates to your repository
