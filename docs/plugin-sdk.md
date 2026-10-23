# Gridlock Plugin SDK (initial manifest draft)

This version defines the local plugin package metadata. It is not yet a runtime
ABI: plugins cannot contribute widgets or execute inside Gridlock.

## Package format

Each package is a directory with a `plugin.toml` file at its root. The initial
manifest fields are:

- `id`, `name`, and `version`
- `api.min` / `api.max` SDK compatibility range
- `runtime`: `data`, `executable`, or `wasm`
- optional relative `entrypoint` for executable runtimes
- optional supported `architectures`, required desktop version, dependency
  list, permissions, and configuration schema

The current API version is `1`. The example data-only plugin is at
`plugins/examples/clock`.

## Local lifecycle

`gridlock-plugin-host inspect <directory>` validates and prints a manifest.
`install <directory>` displays the plugin and requested permissions and requires
typing `install`; it copies regular files without running scripts and leaves the
plugin disabled. `list`, `enable`, `disable`, and `remove` manage the local
installation. Removing a plugin removes its installed package directory only.

## Current security boundary

Plugins are not launched or loaded by this implementation. The `enabled` marker
is registry state only. Runtime isolation, permission checks on every capability,
dependency handling, content hashes/signatures, and rollback must exist before
executable plugin support is added. Never treat the manifest permission list as
enforcement by itself.
