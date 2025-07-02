# Changelog

All notable changes to this project will be documented in this file.


## [Unreleased]



### Restructure

- Organize CI templates into group-specific subfolders (2aadf62)


### Chore

- update CHANGELOG [skip ci] (7ef4982)


### Feat

- add Flake8 GitHub Actions workflow for PEP8 compliance (b9a503d)

- add GitHub Actions workflow for Black code formatter (beb7f08)

- add GitHub Actions workflow for Ruff code linter and formatter (6ec904b)

- add GitHub Actions workflow for Pylint code quality check (c72c721)

- add GitHub Actions workflow for Mypy static type checking (022f0f8)

- add GitHub Actions workflow for Pyright type checking (52be534)

- add GitHub Actions workflow for Pytest testing (8f532ca)

- add GitHub Actions workflow for Unittest testing suite (fde7f81)

- add GitHub Actions workflow for Nose2 test suite (afa17a0)

- add GitHub Actions workflow for generating code coverage reports (528a5f1)

- add GitHub Actions workflow for uploading coverage reports to Codecov (8eded48)

- add GitHub Actions workflow for building Python packages with Poetry (f05362d)

- add GitHub Actions workflow for building documentation with MkDocs (bf133db)

- add GitHub Actions workflow for auto-formatting code with Black (6d04102)

- add GitHub Actions workflow for sorting and grouping imports with isort (1d4a216)

- add GitHub Actions workflow for building documentation with Sphinx (fabf852)

- add GitHub Actions workflow for building documentation with MkDocs (b21a6a0)

- add GitHub Actions workflow for Bandit security scan (150f405)

- add GitHub Actions workflow for Safety security scan (a2aef6b)

- add GitHub Actions workflow for pip-audit security scan (e3a8743)

- add GitHub Actions workflow for checking dependency updates (4d039e2)

- add GitHub Actions workflow for pip dependency check (8f32c28)

- add GitHub Actions workflow for matrix installability check across multiple Python/OS versions (619dbcf)

- add GitHub Actions workflow for running pre-commit hooks on Python files (e92382e)

- add GitHub Actions workflow for caching Poetry dependencies (c68d07d)

- add GitHub Actions workflow for caching pip dependencies (d894c1f)

- add GitHub Actions workflow for validating poetry.lock against pyproject.toml (87be0fd)


### Fix

- correct comment formatting in coverage.yml (5e85372)

- update Python version matrix to allow for flexible version specification (d3df8e3)

- update Ruff command to explicitly check code quality (829c847)

- update GitHub Actions workflow to build Python package using setuptools (d9f83d5)


## [v0.0.43]



### Fix

- syntax error in change log update workflow (8bf28b7)


## [v0.0.38]



### Fix

- updated sed command (ed6bf68)


## [v0.0.37]



### Fix

- sed command to correctly update markdown text part (350a7f8)


## [v0.0.36]



### Fix

- handle the sed cmd correctly to correctly update the versions (0891bb1)


## [v0.0.34]



### Fix

- replace the version in both markdown and plain-text (ff6b9d1)


## [v0.0.33]



### Fix

- removed redundant v while giving versions (24c3aae)


## [v0.0.32]



### Fix

- update sed commands (62dee53)


## [v0.0.31]



### Fix

- Escape the forward slashes and parentheses properly in the sed pattern (3df872e)


## [v0.0.3]



### Fix

- both link part and displayed message part will be displayed (3c382e8)


## [v0.0.29]



### Fix

- added a sed command on account of a missing line (a36662c)


## [v0.0.28]



### Chore

- update CHANGELOG [skip ci] (991edf0)


### Feat

- enable dynamic update of version in  README (869aa23)


### Fic

- typo in git cliff toml  and workflow (ed73387)


### Fix

- changelog workflow  and updated git-cliff toml (1e399ca)

- unexpected command in git-cliff (472e8ec)

- git-cliff command: appended Changelog (6a4cc7e)

- renamed git-cliff to cliff storing changes to temp changelog (e668151)

- git ref branch name (f590ee6)

- trigger to happen only  on push (46e6c2e)

- typo in README (3979eb1)


## [v0.0.27]



### Feat

- update remote:list to fetch with comments (19e1b9c)


## [v0.0.26]



### Chore

- update CHANGELOG [skip ci] (ccd9627)

- update CHANGELOG [skip ci] (d0eb53c)

- update CHANGELOG [skip ci] (f1a6551)

- update CHANGELOG [skip ci] (79eedb4)

- update CHANGELOG [skip ci] (30d5991)

- update CHANGELOG [skip ci] (d02c35d)

- update CHANGELOG [skip ci] (dcf5059)

- update CHANGELOG [skip ci] (56b6a0c)


### Feat

- enhanced template files by adding appropriate tags to them (f71dedf)

- added test and workflow that verifies all templates start with a comment in the first line (b99ae07)

- updated the tests to capture all files\n\nUpdated the tests to capture all files instead of failing the first time it finds an invalid template\n (3e3b2ff)


### Fix

- untamper with MPL 2.0  License (391076a)

- space typo (08b2d4b)

- removed sha256 checksums from README (fc50455)

- updated labels to prevent only one being chosen (1bd173f)

- updated README to enable using repo labels (8d50a8b)

- added red triangle to easily spot the file (c905276)

- added comment to the first line of all our templates that were missing (c2ea0cf)


### Refactor

- renamed naming of licenses (9886d80)

- renamed issue templates and added first line as comment (52b85b0)

- renamed pr templates (c5022f6)

- updated the exising issue templates (978d353)


## [v0.0.22]



### Update

- version in README (86174d4)


### Chore

- update CHANGELOG [skip ci] (6c4f705)


### Fix

- refactor from git-templates to gh templates (398af77)


## [v0.0.21]



### Chore

- update CHANGELOG [skip ci] (fd1f611)


## [v0.0.18]



### Fix

- typo in License (7c0581d)


### Update

- Added License (893a227)

- Added Contribution guidelines and Code of Conduct (33561cc)

- Added my email in CodeOfConduct (c930dc0)

- inserted email inside Code Of Conduct (cafdff4)


### Fix

- refactor from git templates to gh templates (921fc5c)


## [v0.0.24]



### Update

- README, showing distribution for git-templates bin (39a409b)

- README to fix typo and maintain consistency in Doc (8c1a2e9)


### Feat

- introduced update change log in its own workflow with its own trigger (c6a6824)


## [v0.0.17]



### Fix

- avoid pushing  on tag ref (821152e)


## [v0.0.16]



### Fix

- added branch ref to allow commiting (5fe9cf1)


## [v0.0.15]



### Feat

- added use of git cliff to update CHANGELOG (8a48f39)


## [v0.0.14]



### Fix

- Build errors due to wrong ext appending (windows build) (0f9794a)


## [v0.0.13]



### Fix

- Upload assets due to permission errors (8bc6431)


## [v0.0.12]



### Fix

- SSL issues during build (c315928)


## [v0.0.11]



### Feature

- Added Issue template yml (7975e6a)

- Copied the Issue templates to .github (3a6b915)

- Add config yml file (e3e13d0)

- Added PR templates and CodeOfConduct template (b4d0542)

- Added Licenses (4b1e114)

- added issue cli (d822523)

- added build artifact yml workflow (9dffcab)


### Fix

- removed ':' inside bug report yml\n\ntrailing ':' was found under body => dropdown => options at Other validations\n (608ef3a)

- Moved required: true to validations (401b1b1)

- Remove typos (1529b51)

- Updated the remote functions to have better error handling when there is network error (c278694)


### Update

- Added README (f38eeef)

- Renamed the issue templates, Added Docs to README, Added issue template script installer (4e3cd70)

- from Github to Git templates && added default ext in  pretty print (5ed8a83)

Generated by git-cliff
