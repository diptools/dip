<div align="center">
	<h1>dip handbook</h1>
	<p>All our knowledge in one place.</p>
</div>

![Obsidian vault menu](./assets/images/obsidian/handbook.png)
- Handbook first approach
- Single source of truth
- Edit easily with [Obsidian](https://obsidian.md/)
- Git version control

> Follow [installation](#Installation) and [configuration](#Configuration) steps and start browsing here

- [👋 Getting Started](👋%20Getting%20Started)
- [🗂 Index](🗂%20Index.md)%20steps
- [🛒 Glossary](🛒%20Glossary.md)

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

## Obsidian Git

- Branch
- Pull
- Commit
- Push

1. Press command `⌘ + P`
2. Search for `git`
	![Obsidian git commands](./assets/images/obsidian/git-commands.png)
