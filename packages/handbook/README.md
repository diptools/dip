<div align="center">
	<h1>dip handbook</h1>
	<p>All our knowledge in one place.</p>
</div>

![Obsidian vault menu](./assets/images/obsidian/handbook.png)
- Handbook first approach
- Single source of truth
- Edit easily with [Obsidian](https://obsidian.md/)
- Git version control
- [🗂 Index](🗂%20Index.md)
- [🛒 Glossary](🛒%20Glossary.md)

> Follow installation steps and start browsing [here](👋%20Getting%20Started.md).
> If you don't find what you need, PRs or [GitHub discussions](https://github.com/diptools/dip/discussions) are welcome.

## Installation

Install Obsidian (macOS) and GitHub CLI
```sh
brew install obsidian gh
```

Clone repository
```sh
gh repo clone diptools/dip
# or if you have access to diptools/handbook-internal repository
gh repo clone diptools/dip -- --recursive
 
cd dip
```

## Configuration

### Open Vault
1. Start Obsidian app
2. Click "Open folder as vault" -> "Open"
3. Select `packages/handbook` directory in dip repository

![Obsidian vault menu](./assets/images/obsidian/vault-menu.png)
### Enable Community plugins
1. Open "Settings" (`⌘ + ,`) -> "Options" -> "Community Plugins" -> "Turn on community plugins"
2. Click "Community plugins" -> "Browse"
	- ![Enable third party plugin](./assets/images/obsidian/community-plugins.png)   
3. Find these plugins
	- [Obsidian Git](https://github.com/denolehov/obsidian-git)
	- [Linter](https://github.com/platers/obsidian-linter)
4. Click "Install" and "Enable"
	- ![Obsidian Git Plugin](./assets/images/obsidian/obsidian-git-plugin.png)

### Branch / Pull / Commit / Push
1. Press command `⌘ + P`
2. Search for `git`
	![Obsidian git commands](./assets/images/obsidian/git-commands.png)
### Editing Internal handbook
There are two ways to commit changes and push to handbook-internal repository
1. Directly open `packages/handbook/Inside/` directory with Obsidian
	- Follow [Open Vault](#open-vault) step but this time, select `packages/handbook/Internal` directory instead in step 3
2. Via git command line tool
```sh
cd packages/handbook/Internal

git add -A
git commit -m "Your commit message"
git push
```
