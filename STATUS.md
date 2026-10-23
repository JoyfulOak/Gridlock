# Gridlock status

Updated: 2026-10-23

## Current milestone

Phase 0 is incomplete; Phase 1 implementation has started. The Sway-first
prototype has been replaced by a Hyprland adapter, session default, and
Waybar-based profile. The new workspace passes a host `cargo check`, but no
FreeBSD build or Hyprland graphical session has been run.

## 2026-10-23 update — migrate primary compositor to Hyprland

### Changed files

- Replaced `gridlock-compositor-sway` with `gridlock-compositor-hyprland`;
  updated `Cargo.toml`, `Cargo.lock`, `crates/gridlock-cli/{Cargo.toml,src/main.rs}`,
  `crates/gridlock-shell/{Cargo.toml,src/main.rs}`, and
  `crates/gridlock-session/src/main.rs`.
- Implemented Hyprland IPC discovery from `XDG_RUNTIME_DIR` and
  `HYPRLAND_INSTANCE_SIGNATURE`, `j/clients` parsing, window/workspace actions,
  directional selection, and `focuswindow`/`movecursor` dispatches.
- Added `config/defaults/hyprland.conf`, `config/defaults/waybar.jsonc`,
  `config/defaults/waybar.css`, `scripts/gridlock-network-status.sh`, and the
  single `packaging/freebsd/wayland-sessions/gridlock.desktop` entry. Routed
  Super+Arrow bindings through the Gridlock CLI; moved floating toggle to
  Super+Shift+Space. Removed `config/defaults/sway.conf` and the old session entry.
- Updated the
  canonical prompt, install guide, package notes, `README.md`, `AGENTS.md`,
  `PLAN.md`, `DECISIONS.md`, and `KNOWN_LIMITATIONS.md`.

### Commands and outcomes

- `cargo fmt --all` — failed because rustup has no default toolchain configured.
- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/cargo +stable fmt --all` — passed.
- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/cargo +1.85.0 check --workspace --offline` — passed and refreshed `Cargo.lock` for the renamed backend.
- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/cargo +1.85.0 check --workspace --locked` — passed.
- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/cargo +1.85.0 build --workspace --locked` — passed on macOS.
- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/cargo +stable fmt --all -- --check` — passed.
- `git diff --check` — passed. No tests were run; no packages or system settings were changed.

### Limits and evidence

- This is a macOS host build check, not a FreeBSD build or session test.
  Hyprland/Waybar package availability by release and architecture, graphics,
  seat/input, configuration parsing, and bar modules remain target checks.
- Hyprland's official IPC documentation specifies the instance socket at
  `$XDG_RUNTIME_DIR/hypr/[HIS]/.socket.sock` ([IPC docs](https://wiki.hypr.land/IPC/)).
  Upstream discussion documents `movecursor x y` coordinates, and an upstream
  issue reported that cursor dispatch did not update client pointer state
  ([dispatcher example](https://github.com/hyprwm/Hyprland/issues/724),
  [client-position issue](https://github.com/hyprwm/Hyprland/issues/11611)).
  The implementation therefore remains unverified for pointer-centering
  acceptance until exercised against the supported FreeBSD build.
- Gridlock still lacks the graphical launcher, shortcut overlay, power menu,
  settings UI, notifications, plugin runtime isolation, agent service, crash
  monitor, and FreeBSD platform integrations.

### Next concrete step

Build and launch the session on the intended FreeBSD target. Record release,
architecture, Hyprland/Waybar versions, graphics/seat setup, IPC behavior,
pointer position after focus, and module availability before claiming support.

## 2026-10-23 update — FreeBSD build attempt

### Commands and outcomes

- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/rustup target add --toolchain 1.85.0 x86_64-unknown-freebsd` — installed the standard library component into the temporary Rust toolchain.
- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/cargo +1.85.0 check --workspace --locked --target x86_64-unknown-freebsd` — passed cross-target type checking on macOS.
- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/cargo +1.85.0 build --workspace --locked --target x86_64-unknown-freebsd` — failed at linking. The host Apple linker does not accept the GNU linker flags and FreeBSD libraries required for this target.
- A FreeBSD 15.1-RELEASE-p6 ARM64 VM booted under QEMU using `-snapshot`. Its working tree was copied into the guest. The approved `sudo pkg install -y rust` operation had started but was interrupted before completion; no Gridlock build ran in the VM.
- The temporary VM was shut down. Its overlay was discarded, leaving the original VM image unchanged. No FreeBSD package or system configuration was changed persistently.
- No tests were run.

### Result and next step

The workspace has FreeBSD cross-target type-check evidence for amd64, but no
FreeBSD executable was produced. The ARM64 VM was not used to build Gridlock.
Continue only when a FreeBSD build environment is available and explicitly
resume that target work; record a native FreeBSD build separately from a
cross-target check.

## Repository audit

- Initial repository contained `README.md` and `.gitattributes`; no Rust
  workspace, tests, package metadata, or local `AGENTS.md` instructions were found.
- Git was clean at start on branch `main`, at commit `3adcb16` (`first`).
- No existing implementation or user data was found in the repository. The
  documentation additions in this milestone are non-destructive.

## Host and target evidence

| Item | Observed | Status |
|---|---|---|
| Current host OS | macOS 27.0.1, Darwin 27 | Observed; not a FreeBSD target |
| Host CPU/hardware | Apple M4 Pro, arm64 Mac mini | Observed; not evidence for FreeBSD aarch64 support |
| Temporary Rust toolchains | Rust/Cargo 1.85.0, 1.86.0, and 1.99.1 under `/tmp/gridlock-rustup` and `/tmp/gridlock-cargo` | Used for checks; no system package was installed |
| Display | 1920×1080 external display, 120 Hz | Observed on macOS only |
| FreeBSD release / ABI | Not available (`freebsd-version` absent) | Untested |
| FreeBSD amd64 compilation | `cargo check --workspace --locked --target x86_64-unknown-freebsd` | Passed cross-target type checking; not a FreeBSD build or runtime test |
| FreeBSD aarch64 compilation | `cargo check --workspace --locked --target aarch64-unknown-freebsd` | Passed cross-target type checking; not a FreeBSD build or runtime test |
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

- Copied the complete user-provided coding-agent brief into
  `docs/gridlock-coding-agent-prompt.md` and verified it byte-for-byte against
  the Desktop source using `cmp` and matching SHA-256 hashes.
- Added root `AGENTS.md` so repository coding agents are directed to the full
  brief and current status before making changes; linked both from README.
- Added the current prototype's FreeBSD manual setup guide and the `evdev`
  mapping and explicit exit binding to the starter Sway config.
- Added a FreeBSD Wayland session entry and registration notes. The session
  launcher now defaults to the user's Gridlock Sway config when no argument is
  passed, for display-manager startup. This template has not been exercised on
  a FreeBSD login manager. Added optional SDDM instructions for a graphical
  sign-in screen at boot, with Gridlock selected after authentication; no
  system service was enabled on this macOS host.
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
- Added `Cargo.lock`, fixed a shell title type-inference error found by the first
  build, and formatted the Rust workspace.
- Raised the declared Rust minimum to 1.85 to match Cargo 2024-edition
  transitive dependency manifests in the generated lockfile; Rust 1.85 locked
  workspace check passed.
- Added explicit `/usr/local/bin` installation instructions for the SDDM
  session and refreshed VM steps to use locked Cargo commands and check the
  Sway configuration before starting a graphical login manager.
- Added an example data-only plugin and the initial SDK manifest guide.
- Updated README to accurately describe the repository state.

## Commands and results

- `git status --short --branch` — `main...origin/main`; implementation changes
  in this work session are in the working tree and have not been committed.
- `git log -1 --oneline` — `3adcb16 first`.
- `uname -a` / `sw_vers` — macOS 27.0.1 on arm64.
- `rustc --version` / `cargo --version` — system PATH does not provide Rust;
  temporary Rust/Cargo 1.85.0, 1.86.0, and 1.99.1 toolchains were used from
  `/tmp`.
- `docker run --rm ... rust:1.86 ...` — could not run because the local Docker
  daemon was unavailable.
- `cargo test --workspace` — first attempt found a type inference error in
  `gridlock-shell`; fixed by explicitly typing the sanitized title as `String`.
- `cargo test --workspace --locked` — passed, 7 unit tests total.
- `cargo build --workspace --locked --release` — passed on macOS arm64.
- `cargo fmt --all -- --check` — passed after formatting the workspace.
- `cargo clippy --workspace --all-targets --locked -- -D warnings` — passed.
- `cargo check --workspace --locked --target x86_64-unknown-freebsd` — passed
  cross-target type checking on macOS; no target linking or execution.
- `cargo check --workspace --locked --target aarch64-unknown-freebsd` — passed
  cross-target type checking with Rust 1.99.1; no target linking or execution.
- `cargo check --workspace --locked` with Rust 1.85.0 — passed, validating the
  declared minimum compiler for the locked dependencies.
- `cargo test --workspace --locked` with Rust 1.85.0 — passed, 7 unit tests.
- Release-binary smoke checks for `gridlock config validate`, `gridlock plugin
  list`, and `gridlock-plugin-host list` — passed with temporary config/data
  directories.
- Sway configuration parsing, compositor IPC, cursor warp, graphical startup,
  and SDDM login — not testable on this macOS host; test these in the FreeBSD VM.
- `pkg-config --modversion gtk4` — `4.22.1` (macOS host only).
- `git diff --check` — passed; trailing-whitespace scan returned no matches.
- An initial Rust 1.86 aarch64 FreeBSD standard-library install attempt failed
  because that toolchain had no prebuilt target component; Rust 1.99.1 includes
  it and passed the aarch64 cross-target check.

The workspace is now build- and test-verified on macOS, and type-checks for
FreeBSD amd64 and aarch64. This is not evidence of a successful FreeBSD link,
installation, or graphical session. The current working-tree changes are not
committed or pushed, so a VM checkout from the remote URL will not contain
them yet; copy this working tree to the VM or test after the changes are
published.

## Earlier next step (superseded by the Hyprland migration)

The previous milestone called for Sway VM checks. The current next step is the
Hyprland FreeBSD target validation recorded in the migration update below.

## 2026-10-23 update — experimental Hyprland top bar

### Changed files

- Added `config/defaults/hyprland.conf` with a compact Hyprland setup, keyboard
  workspace/window controls, and Waybar startup.
- Added `config/defaults/waybar.jsonc` and `config/defaults/waybar.css` for a
  dark Gridlock bar with persistent workspaces, focused title, network, sndio
  audio, battery, and clock modules.
- Added executable `scripts/gridlock-network-status.sh`, which reads the
  FreeBSD default route and interface state using `route` and `ifconfig`.
- Updated `gridlock-session` with `--compositor sway|hyprland` selection and
  matching default config paths; added a Gridlock (Hyprland) session entry.
- Updated the FreeBSD install/package docs, `DECISIONS.md`, `PLAN.md`, and
  `KNOWN_LIMITATIONS.md` to document the profile and its current limits.

### Research and verification

- FreeBSD Ports currently lists `x11-wm/hyprland` and `x11/waybar`. The Waybar
  port feature list includes Hyprland workspaces/window title, clock, battery,
  and `sndio`; its network module is Linux-only, which is why this profile uses
  a small FreeBSD command-based custom module. Sources: [FreeBSD Hyprland
  port](https://cgit.freebsd.org/ports/tree/x11-wm/hyprland/), [FreeBSD Waybar
  port](https://cgit.freebsd.org/ports/tree/x11/waybar/), [Waybar FreeBSD
  features](https://www.freshports.org/x11/waybar), and [Hyprland install
  guide](https://wiki.hypr.land/getting-started/installation/).
- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/cargo +1.85.0 check --workspace --locked` — passed.
- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/cargo +1.85.0 build --workspace --locked` — passed on macOS.
- `RUSTUP_HOME=/tmp/gridlock-rustup CARGO_HOME=/tmp/gridlock-cargo /tmp/gridlock-cargo/bin/cargo +stable fmt --all -- --check` — passed after formatting.
- `python3 -m json.tool config/defaults/waybar.jsonc` — passed (JSON syntax).
- `sh -n scripts/gridlock-network-status.sh` — passed (shell syntax).
- `git diff --check` — passed.
- No tests were run. No packages or system settings were changed. Rust 1.85's
  temporary toolchain lacks `cargo-fmt`; formatting used the installed stable
  temporary toolchain instead.
- No FreeBSD target session was available. Hyprland/Waybar startup, graphics,
  input, battery, sndio, font glyphs, and config compatibility remain
  unverified on the target. The profile launches through `gridlock-session`,
  Gridlock CLI still depended on Sway IPC at that milestone. A power menu and
  launcher action were not implemented in that profile.
- No reference screenshot was present in the chat or repository inventory;
  the current palette and layout are an original Omarchy-inspired starting
  point rather than a pixel-matched reproduction.

### Next concrete step

Build and run the Hyprland session entry on the user's FreeBSD target; record
the target release, architecture, graphics/seat setup, and bar module results.
Then add a Hyprland compositor adapter so Gridlock's window/workspace CLI can
control that session. Use the reference image if supplied to tune spacing,
colors, and module placement.
