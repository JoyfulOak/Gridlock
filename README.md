# Gridlock

An original, keyboard-first tiling desktop environment for FreeBSD.

## Project status

Gridlock is in early Phase 1. It contains a Rust workspace and an initial
Hyprland-backed CLI/session slice, but it has not been built or exercised on
FreeBSD, and it is not yet a complete desktop environment.

See [STATUS.md](STATUS.md) for the host audit, evidence, and current blockers;
[PLAN.md](PLAN.md) for milestones; [DECISIONS.md](DECISIONS.md) for architecture
decisions; and [KNOWN_LIMITATIONS.md](KNOWN_LIMITATIONS.md) for unverified or
unsupported areas. Coding agents should follow [AGENTS.md](AGENTS.md) and the
complete [Gridlock coding-agent brief](docs/gridlock-coding-agent-prompt.md).

## Build (Rust/Cargo 1.85 or newer)

```sh
cargo build --workspace --locked
cargo test --workspace --locked
```

The workspace has previously been built and tested on macOS. The Hyprland
adapter requires a running Hyprland session with `XDG_RUNTIME_DIR` and
`HYPRLAND_INSTANCE_SIGNATURE` set. Native FreeBSD build and runtime checks are
still required.

The current CLI includes `gridlock status`, `gridlock config validate`, basic
window/workspace controls, `gridlock terminal`, and `gridlock plugin ...`.
Plugin package details and current runtime limits are in
[docs/plugin-sdk.md](docs/plugin-sdk.md).

For the current manual FreeBSD prototype setup, see
[docs/install-freebsd.md](docs/install-freebsd.md).
