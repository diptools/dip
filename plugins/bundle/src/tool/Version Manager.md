
This plugin manages runtimes with different versions. Inspired by asdf and implemented in Rust.

## Specifications

- It should install dependencies
- It should switch versions
- It should export PATH in shell
- It should clean dependencies

### Dependencies

- [[docs/handbook/Product/Framework/Plugins/Configuration]]

### Configuration

#### Global

`$DATA_DIR/dip/bundle.toml`

```toml
bundle_root = "./bundle"

[vm.runtime]
tailwindcss = ["3.2.4"] # ["default-version", ..other-versions]
```

### Installation destination

`$DATA_DIR/dip/bundle/installs/<version>/<runtime>`

### Supported runtime
	
- Tailwind CSS
- Node
	- [Mirror Registry](https://registry.npmmirror.com/binary.html?path=node/)
- Yarn ?
- Pnpm ?
- Go ?
- Python ?
- Ruby ?
- Java ?

### Events

#### ApplyBundle 

- Check binary for each (runtime x version) matrix
- Clean installs which removed from config file
- Install all newly added versions
- Create shim file

#### CleanBundle ?

- Cleanup `~/.dip/bundle/installs/` ?

### Shims

Path: `$DATA_DIR/dip/shims/`

```sh
#!/bin/sh 

"$HOME/Library/Application Support/dip/bundle/installs/<runtime>/x.y.z/bin/<bin-name>"
```

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
