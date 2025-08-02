---
title: "gh-templates issue preview"
sidebar_label: "issue preview"
---

# Preview Issue Templates

Preview the content of one or more issue templates before adding them to your repository.

## Usage

```bash
gh-templates issue preview [TEMPLATES]...
```

## Arguments

| Argument        | Description                        |
|-----------------|------------------------------------|
| `[TEMPLATES]...`| Names of templates to preview      |

## Options

| Option           | Description   |
|------------------|--------------|
| `-h, --help`     | Print help   |

## Examples

### Preview a Single Template

```bash
gh-templates issue preview bug
```

This displays the complete content of the "bug" issue template.

### Preview Multiple Templates

```bash
gh-templates issue preview bug feature
```

Shows the content of both the "bug" and "feature" templates in sequence.

### Preview All Templates

```bash
gh-templates issue preview
```

When no template names are provided, this may show a selection prompt or preview all available templates.

## Sample Output

```bash
$ gh-templates issue preview bug

# Bug Report Template
name: 🐛 Bug Report
description: Report unexpected behavior, failures, or issues in the project.
title: "[Bug]: "
labels:
   - bug
body:
   - type: dropdown
      id: bug-category
      attributes:
         label: Bug Category
         description: Please select the most appropriate category for this bug. If none apply, select "_No Response_"
         options:
         ...
```

## Available Templates

To see all available templates, use:

```bash
gh-templates issue list
```

Sample output:

```
> bug - Bug Report Template
> chore - Chore Issue Template
> community - Report issues or suggestions related to community, collaboration, or project governance.
> docs - Report issues or suggest improvements related to documentation, guides, or help content.
> dx - Report issues that affect developers' experience
> feature - Suggest a new feature or improvement for a project.
> refactor - Refactor Issue template for GitHub
> support - Ask a question or request support (not for bugs or feature requests)
> technical-debt - Technical Debt Issue Template
> test - Report issues related to testing or quality assurance.
```

## Why Preview?

Previewing templates is useful for:

1. **Understanding Content**: See exactly what will be added to your repository.
2. **Choosing Templates**: Compare different templates to find the best fit.
3. **Customization Planning**: Identify what might need to be modified after adding.
4. **Avoiding Conflicts**: Check if templates match your project's style.

## Template Structure

Issue templates are always written in YAML format. They define all content and structure using YAML fields, following GitHub's issue forms specification.

- **Metadata**: Fields like `name`, `description`, `title`, and `labels` are specified at the top of the YAML file.
- **Form Fields**: The `body` section contains definitions for form elements such as dropdowns, checkboxes, input fields, and more, using GitHub's issue form syntax.

## Best Practices

1. **Preview Before Adding**: Always preview templates to understand their YAML structure and content.
2. **Compare Options**: Preview multiple templates to select the most suitable one for your workflow.
3. **Check Compatibility**: Ensure the template's YAML format matches your project's requirements and GitHub's issue forms specification.

## Next Steps

After previewing templates:

1. **Add the template** if you like it:

    ```bash
    gh-templates issue add bug
    ```

2. **Explore alternatives** if needed:

    ```bash
    gh-templates issue list
    ```

## Related Commands

- [List Issue Templates](./issue-list.md) - See all available templates
- [Add Issue Templates](./issue-add.md) - Add templates to your repository
