# Gridlock architecture decisions

## ADR 0001 — Start with a Sway adapter behind a compositor boundary

- **Status:** Provisional
- **Date:** 2026-10-23
- **Decision:** Use Sway as the initial Wayland compositor and isolate its IPC
  and command vocabulary behind a Gridlock compositor API. Keep Sway-specific
  types out of plugin interfaces.
- **Reason:** The FreeBSD Handbook documents Sway installation and operation.
  Its IPC exposes the window tree, workspaces, outputs, events, and commands
  needed for an initial adapter.
- **Consequence:** Behavior and version compatibility still need verification
  against the Sway package on a selected FreeBSD release.

## ADR 0002 — GTK4 is the initial shell UI candidate

- **Status:** Provisional
- **Date:** 2026-10-23
- **Decision:** Prefer GTK4 for shell and settings UI where it works on the
  selected FreeBSD release. Keep the shell surface integration replaceable
  until layer-shell support is verified on target.
- **Reason:** GTK4 and a GTK4 layer-shell port are listed in current FreeBSD
  package/ports references. This does not establish package availability on
  every target architecture/release or runtime compatibility.
- **Consequence:** No GTK application or Wayland layer surface is claimed yet.

## ADR 0003 — Pointer centering is a release gate for directional focus

- **Status:** Provisional
- **Date:** 2026-10-23
- **Decision:** Do not ship Super+arrow navigation as satisfying the brief
  until the pointer reaches the newly focused window center after successful
  focus, and remains still if focus fails.
- **Candidate mechanism:** Current upstream Sway documentation describes
  `seat <seat> cursor set <x> <y>` as an absolute global-coordinate cursor
  operation; that command is marked deprecated in favor of the virtual-pointer
  Wayland protocol. The current Sway IPC accepts Sway commands, so this is a
  candidate to test, not proof for the FreeBSD packaged Sway build.
- **Consequence:** Verify coordinates, races, multi-output layouts, constraints,
  and deprecation behavior on the selected package. If unsupported, evaluate a
  documented compositor extension rather than fake input.

## ADR 0004 — Keep initial implementation and runtime unprivileged

- **Status:** Accepted
- **Date:** 2026-10-23
- **Decision:** Run the shell, agent, plugin host, session manager, and CLI as
  the logged-in user. Any privileged operation must be isolated in a narrowly
  scoped helper and require explicit authorization.
- **Reason:** Limits impact from plugins and agent actions and follows the
  project brief's security requirements.
