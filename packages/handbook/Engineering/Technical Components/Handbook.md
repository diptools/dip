## User story
As a developer, I want a handbook that describes everything about dip organization. Everyone related to this project even not an engineer should be able to read or edit easily.

## Features
- Single source of truth
  - Inspired by [Gitlab Handbook](https://about.gitlab.com/handbook/)
  - Docs, wikis, meeting minutes, everything should be documented here
  - If you happened to explain the same thing again and again to different people, it means you are missing handbook page
- Markdown + git
  - No vendor lock-in
  - Flexible technical stack
- Obsidian compatibility
  - Easy reading/editing experience for both dev and non-dev people
- Internal document support

## Setup and Usage
  -  [[README]]

## Specs

#### Repositories
- dip
	- linked to handbook-private via submodule 
	- linked path is `packages/handbook/Internal`
- handbook-internal
	- stores internal documents
	- private repo
	- submodule for dip repository