+++
title = "Website"
weight = 2
+++

## Prerequisites
- [Zola](https://github.com/getzola/zola): A fast static site generator in a single binary with everything built-in.
  ```sh
  brew install zola
  ```
- [Node.js](https://nodejs.org/en/download/): To install Tailwind CSS

## Website

```sh
# Install dependencies
npm i

# Serve locally
zola -r packages/website serve --drafts

# Watch Tailwind CSS
npm run watch

# or build
npm run build
```
