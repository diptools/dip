+++
title = "Website"
weight = 2
+++

## Prerequisites

- [Zola](https://github.com/getzola/zola): A fast static site generator in a single binary with everything built-in.

  ```sh
  brew install zola
  ```

## Website

```sh
# Build Tailwind CSS
dip build -p docs/website

# Serve locally
zola -r docs/website serve --drafts
```
