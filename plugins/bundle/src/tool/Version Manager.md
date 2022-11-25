
This plugin manages runtimes with different versions. Inspired by asdf and implemented in Rust.

## Specifications

- It should install dependencies
- It should switch versions
- It should export PATH in shell
- It should clean dependencies

### Dependencies

- [[docs/handbook/Product/Framework/Plugins/Configuration]]

### Configuration

`dip.toml`

```toml
[vm.runtime]
tailwindcss = ["3.2.4"] # ["default-version", ..other-versions]
```

### Installed destination

- `$HOME/.dip/bundle/`
	- `installed/`
		- `node/`

### Binary PATH setup
 `~/.zshrc`
 
```sh
dip bundle init
```

### Supported runtime
	
- Tailwind CSS
- Todo
	- Node
		- [Mirror Registry](https://registry.npmmirror.com/binary.html?path=node/)
	- Yarn
	- Python
	- Ruby
	- Java

### Versions

#### Set Current Version

- `dip bundle global name@version`: `$HOME/.dip/dip.toml`
- `dip bundle shell name@version`: `DIP_BUNDLE_${RUNTIME}_VERSION`
- `dip bundle local name@version`: `$PWD/bundle/vm/versions.toml` ?

#### Supported versions

- `10.15.0`
- `ref:v1.0.2` ?
- `path:/src/elixir` ?
- `system` ?

### Hooks

#### Apply

- Check binary for each runtime x version matrix
- Clean installs which removed from config file
- Install all newly added versions
- Set current version

#### Clean

- Cleanup `~/.dip/bundle/installs`

## Unsolved topics 

- [x] Install binary or from source
	- [[#References]]
- [x] Where to lookup config file?
	- [[#Configuration]]
- [x] Where to store current version?
	- [[#Set Current Version]]

## References

- [Building a Go Version Manager (GVM)](https://benjiv.com/building-a-go-version-manager/)
- [Thoughts on Package Managers: Source vs. Binary](https://www.linux.com/training-tutorials/thoughts-package-managers-source-vs-binary/#:~:text=Source%20package%20include%20a%20tarball,takes%20everything%20out%20of%20it.)
- [Create a Plugin - asdf](https://asdf-vm.com/plugins/create.html)
- [bin/install - asdf-nodejs](https://github.com/asdf-vm/asdf-nodejs/blob/master/bin/install)
- [Set Current Version](https://asdf-vm.com/manage/versions.html#set-current-version)
- [`.tool-versions`](https://asdf-vm.com/manage/configuration.html#tool-versions)
