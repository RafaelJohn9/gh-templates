# GitHub Templates

This repository provides a collection of reusable GitHub templates to streamline your workflow across multiple repositories. It includes fully supported **Issue Templates**, **Pull Request Templates**, and more.

## Features

- **Issue Templates**: Standardized templates for bug reports, feature requests, documentation, community collaboration, developer experience feedback, support questions, and tests.
- **Pull Request Templates**: Easily add consistent PR templates to your repositories.
- **Easy Installation**: Quickly set up templates using a provided installation script.
- **Customizable**: All templates can be tailored to fit your project's needs.

## Installation

### ISSUE TEMPLATES

To install the issue templates in your repository, run:

```sh
wget -qO- https://raw.githubusercontent.com/rafaeljohn9/github-templates/main/scripts/install_issue_template.sh | sh
```

#### Installing Specific Templates

If you only want to install certain issue templates instead of all, you can use the `install_issue_template.sh` script and specify the template filenames as arguments. For example:

```sh
wget -O install_issue_template.sh https://raw.githubusercontent.com/rafaeljohn9/github-templates/main/scripts/install_issue_template.sh
chmod +x install_issue_template.sh
./install_issue_template.sh 01-bug.yml 02-feature-request.yml
```

Replace the filenames with the templates you need. Available templates include:

- `01-bug.yml`
- `02-feature-request.yml`
- `03-documentation.yml`
- `04-community-collaboration.yml`
- `05-developer-experience-feedback.yml`
- `06-support-question.yml`
- `07-test.yml`

### Usage

After installation, the templates will be available in your repository's `.github/ISSUE_TEMPLATE` directory. GitHub will automatically use these templates when users create new issues or pull requests.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have suggestions or improvements.
