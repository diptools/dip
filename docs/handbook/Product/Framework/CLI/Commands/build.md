A command to build application.

### Example
```sh
dip build -p examples/todomvc
```

### Options

- -p, --path: project path
- -c, --config: Tailwind CSS config path
- -i, --input: Tailwind CSS input file path
- -o, --output: Tailwind CSS output path
- -w, --watch: Watch file changes and re-compile Tailwind CSS

### Tailwind CSS support
- It compiles css file when `<project-root>/tailwind.config.js` exists
- It installs tailwind cli binary if not installed
	- `$HOME/.dip/installed/tailwindcss`
