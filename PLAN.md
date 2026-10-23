# Gridlock implementation plan

## Phase 0 — Environment audit and feasibility spike

- [x] Inspect the repository and current host without changing system state.
- [x] Confirm upstream FreeBSD documentation describes the Wayland platform.
- [x] Identify Hyprland IPC dispatchers requiring target validation.
- [x] Replace the Sway-first architecture with a Hyprland-backed CLI/session slice.
- [ ] Select the initial FreeBSD release and obtain an amd64 target.
- [ ] Audit target release, architecture, virtualization, graphics, input, and package versions.
- [ ] Start a minimal Hyprland session on the target and verify keyboard and pointer input.
- [ ] Verify directional focus plus pointer centering on the target, including failure cases.
- [ ] Establish an aarch64 target and UTM compatibility evidence.
- [ ] Record supported package versions, protocol versions, and build/runtime dependencies.

**Exit condition:** a target FreeBSD machine can run Hyprland and accept keyboard and
pointer input, and the selected pointer-centering method is exercised against
the packaged Hyprland version. Document any architecture or VM fallback.

**Started:** workspace, core configuration/directional-focus model, Hyprland
IPC client, CLI controls, session launcher, and Hyprland starter config exist.
The earlier Sway implementation was built and tested on macOS and cross-target
type-checked for FreeBSD amd64 and aarch64. The replacement backend still needs
a build check and native FreeBSD compositor/runtime validation.

**Plugin foundation started early:** the SDK manifest schema and a local
install/inspect/list/enable/disable/remove tool exist. Plugin execution,
permission enforcement at runtime, dependency resolution, and rollback remain
unimplemented.

## Phase 1 — Buildable desktop foundation

- Cargo workspace, core models, typed TOML configuration, versioned local IPC,
  session lifecycle, structured logs, and `gridlock` CLI.
- Launch Hyprland and a minimal shell process; document restart and TTY recovery.
- Build and smoke-test on an actual supported FreeBSD target.
- Register a FreeBSD Wayland session entry so a compatible login manager can
  start Gridlock after user authentication.

## Phase 2 — Tiling and keyboard navigation

- Directional focus, center-pointer option, shortcuts, window movement,
  floating/fullscreen behavior, workspaces, and output-aware navigation.
- Test tiled geometry, floating windows, multiple outputs, disappearing targets,
  empty workspaces, and focus races on target hardware.

## Phase 3 — Unified graphical shell

- Top bar, launcher, system menu, shortcut viewer, notifications, themes, and
  keyboard-accessible settings foundation.
- [ ] Validate the Hyprland + Waybar top bar on a FreeBSD target and verify the
  adapter's IPC and pointer-centering behavior.

## Phase 4 — Plugin runtime and SDK

- Versioned manifests and SDK, permissions, isolated execution, lifecycle,
  crash supervision, rollback, and sample plugins.

## Phase 5 — AI provider manager and diagnostics

- Opt-in provider adapters, typed tools, crash diagnostics, redaction, review
  and approval flow, audit log, verification, and rollback where possible.

## Phase 6 — FreeBSD system integrations

- Capability-detected networking, audio, Bluetooth, power, packages, displays,
  input, updates, and system health integrations.

## Phase 7 — Packaging, documentation, and hardening

- FreeBSD packaging, installation and recovery guides, migration paths, CI,
  plugin developer guide, privacy documentation, and security review.

Each milestone must record changed files, exact commands and results, known
limitations, and the next step in `STATUS.md`. A feature is supported only
after it has been exercised on the corresponding FreeBSD target.
