# Known limitations

Updated: 2026-10-23

- Gridlock has initial Rust source but no build-verified binaries or packages.
- There is no generated `Cargo.lock` yet; dependency resolution and the
  reproducible lockfile are pending a Cargo-enabled build.
- `gridlock-shell` is currently a Sway status provider; it is not yet the
  graphical top bar, launcher, menus, settings UI, or notification center.
- The plugin SDK manifest parser and local file manager exist, but execution,
  runtime permission enforcement, dependency resolution, signing, repository
  support, and rollback are not implemented yet.
- Agent service, crash monitor, settings app, and FreeBSD platform integrations
  are not implemented yet.
- The repository has only been inspected on macOS 27.0.1 / Apple Silicon; no
  FreeBSD release or architecture has been built or run.
- Sway and Wayland development tools are not installed on the current host.
- Sway pointer centering is not validated. Current upstream docs describe an
  absolute cursor-set command but deprecate it in favor of a virtual-pointer
  protocol; support in the target FreeBSD package is unknown.
- FreeBSD package availability for Sway, Rust dependencies, GTK4, and GTK4
  layer-shell is not yet verified against a selected release and package ABI.
- FreeBSD graphics acceleration, seat/input access, multi-monitor behavior,
  aarch64 support, and UTM behavior are untested.
- No feature should be described as working until it has been exercised on a
  supported FreeBSD target.
