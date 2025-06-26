# -----------------------------------------------------------------------------
# install_issue_template.sh
#
# This script automates the installation of GitHub issue templates into a local
# repository. It downloads a predefined set of issue template YAML files from a
# remote GitHub repository and places them in the `.github/ISSUE_TEMPLATE`
# directory of the current project.
#
# Usage:
#   ./install_issue_template.sh
#
# Features:
#   - Creates the `.github/ISSUE_TEMPLATE` directory if it does not exist.
#   - Downloads multiple issue template files and a configuration file from a
#     specified remote repository.
#   - Ensures robust execution with strict error handling (`set -euo pipefail`).
#
# Requirements:
#   - curl: Command-line tool for downloading files.
#
# Environment Variables:
#   REPO_BASE_URL: Base URL of the remote repository containing the templates.
#
# Output:
#   - Issue template files are saved to `.github/ISSUE_TEMPLATE/`.
#   - Informational messages are printed to the console.
#
# Author: rafaeljohn9
# -----------------------------------------------------------------------------
#!/usr/bin/env bash

set -euo pipefail

REPO_BASE_URL="https://raw.githubusercontent.com/RafaelJohn9/github-templates/main"
TEMPLATE_DIR=".github/ISSUE_TEMPLATE"
ALL_FILES=(
  "01-bug.yml"
  "02-feature-request.yml"
  "03-documentation.yml"
  "04-community-collaboration.yml"
  "05-developer-experience-feedback.yml"
  "06-support-question.yml"
  "07-test.yml"
)

# If arguments are provided, use them as the list of files to download
if [[ $# -gt 0 ]]; then
  FILES=()
  for arg in "$@"; do
    if [[ " ${ALL_FILES[*]} " == *" $arg "* ]]; then
      FILES+=("$arg")
    else
      echo "⚠️  Warning: '$arg' is not a recognized template. Skipping."
    fi
  done
  if [[ ${#FILES[@]} -eq 0 ]]; then
    echo "❌ No valid templates specified. Exiting."
    exit 1
  fi
else
  FILES=("${ALL_FILES[@]}")
fi

echo "📁 Creating template directory: $TEMPLATE_DIR"
mkdir -p "$TEMPLATE_DIR"

for file in "${FILES[@]}"; do
  echo "⬇️  Downloading $file"
  curl -sSfL "$REPO_BASE_URL/templates/ISSUE_TEMPLATE/$file" -o "$TEMPLATE_DIR/$file"
done

echo "✅ GitHub issue templates installed to '$TEMPLATE_DIR'."
