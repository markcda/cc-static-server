# cc-static-server

Simple static server. Features:

- running on top of `cc-server-kit` (see [configuration example](https://github.com/markcda/cc-server-kit?tab=readme-ov-file#4-quick-start-steps))
- serves all your files from `dist` or `/usr/local/frontend-dist/` folder
- when receives any request other than `/`, it returns `index.html`, excluding files

## Build

This project is supporting [Deployer](https://github.com/impulse-sw/deployer). You can build server with:

```bash
deployer build
```

Or, alternatively, just build with `cargo`:

```bash
cargo build --release
```
