# Gridlock

An original, keyboard-first tiling desktop environment for FreeBSD.

## Project status

Gridlock is in early Phase 1. It contains a Rust workspace and an initial
Sway-backed CLI/session slice, but it has not been built or exercised on
FreeBSD, and it is not yet a complete desktop environment.

See [STATUS.md](STATUS.md) for the host audit, evidence, and current blockers;
[PLAN.md](PLAN.md) for milestones; [DECISIONS.md](DECISIONS.md) for architecture
decisions; and [KNOWN_LIMITATIONS.md](KNOWN_LIMITATIONS.md) for unverified or
unsupported areas.

## Build (once Rust is available)

```sh
cargo build --workspace
cargo test --workspace
```

These commands have not yet been run. Sway IPC commands require a running Sway
session with its socket exported in `SWAYSOCK`.

The current CLI includes `gridlock status`, `gridlock config validate`, basic
window/workspace controls, `gridlock terminal`, and `gridlock plugin ...`.
Plugin package details and current runtime limits are in
[docs/plugin-sdk.md](docs/plugin-sdk.md).
