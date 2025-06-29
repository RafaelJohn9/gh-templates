---
name: 🐞 Bug Fix
about: Submit a fix for a known bug
title: "[BUGFIX] <short summary>"
labels: bugfix
---

## 🔗 Related Issue(s)

- Fixes #<issue_number>  

> Please ensure the bug is tracked in an issue. If not, create one first.

---

## 🧠 Solution Summary

> Describe **what change** you made and **why**.
>
> Focus on the reasoning or technical approach.

Example:
> Refactored the state handling logic in the checkout form to prevent submission when shipping is undefined. Added a guard clause to catch this condition.

---

## 🔬 Testing Strategy

How did you test the fix? Include test cases or manual steps.

- [ ] Added/updated unit tests
- [ ] Ran manual tests
- [ ] Validated with affected team/user

Optional:

```bash
# Steps to validate the fix
npm install
npm run dev
```
