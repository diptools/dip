## User story
As a developer, I want a handbook that describes everything about dip organization. Everyone related to this project even not an engineer should be able to read or edit easily.

## Features
- Single source of truth for a distributed community
  - Inspired by [Gitlab Handbook](https://about.gitlab.com/handbook/)
	  <span>
		  <iframe id="ytplayer" type="text/html" width="640" height="360"
	  src="https://www.youtube.com/embed/3HHyjAV3hYE?origin=http://example.com"
	  frameborder="0"></iframe>
	  </span>
	  
  - Docs, wikis, meeting minutes, everything should be documented here
  - If you get questions from people, add them to handbook right after
- Markdown + git
  - No vendor lock-in
  - Flexible technical stack
- Obsidian support
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