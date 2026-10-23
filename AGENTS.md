# Instructions for coding agents

Before making project changes, read the complete product and implementation
brief in [`docs/gridlock-coding-agent-prompt.md`](docs/gridlock-coding-agent-prompt.md).
That file is the canonical Gridlock specification and contains the full product
requirements, architecture, security rules, phase plan, acceptance criteria,
and definition of done.

## Working rules

- Treat the brief as the project requirements. Do not silently omit requirements
  or claim the full desktop is complete while any definition-of-done item remains.
- Inspect the current repository and working tree before editing. Preserve
  existing user work. Ask before destructive changes.
- Work in small, reviewable milestones. Prefer completing implementation over
  returning a plan. Keep `PLAN.md`, `DECISIONS.md`, `STATUS.md`, and
  `KNOWN_LIMITATIONS.md` current as work proceeds.
- Do not install packages or change the OS, bootloader, login manager, services,
  or privileged system configuration without first showing the exact commands
  and obtaining approval when the change is privileged or potentially disruptive.
- Never run the Gridlock shell or agent as root. Keep optional AI providers
  offline-safe and do not transmit user data without explicit opt-in.
- Treat Hyprland pointer-centering support, GTK4 layer-shell support, FreeBSD
  graphics/input, and every claimed CPU architecture as unverified until tested
  on the actual supported FreeBSD target. Do not substitute a weaker behavior
  and label it as meeting acceptance criteria.
- At each stopping point, update `STATUS.md` with changed files, exact commands
  and outcomes, limitations, and the next concrete step. Clearly distinguish
  authored tests from tests that were actually run.
- Read `docs/install-freebsd.md` before giving installation steps. It describes
  the current prototype only; it is not evidence that Gridlock is a finished or
  packaged desktop environment.

## Current repository state

The repository is an early prototype. Its Rust workspace, Hyprland adapter, CLI,
session launcher, status provider, and local plugin manifest tools still need a
successful build and FreeBSD target validation. The full graphical shell,
launcher, settings, notifications, plugin execution/isolation, agent service,
crash monitor, FreeBSD integrations, packaging, and recovery workflow remain
incomplete. Confirm the current state from `STATUS.md` rather than relying on
this snapshot.
