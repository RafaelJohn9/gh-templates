---
title: "gh-templates pr add"
sidebar_label: "pr add"
---

# Add PR Templates

Add one or more pull request templates to your repository.

## Usage

```bash
gh-templates pr add [OPTIONS] [TEMPLATE]...
```

## Arguments

| Argument | Description |
|----------|-------------|
| `[TEMPLATE]...` | PR template names to add (e.g., default, detailed, simple) |

## Options

| Option | Description |
|--------|-------------|
| `--dir <DIR>` | Directory to save the PR templates |
| `--force` | Force overwrite existing PR template files |
| `--all` | Download all available PR templates |
| `-o, --output <OUTPUT>...` | Output file names for the templates (in order of templates) |
| `-h, --help` | Print help |

## Examples

### Add a Single Template

```bash
gh-templates pr add default
```

This downloads the default PR template and saves it as `pull_request_template.md` in `.github/`.

### Add Multiple Templates

```bash
gh-templates pr add default <template2> <template3>
```

Downloads multiple PR templates for different use cases.

### Custom Output Directory

```bash
gh-templates pr add default --dir .github/PULL_REQUEST_TEMPLATE/
```

Saves templates to the standard multiple-template directory.

### Custom File Names

```bash
gh-templates pr add default -o feature_pr.md 
```

Specify custom output file names for each template.

### Force Overwrite

```bash
gh-templates pr add default --force
```

Overwrites existing PR template files without prompting.

### Add All Templates

```bash
gh-templates pr add --all
```

Downloads all available PR templates.

### Complex Example

```bash
gh-templates pr add default detailed --dir .github/ --force -o pull_request_template.md detailed_pr_template.md
```

This command:

- Adds default and detailed PR templates
- Saves to `.github/` directory
- Forces overwrite of existing files
- Uses custom output file names

## Default Behavior

- **Output Directory**:
  - `.github/` (for the default template)
  - `.github/PULL_REQUEST_TEMPLATE/` (for other templates)
- **File Names**:
  - Single template: `pull_request_template.md`
  - Multiple templates: Template-specific names
- **Overwrite**: Prompts before overwriting existing files (unless `--force` is used)

## Template Placement Strategies

### Single Template

For repositories with one PR template:

```bash
gh-templates pr add default -o pull_request_template.md --dir .
```

### Multiple Templates

For repositories with multiple PR types:

```bash
gh-templates pr add default <template2> <template3> --dir .github/PULL_REQUEST_TEMPLATE/
```

This creates multiple templates that users can choose from when creating PRs.

## GitHub Integration

GitHub automatically uses PR templates when:

1. **Single Template**: `pull_request_template.md` in `.github/` or repository root
2. **Multiple Templates**: Templates in `.github/PULL_REQUEST_TEMPLATE/` directory
3. **User Selection**: GitHub shows a dropdown for multiple templates

## Template Customization

After adding PR templates, consider customizing:

1. **Project-specific Sections**: Add sections relevant to your workflow
2. **Checklist Items**: Customize the checklist for your review process
3. **Links and References**: Update links to match your project structure
4. **Team Mentions**: Add default reviewers or team mentions

## Tips

1. **Start Simple**: Begin with a basic template and iterate
2. **Team Input**: Get feedback from team members on template content
3. **Regular Updates**: Update templates as your process evolves
4. **Multiple Types**: Use different templates for different change types
5. **Clear Instructions**: Include clear guidance for contributors

## Common Template Sections

Most PR templates include:

- **Summary**: Brief description of changes
- **Motivation**: Why these changes are needed
- **Testing**: How changes were tested
- **Checklist**: Pre-submission requirements
- **Type of Change**: Bug fix, feature, breaking change, etc.
- **Related Issues**: Links to issues addressed
