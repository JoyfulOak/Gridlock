# Gridlock status

Updated: 2026-10-23

## Current milestone

Phase 0 is incomplete; Phase 1 implementation has started. Initial source exists,
but it has not been build-verified and does not yet form a runnable desktop.

## Repository audit

- Initial repository contained `README.md` and `.gitattributes`; no Rust
  workspace, tests, package metadata, or local `AGENTS.md` instructions were found.
- Git was clean at start on branch `main`, at commit `7934cd8` (`Initial commit`).
- No existing implementation or user data was found in the repository. The
  documentation additions in this milestone are non-destructive.

## Host and target evidence

| Item | Observed | Status |
|---|---|---|
| Current host OS | macOS 27.0.1, Darwin 27 | Observed; not a FreeBSD target |
| Host CPU/hardware | Apple M4 Pro, arm64 Mac mini | Observed; not evidence for FreeBSD aarch64 support |
| Display | 1920×1080 external display, 120 Hz | Observed on macOS only |
| FreeBSD release / ABI | Not available (`freebsd-version` absent) | Untested |
| Sway | Not installed / `sway --version` unavailable | Untested |
| Wayland client development package | `pkg-config wayland-client` unavailable | Untested |
| GTK4 on this host | pkg-config reports 4.22.1 | Host-only observation |
| GTK4 layer-shell on FreeBSD | FreeBSD port reference found | Package/runtime availability on target unverified |
| FreeBSD graphics/input | No FreeBSD kernel, GPU driver, seat, or input stack to inspect | Untested |
| UTM | Not established | Untested |

## Feasibility research

- The FreeBSD Handbook documents Wayland and Sway setup, and identifies the
  `video` group and `evdev` support as relevant to compositor input. This shows
  an established path to investigate, not compatibility for an unspecified
  release or hardware target.
- The FreeBSD ports tree contains `x11-wm/sway`; the GTK4 layer-shell port is
  also listed. Exact binary package availability must be queried on the target
  ABI and selected package branch.
- Current upstream Sway IPC exposes command execution, window trees, workspaces,
  outputs, and subscriptions. Current upstream input documentation describes
  `seat cursor set` but marks it deprecated in favor of virtual-pointer protocol.
  The command and protocol support of the eventual FreeBSD package is unverified.
- Sources checked: [FreeBSD Handbook: Wayland](https://docs.freebsd.org/en/books/handbook/wayland/),
  [FreeBSD ports: Sway](https://cgit.freebsd.org/ports/tree/x11-wm/sway),
  [FreeBSD ports reference: GTK4 layer-shell](https://www.freshports.org/x11-toolkits/gtk4-layer-shell/),
  [Sway IPC manual](https://github.com/swaywm/sway/blob/master/sway/sway-ipc.7.scd),
  and [Sway input manual](https://github.com/swaywm/sway/blob/master/sway/sway-input.5.scd).

## Blockers and risks

1. A supported FreeBSD release and a real or suitable virtual target are needed
   to finish Phase 0. The current macOS host cannot validate FreeBSD graphics,
   seat/input, packages, or Sway behavior.
2. The required pointer-centering behavior is a release gate. Upstream documents
   a candidate cursor-set command, but it is deprecated and target package
   behavior has not been exercised.
3. FreeBSD aarch64 and UTM graphics/session behavior remain untested.
4. GTK4 layer-shell and Rust dependency/package compatibility must be confirmed
   for the selected FreeBSD release and architecture.

## Work completed so far

- Inspected repository, git state, host OS/hardware, and available Rust/Sway/
  Wayland/GTK development tools without installing packages or changing system
  configuration.
- Added `PLAN.md`, `DECISIONS.md`, `STATUS.md`, and `KNOWN_LIMITATIONS.md`.
- Added workspace crates for core, Sway integration, CLI, session startup, and
  the first shell status provider, plus Plugin SDK and plugin host crates.
- Implemented versioned TOML config parsing, directional target selection,
  Sway IPC tree reading and command sending, pointer-centering candidate,
  visible-window filtering, workspace/window CLI actions, unit tests for core
  geometry/config and plugin manifest behavior, and starter Sway bindings.
- Implemented plugin manifest validation and local install/list/inspect/
  enable/disable/remove commands. Install/remove ask for typed confirmation;
  install copies files only and leaves the plugin disabled.
- Added an example data-only plugin and the initial SDK manifest guide.
- Updated README to accurately describe the repository state.

## Commands and results

- `git status --short --branch` — clean at start (`main...origin/main`).
- `git log -1 --oneline` — `7934cd8 Initial commit`.
- `uname -a` / `sw_vers` — macOS 27.0.1 on arm64.
- `rustc --version`, `cargo --version`, `sway --version`, and Wayland pkg-config
  query — unavailable on PATH / not installed.
- `pkg-config --modversion gtk4` — `4.22.1` (macOS host only).
- `git diff --check` — passed; trailing-whitespace scan returned no matches.
- Python is 3.9.6, which lacks the standard-library `tomllib` parser used in an
  attempted static TOML syntax check; no Cargo metadata parse could be run.

No Rust build or target test was possible in this environment. The core unit
tests are authored but not run. Rust (`rustc`/`cargo`) is not installed, and the
host does not have the target Sway/Wayland stack.

## Next step

Make the workspace buildable and resolve compiler diagnostics on a Rust-enabled
host, then run Phase 0 on a selected FreeBSD release/ABI. Capture exact Sway and
GTK4 layer-shell package versions, and exercise input plus pointer centering
before relying on compositor-dependent features.
