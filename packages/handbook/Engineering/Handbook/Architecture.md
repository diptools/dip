### Spec

#### Repositories
- dip
	- linked to handbook-private via submodule 
	- linked path is `packages/handbook/private`
- handbook-private
	- stores private documents
	- private repo
	- submodule for dip repository
	- should be the same file structure as public so that it can be merged easily

#### Build flow
- private production
	- git clone with submodule
- public production
	- git clone without submodule
- convert obsidian flavored markdown to standard one 
	- https://github.com/zoni/obsidian-export
		- provides CLI command also with underlying lib
			- either introduce new dip command with obsidian-export lib
			- or simply uses their cli?
- build website with Zola based on exported markdown files
	- which path?
- deploy via Vercel CLI from a custom GitHub Action

### Implementation steps
1. prepare dip repo for new submodule
	1. mkdir `packages/handbook`
	2. mkdir `packages/handbook/internal`
2. create new repo `handbook-internal`
	1. same directory structure as public one
3. 