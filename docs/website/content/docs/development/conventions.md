+++
title = "Conventions"
weight = 3
+++

## Branch name example
```sh
git checkout -b docs/#20-guide-website
```

## Conventional Commits
Make sure to use `convco commit` instead of `git commit` when it should be noted in changelog. [git-cliff](https://github.com/orhun/git-cliff) will automatically generates changelog on conventional-commit message that convco produces.
```sh
convco commit
```
