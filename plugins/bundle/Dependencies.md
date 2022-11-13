
### Platform

- macOS
- Linux?
- Windows?
	- Probably via WSL 2 is enough

### Plugins

- UI
	- CliPlugin
- Utilities
	- AsyncActionPlugin
	- ConfigPlugin
- Bundle specific
	- GeneratorPlugin
		- [cargo-generate/cargo-generate](https://github.com/cargo-generate/cargo-generate)
	- BundlePlugin
		- [rust-shell-script/rust_cmd_lib](https://github.com/rust-shell-script/rust_cmd_lib)?
		- [BurntSushi/walkdir](https://github.com/BurntSushi/walkdir)
		- Features
			- default = ["full"]
			- full = ["brew", "asdf", "cargo", "os"]
			- brew = []
			- asdf = []
			- cargo = []
			- os = []
		- ToolPlugin
			- HomebrewPlugin
				- Package manager for macOS
					- [Homebrew/brew](https://github.com/Homebrew/brew)
					- [Homebrew/homebrew-bundle](https://github.com/Homebrew/homebrew-bundle)
			- AsdfPlugin
				- [asdf-vm/asdf](https://github.com/asdf-vm/asdf)
			- CargoPlugin
			- OsSettingsPlugin
				- macOSSettings Plugin
