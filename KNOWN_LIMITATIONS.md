# Known limitations

Updated: 2026-10-23

- Gridlock has build-verified macOS release binaries, but no FreeBSD package or
  FreeBSD runtime test yet.
- `gridlock-shell` is currently a Hyprland status provider; it is not yet the
  graphical top bar, launcher, menus, settings UI, or notification center.
- The Hyprland adapter, session, and Waybar profile have not been run on
  FreeBSD hardware. Pointer positioning via Hyprland's `movecursor` dispatcher
  and its coordinate semantics are unverified. The profile lacks a power menu
  and native Gridlock launcher.
- Waybar's network indicator uses FreeBSD `route` and `ifconfig`; the audio
  widget expects Waybar's `sndio` module. Device and port feature availability
  must be checked on the target machine.
- The plugin SDK manifest parser and local file manager exist, but execution,
  runtime permission enforcement, dependency resolution, signing, repository
  support, and rollback are not implemented yet.
- Agent service, crash monitor, settings app, and FreeBSD platform integrations
  are not implemented yet.
- The host workspace tests pass on macOS 27.0.1 / Apple Silicon. `cargo check`
  passes for FreeBSD amd64, but no FreeBSD release or architecture has been
  built and run natively.
- The current host does not have Hyprland or a FreeBSD Wayland session, so runtime
  behavior must be checked in the VM.
- Hyprland pointer centering is not validated; availability and coordinate
  semantics for its cursor dispatcher in the target FreeBSD package are unknown.
- FreeBSD package availability for Hyprland, Rust dependencies, GTK4, and GTK4
  layer-shell is not yet verified against a selected release and package ABI.
- FreeBSD graphics acceleration, seat/input access, multi-monitor behavior,
  aarch64 support, and UTM behavior are untested.
- Gridlock has no custom password greeter or authentication code. FreeBSD
  instructions now document SDDM as an optional system login manager and the
  Gridlock Wayland session entry, but this boot/login flow has not been tested
  on a FreeBSD target. No default-session or passwordless auto-login setting
  is installed.
- No feature should be described as working until it has been exercised on a
  supported FreeBSD target.
