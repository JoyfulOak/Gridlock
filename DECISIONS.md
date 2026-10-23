# Gridlock architecture decisions

## ADR 0001 — Use Hyprland behind a compositor boundary

- **Status:** Provisional
- **Date:** 2026-10-23
- **Decision:** Use Hyprland as the initial Wayland compositor and isolate its IPC
  and command vocabulary behind a Gridlock compositor API. Keep Hyprland-specific
  types out of plugin interfaces.
- **Reason:** The updated product brief selects Hyprland and its IPC exposes
  clients, workspaces, monitors, events, and dispatchers needed for an adapter.
- **Consequence:** Behavior and version compatibility need verification against
  the Hyprland package on each selected FreeBSD release.

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
- **Candidate mechanism:** The adapter follows Hyprland IPC `dispatch` for
  `focuswindow` and `movecursor`. The movecursor availability and coordinate
  semantics have not been validated on a FreeBSD package.
- **Consequence:** Verify coordinates, races, multi-output layouts, constraints,
  and dispatcher behavior on supported packages. If unsupported, evaluate a
  version-pinned compositor extension rather than fake input.

## ADR 0004 — Keep initial implementation and runtime unprivileged

- **Status:** Accepted
- **Date:** 2026-10-23
- **Decision:** Run the shell, agent, plugin host, session manager, and CLI as
  the logged-in user. Any privileged operation must be isolated in a narrowly
  scoped helper and require explicit authorization.
- **Reason:** Limits impact from plugins and agent actions and follows the
  project brief's security requirements.

## ADR 0005 — Make Hyprland the primary compositor

- **Status:** Provisional
- **Date:** 2026-10-23
- **Decision:** Replace the Sway-first implementation with Hyprland as the
  project's sole initial compositor. Use Waybar for the prototype status bar.
- **Reason:** The supplied updated product brief selects Hyprland throughout.
- **Consequence:** The new backend and bar have not been tested on FreeBSD;
  pointer centering and full shell behavior remain unverified or incomplete.
