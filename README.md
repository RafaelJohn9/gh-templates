# GitForge (formerly GH Templates)

> [!NOTE]
> This project is currently being renamed and restructured.  
> Some documentation and package references may still use the old name  
> **`gh-templates`** until the migration is complete.

GitForge is a developer-friendly toolkit for managing and applying reusable **GitHub repository templates** — including **Issue Templates**, **Pull Request Templates**, **.gitignore files**, **Licenses**, and more.  
It helps you quickly scaffold standard project configurations without repetitive manual setup.

---


[![GitHub Release](https://img.shields.io/github/v/release/rafaeljohn9/gh-templates?label=GitHub%20Release)](https://github.com/rafaeljohn9/gh-templates/releases)
[![License](https://img.shields.io/github/license/rafaeljohn9/gh-templates?label=License)](LICENSE)

[![Crates.io](https://img.shields.io/crates/v/gh-templates?label=crates.io)](https://crates.io/crates/gh-templates)
[![Crate Downloads](https://img.shields.io/crates/d/gh-templates?label=downloads)](https://crates.io/crates/gh-templates)

[![PyPI version](https://img.shields.io/pypi/v/gh-templates?label=PyPI)](https://pypi.org/project/gh-templates/)
[![PyPI downloads](https://static.pepy.tech/personalized-badge/gh-templates?period=month&units=international_system&left_color=black&right_color=blue&left_text=PyPI%20Downloads)](https://pepy.tech/project/gh-templates)

[![npm version](https://img.shields.io/npm/v/gh-templates?label=npm)](https://www.npmjs.com/package/gh-templates)
[![npm downloads](https://img.shields.io/npm/dm/gh-templates?label=npm%20downloads)](https://www.npmjs.com/package/gh-templates)

---

## About

**GitForge** is designed to simplify repository setup and management by offering ready-to-use templates and configuration files.  
Instead of manually creating `.gitignore`, license, and issue templates for each new project, GitForge helps you **generate them instantly** from trusted sources.

It works across different ecosystems and supports multiple package managers — **Rust (Cargo)**, **Python (PyPI)**, and **Node (npm)**.

---

## Contributing

We welcome contributions from everyone — whether you’re improving templates, fixing docs, or enhancing CLI commands.

Most of the repository consists of:
- **YAML files** – for GitHub workflows  
- **Markdown files** – for templates & documentation  
- **License files** – for open source compliance  

Rust, Python, or Node are only used for the CLI layer, so you can contribute without deep language knowledge.

You can help by:
- Opening issues  
- Suggesting new templates  
- Improving command UX or documentation  

---

## Features

- **Reusable Templates:** Standardized templates for issues, PRs, and licenses.  
- **CLI Commands:** Apply templates quickly via a simple command-line interface.  
- **Gitignore Management:** Add, list, or preview `.gitignore` templates from the official GitHub source.  
- **License Templates:** Instantly add popular license files (MIT, Apache, GPL, etc.).  
- **Cross-Platform:** Works on Linux, macOS, and Windows.  
- **Extensible:** Designed to support new template categories in the future (like Codes of Conduct, Security Policies, etc.).

---

## Installation

### Quick Install (Recommended)

#### Using npm:
```bash
npm install -g gitforge
```

#### Using pip:

```
pip install gitforge
```


#### Using Cargo:

```
cargo install gitforge
```



### Example Usage

> [!NOTE]
> Some commands may still appear under the old name gh-templates
> until the migration is finalized.


#### New Command Syntax

```sh
# Add a Python .gitignore
gitforge add gitignore python

# List available gitignore templates
gitforge list gitignores

# Add a bug report issue template
gitforge add issue-template bug

# Preview a pull request template
gitforge preview pr-template

# List available licenses
gitforge list licenses
```

#### Old Syntax (Still Supported Temporarily)

```sh
gh-templates gitignore add python
gh-templates issue add bug
```


### Migration Notice

- GitForge is the next evolution of GH Templates — bringing a cleaner command structure, modern UX, and broader language support.
- You may still see references to gh-templates across:
  - Code imports
  - Package registries (npm, PyPI, crates.io)
  - Docs and badges




These will be updated as the migration completes.


# License:

- Licensed under the Apache 2.0 License.


---

### Key Highlights:
- Renamed to **GitForge**, with a clear **transition notice**.
- Updated **badges, usage examples**, and **command syntax**.
- Introduced new **natural language CLI syntax** (`gitforge add gitignore python`).
- Kept **backward compatibility** note for developers still using `gh-templates`.

---

