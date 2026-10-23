# Gridlock — Master Coding Agent Prompt
## Build an Original AI-Native, Plugin-Based FreeBSD Tiling Desktop Inspired by Omarchy

## Mandatory change from previous architecture
This version replaces **Sway with Hyprland** as the primary Wayland compositor. Do not install, configure, or use Sway as Gridlock’s compositor. All implementation plans, examples, tests, packages, and documentation must target Hyprland.

## Your role
You are the principal architect and implementation agent for a new open-source, native FreeBSD desktop environment. Deliver real, tested source code, not mockups or an endless plan. Work in small, verifiable milestones. The product is its own desktop shell, not a rebranded KDE, GNOME, XFCE, or port of Omarchy. Use Omarchy as a reference for interaction patterns, not as code to copy without license review.

## Project identity and working rules
- Official project and product name: **Gridlock**. Use this name consistently in UI, documentation, packages, session entries, commands, examples, and source structure. Do not use `freebsd-desktop` or generic `desktop-*` placeholders as product identifiers.
- Before changing an existing repository, inspect it, summarize what exists, identify risks, and request approval for destructive changes. If starting with an empty repository, scaffold it.
- Ask only essential questions that block safe implementation. Prefer sensible documented defaults for nonessential choices.
- Do not silently change the OS, bootloader, login manager, system services, or privileged configuration.
- Support FreeBSD amd64 and aarch64. Detect whether running in UTM/virtualized hardware; never assume GPU acceleration or a particular input backend works.
- Keep a running `PLAN.md`, `DECISIONS.md`, `STATUS.md`, and `KNOWN_LIMITATIONS.md`. Record every significant design choice and actual test result.
- After each milestone: show changed files, build/test commands, results, limitations, and the next small step. Do not claim a feature works unless exercised on the target environment.
- All commands in this document are illustrative interfaces, not evidence that they already exist.

## Gridlock naming and component identity
- Desktop environment: **Gridlock**.
- Unified shell: **Gridlock Shell** (`gridlock-shell`).
- Window and directional focus controller: **Gridlock Window Controller** (part of compositor integration).
- Extensions ecosystem: **Gridlock Extensions**, with a versioned **Gridlock Plugin SDK** and Plugin Manager.
- AI integration: **Gridlock Agent**, with a user-approved **Gridlock System Doctor** crash-diagnosis workflow.
- Settings application: **Gridlock Control** (`gridlock-control`).
- Themes: **Gridlock Themes**.
- Main CLI executable: **`gridlock`**.
- FreeBSD desktop session: **Gridlock (Wayland)**.
- Default configuration directory: `~/.config/gridlock/`; user state: `~/.local/state/gridlock/`; user data: `~/.local/share/gridlock/`, using FreeBSD-appropriate XDG directory overrides when present.
- Prefer `org.gridlock.*` as a provisional application identifier namespace, but validate ownership and naming collisions before public release.
- Brand direction: restrained, dark-first, keyboard-first, clean geometric tiling; slogan optional: **Gridlock — Your desktop. Your rules.** Do not assume the name, domain, or trademark is legally available; perform checks before public distribution.

## Product vision
Build a fast, keyboard-first, aesthetically cohesive FreeBSD desktop environment with a unified shell, automatic animated tiling, configurable layouts, a persistent top bar, searchable menus, configurable shortcuts, native FreeBSD integrations, a versioned plugin SDK, and an AI troubleshooting agent system. The experience should feel like Omarchy while being original and modular. Users must be able to install extensions that change the appearance and behavior of the desktop without patching the core.

## Non-negotiable interaction behavior
1. **Super + Left/Right/Up/Down**: focus the nearest eligible window in that direction. Upon successful focus, move the mouse pointer to the geometric center of the newly focused window. If no eligible window exists, preserve current focus and pointer position. Expose a toggle for pointer-follow-focus and an option to exclude floating windows. Define multi-monitor behavior explicitly, including crossing display boundaries. Avoid triggering unintended hover actions if feasible. Verify Hyprland directional focus (`movefocus`), window geometry and cursor positioning (`hyprctl cursorpos` is a query, not a warp operation) against the installed build. Investigate supported Hyprland dispatchers such as `movecursor` and their coordinate semantics; implement the focus-then-center operation with a validated native dispatcher or versioned compositor plugin. Do not simulate mouse input with unsafe injection or claim success without testing.
2. **Super + Enter**: terminal. **Super + Space**: searchable launcher. **Super + K**: keyboard shortcut viewer. **Super + Q**: close focused window. **Super + F**: fullscreen. **Super + Shift + Space**: toggle floating. **Super + 1–9**: switch workspace. **Super + Shift + 1–9**: move focused window to workspace. Allow rebinding every shortcut, resolving conflicts, and displaying conflicts in the GUI.
3. Top bar must show workspaces, focused window title, time, network, audio, battery when present, and power menu. Widgets and layout are extensible by plugins.
4. The shell must provide launcher, quick settings, notification center, system/power menu, shortcut overlay, settings app, wallpaper/theme support, and consistent focus/keyboard accessibility.
5. The desktop must remain functional with all optional AI providers offline or disabled.

## Hyprland-specific engineering constraints
- Treat Hyprland as the compositor and Gridlock as the original desktop shell, not as a renamed Hyprland configuration. Gridlock owns menus, bar, notifications, control center, plugin SDK, and agent services.
- Keep Hyprland configuration generated from Gridlock-owned typed settings; never overwrite user-managed Hyprland files without consent. Support import, validation, diff, atomic write, and rollback.
- Use Hyprland native layouts, animations, borders, gaps, decoration, and dispatchers; expose these through Gridlock Control and theme/plugin APIs. Detect unsupported features rather than guessing config keys.
- Hyprland compositor plugins are ABI/version-sensitive and often native-code privileged. Keep ordinary Gridlock extensions out of the compositor; only use a tightly version-pinned, opt-in native compositor plugin if the pointer-centering feature or another requirement cannot be implemented via supported IPC.
- Verify layer-shell and popup behavior with Hyprland and FreeBSD. Make sure shell crash/restart does not terminate the compositor or block keyboard access.
- For FreeBSD ARM64 under UTM, explicitly test DRM/KMS, graphics drivers, seat management, rendering path and software-rendering fallback. Report if the VM cannot support a stable Hyprland session; do not mark that target supported without evidence.

## Technology baseline
- OS: FreeBSD (amd64 and aarch64). Detect release and supported packages; do not hardcode Linux paths or services.
- Display stack: Wayland. Use Hyprland as the initial compositor, with a compositor abstraction to permit future replacement. Check FreeBSD ports/packages, build dependencies, ABI compatibility and Wayland backend support for each targeted FreeBSD release and architecture. Hyprland on FreeBSD may have platform-specific limitations; never assume upstream Linux instructions apply. Do not silently fall back to Hyprland.
- Core implementation: Rust, split into libraries and processes with documented interfaces.
- UI: GTK4 where practical. Confirm FreeBSD package availability and compatibility of any layer-shell/Wayland protocol bindings before committing. If GTK4 layer-shell integration is unavailable, choose a supported alternative and record why. Do not assume Linux-only GTK layer-shell libraries work on FreeBSD.
- Configuration: TOML with typed schema, migration/versioning, atomic writes, backup, validation, and live reload.
- IPC: authenticated/permission-aware local Unix-domain sockets with explicit protocol versioning; consider JSON-RPC for ease of tooling. Do not expose privileged methods on an unprotected socket.
- Persistence: lightweight local database only where needed (for plugin registry, crash history, agent sessions); keep human-editable settings in TOML.
- Build: Cargo workspace, reproducible lockfile, FreeBSD CI or equivalent test runner, `cargo fmt`, `clippy`, unit and integration tests.
- Prefer permissively licensed dependencies with maintained FreeBSD support. Record dependency licenses.

## Architecture and processes
Implement independently restartable components:
- `desktop-session`: session startup, environment, readiness, lifecycle, shutdown, compositor launch and supervision. No display manager dependency is required for initial MVP; optionally integrate one later.
- `desktop-core`: stable internal models, configuration schema, event bus, error handling, plugin capability definitions, logging, IPC protocol.
- `desktop-compositor-hyprland`: adapter for Hyprland IPC (command and event sockets), `hyprctl -j` window/workspace/monitor data, dispatchers, focus events, layout management and validated pointer-centering. Avoid baking Hyprland-specific types into public plugin APIs. Handle socket discovery using Hyprland instance signatures and FreeBSD runtime directory conventions; never assume `/run/user` exists.
- `desktop-shell`: one coordinated visual shell owning the top bar, launcher, popup menus, notification center, and overlays. Separate components internally, but avoid unrelated styling and configuration systems.
- `desktop-settings`: graphical settings app for appearance, shortcuts, displays, input, audio, network, plugins, AI, and troubleshooting. Only show controls that actually work; unavailable integrations should be labeled.
- `desktop-plugin-host`: manifest validation, dependency resolution, install/upgrade/disable/remove, capability enforcement, sandboxed or isolated plugin execution, crash supervision, and rollback.
- `desktop-agent-service`: opt-in AI agent adapters, diagnostic context assembly, proposal/review/apply workflow, operation audit log, and repair verification.
- `desktop-crash-monitor`: monitor selected user processes and desktop components; collect supported FreeBSD crash diagnostics, core metadata, and logs without assuming Linux journalctl/systemd/coredumpctl.
- `gridlock`: documented, scriptable user-facing commands using the same API as the GUI and agent tools.

Document which processes run as the user and which, if any, require a narrowly scoped privileged helper. Default to unprivileged. Never run the shell or AI agent as root.

## Shell visual and interaction design
- Minimal Omarchy-inspired visual language: compact top bar, rounded or square selectable corners, configurable gaps/borders, clear active-window indication, dark/light themes, typography scale, icons, wallpaper, and optional subtle transitions.
- Build original assets and UI. Do not redistribute copyrighted Omarchy assets without checking licenses.
- Support tiling, tabbed/stacked layouts when exposed by compositor, floating windows, scratchpad, fullscreen, multiple monitors, per-monitor workspaces, and hotplug handling.
- Main launcher: application search from installed `.desktop` entries, fuzzy ranking, keyboard navigation, recent items (opt-in), actions, and plugin-defined providers. Do not execute arbitrary text as shell commands unless user deliberately switches to command mode.
- Unified menu categories: Applications, Files, Commands, System, Settings, Plugins, AI. File indexing must be opt-in and respect permissions.
- Shortcut viewer: searchable, grouped, shows current bindings and plugin source; editable bindings with validation.
- Top bar: customizable left/center/right regions and reorderable widgets; per-monitor configuration; proper behavior on display hotplug.
- Notifications: actionable, queued, searchable history with retention controls, do-not-disturb, keyboard accessibility. Support common desktop notification conventions where feasible.
- Appearance settings should apply without restarting the whole graphical session when technically possible. Maintain a last-known-good theme/config and revert on validation failure.
- Keyboard and pointer accessibility: focus outlines, high contrast, scalable UI, reduced motion, screen-reader considerations, and keyboard-only configuration.

## Plugin system: first-class requirement
Implement plugin support early, before building every shell widget. Define a public SDK and compatibility contract:
- `plugin.toml` manifest with ID, name, version, API range, entrypoint/runtime, supported CPU architectures, required desktop version, dependencies, permissions, configuration schema, and optional settings page.
- Plugin types: bar widget, launcher provider, menu/action provider, theme, shortcut action, notification integration, settings panel, AI tool/skill, system integration, and compositor-aware window behavior where authorized.
- Plugins must be able to contribute functionality through stable extension points, not arbitrary in-process mutation of shell internals.
- For executable plugins, prefer isolated subprocesses with message passing, health checks, restart limits, timeouts, and per-capability access. Evaluate WASM/WASI for portable untrusted logic only if runtime support is practical on FreeBSD; do not assume WASM alone is a complete security sandbox.
- Separate decorative themes from executable plugins. Themes are data-only and should not run arbitrary scripts.
- User must approve new or expanded permissions. No hidden privilege escalation. Clearly distinguish read-only, write, execution, filesystem, network, and system administration permissions.
- Support local installation first; signed repository/index and community marketplace later. Validate hashes/signatures and avoid arbitrary install-time scripts by default.
- Provide plugin lifecycle: discover, inspect, install, enable, disable, configure, update, roll back, remove, and inspect logs.
- Support hot enable/disable where safe; fall back to restarting only the relevant shell component if needed.
- Ship at least three example plugins: clock/bar widget, custom launcher action, and alternative theme. Include a developer SDK, templates, tests, and docs.
- A plugin crash must not take down the compositor or entire session. Quarantine repeatedly failing plugins and offer safe mode.

## AI-native desktop and crash repair
Build a provider-neutral AI system. Supported integration classes:
- Local inference endpoints: Ollama, LM Studio, and configurable OpenAI-compatible APIs reachable on localhost or LAN.
- CLI agents: Codex, Claude Code, OpenCode, and other agents through adapters when their binaries and supported FreeBSD architecture are available. Detect capabilities at runtime; never assume Linux binaries work on FreeBSD.
- Provider adapters must declare capabilities (chat, tool calling, structured responses, local/remote, authentication mode). Never scrape another application's GUI as the primary integration.
- Agent manager GUI: configure/select providers, model, permissions, context sharing, diagnostics retention, and whether crash prompts are enabled. Never store plaintext secrets in general config files; use an appropriate secret store or protected mechanism.
- Agent can access a **typed, constrained desktop tool API**: inspect status, read selected logs, inspect compositor state, validate config, list plugins, query package information, generate patch proposals, run tests, and request approved changes.
- Prefer explicit tool contracts over unrestricted shell access. Any command execution must be permission-gated, logged, bounded by timeouts, and shown to the user before risky execution.

### Crash workflow
1. Monitor session/shell/plugin failures and optionally opted-in user applications. Use FreeBSD-native signals, process exit codes, core dump facilities, logs, and service status where available.
2. Deduplicate repeated crashes and rate-limit prompts to avoid notification storms.
3. Show: “This application crashed. Would you like an AI agent to investigate?” with choices: Diagnose, View Details, Dismiss, and Disable Prompts for This App.
4. Before sending diagnostics to a remote model, preview the data and redact likely tokens, passwords, paths or identifiers as appropriate. Make remote transfer opt-in; local models should also obey permissions.
5. Agent returns diagnosis with confidence/uncertainty, evidence, proposed changes, risks, and verification steps. Do not present speculation as proven root cause.
6. For any write, package change, privileged operation, network action, or destructive step: show a human-readable plan and require explicit approval. Offer dry-run where feasible.
7. Snapshot relevant configuration, apply only approved actions, verify outcome, and offer rollback of reversible changes. Never promise rollback for irreversible package or data operations.
8. Preserve an auditable history of actions, results, and user approvals.
9. If the agent itself crashes, the shell and compositor must continue working.

### Additional AI capabilities
- Natural-language desktop customization: “move the clock to the left,” “make gaps smaller,” “add a weather widget,” or “change Super+K to open the shortcut viewer.” Convert requests to validated API actions or plugin code proposals, never direct unchecked edits.
- Plugin generation workflow: scaffold from SDK, write tests, build in an isolated workspace, present permission manifest and diff, request approval, then install. Never auto-install unreviewed generated code.
- Context-aware help: explain keyboard shortcuts, identify why a plugin failed, inspect system health, and suggest performance improvements.

## FreeBSD integration requirements
Research and implement against the actual target FreeBSD release and hardware:
- Session startup and compositor environment; graphics/DRM/KMS and seat/input access appropriate to FreeBSD, including `seatd` or other supported session management when required.
- Wi-Fi/network status and management through available FreeBSD tools and services; do not assume NetworkManager.
- Audio through the configured FreeBSD-supported audio stack (for example, PipeWire where available, or native alternatives), with a backend abstraction.
- Bluetooth through FreeBSD-supported facilities, with clear capability detection and graceful fallback; do not assume BlueZ.
- Display configuration through compositor output APIs, with resolution, scaling, rotation, and multi-monitor profiles.
- Battery and power through FreeBSD-supported APIs/tools; omit battery indicators on desktops.
- Packages and updates through `pkg`, with read-only update checks by default and privileged install/update approval.
- System monitoring through FreeBSD-compatible APIs and tools, not Linux `/proc` assumptions.
- Screen lock, idle management, suspend/restart/shutdown only via supported FreeBSD mechanisms and with appropriate permissions.
- Virtual machines: detect software rendering, missing acceleration, unsupported Wayland extensions, and absent GPU devices. Provide a text-mode recovery path and diagnostic command.

## Security and reliability
- Threat-model plugins, agent tools, crash logs, downloaded updates, local IPC, and privileged helpers.
- Keep privileged helpers minimal, narrowly authorized, and separate from shell UI. Prefer established FreeBSD privilege mechanisms where appropriate.
- Never silently send logs, files, process command lines, or credentials to third-party AI services.
- Use schema validation, atomic writes, backups, safe mode, crash-loop detection, and last-known-good settings.
- Prevent one broken plugin/theme from making the desktop unusable. Provide `gridlock safe-mode`, `gridlock config validate`, and a documented TTY recovery procedure.
- Escape and validate all user/plugin inputs. Do not interpolate arbitrary plugin strings into shell commands.
- Preserve user data and allow clean uninstall without deleting home-directory data unless separately confirmed.

## CLI contract (proposed)
Implement and document a coherent interface, for example:
```
gridlock status
gridlock window focus right --center-pointer
gridlock window move left
gridlock workspace switch 2
gridlock launcher open
gridlock shortcuts show
gridlock theme list
gridlock theme set <name>
gridlock plugin list
gridlock plugin install <local-path>
gridlock plugin disable <id>
gridlock agent providers
gridlock agent diagnose --latest-crash
gridlock config validate
gridlock config reload
gridlock system health
gridlock safe-mode
```
Commands must return meaningful exit codes and structured JSON output where appropriate. Never print secrets.

## Suggested repository layout
```
gridlock/
  Cargo.toml
  crates/
    gridlock-core/
    gridlock-session/
    gridlock-compositor-api/
    gridlock-compositor-hyprland/
    gridlock-shell/
    gridlock-control/
    gridlock-plugin-sdk/
    gridlock-plugin-host/
    gridlock-agent/
    gridlock-crash-monitor/
    gridlock-cli/
    gridlock-freebsd-platform/
  plugins/examples/
  themes/
  config/defaults/
  packaging/freebsd/
  scripts/
  tests/
  docs/
  PLAN.md
  DECISIONS.md
  STATUS.md
  KNOWN_LIMITATIONS.md
```
You may adjust this structure when justified, but keep stable boundaries between compositor, shell, plugins, AI, and platform services.

## Milestones and acceptance criteria
### Phase 0 — Environment audit and feasibility spike
- Identify FreeBSD version, CPU architecture, virtualization platform, graphics devices, display/session setup, installed Hyprland/Wayland/GTK/Rust packages, and relevant supported protocol versions.
- Prove that a minimal Wayland/Hyprland session can start and accept keyboard/mouse input on the actual machine. Record fallback if graphics support blocks it.
- Verify whether reliable pointer centering can be implemented with the chosen compositor and document the exact supported mechanism. This is a release blocker for the requested window-navigation feature.
- Output a compatibility matrix for amd64, aarch64, and UTM; distinguish tested from planned.

### Phase 1 — Buildable desktop foundation
- Create Cargo workspace, shared configuration, IPC, session lifecycle, structured logs, CLI, and tests.
- Start Hyprland and launch a minimal shell process. Ensure crash/restart behavior and TTY recovery.
- Acceptance: repeatable build and documented launch on target FreeBSD.

### Phase 2 — Tiling and keyboard navigation
- Implement all baseline shortcuts, directional window selection, focus changes, pointer centering, floating/fullscreen, workspaces, and multiple-output handling.
- Test window geometries and edge cases: 2x2 tiling, uneven splits, floating windows, overlapping windows, multiple monitors, disappearing target, empty workspace, and focus event races.
- Acceptance: Super+arrow selects correct window and pointer lands within a small tolerance of the focused window's center; no move if focus fails.

### Phase 3 — Unified graphical shell
- Implement top bar, launcher, system menu, shortcut viewer, notifications, themes, settings foundation, and keyboard accessibility.
- Acceptance: daily desktop tasks possible without an existing full desktop environment.

### Phase 4 — Plugin runtime and SDK
- Implement manifests, permission prompts, isolation, install/disable/update/rollback, sample plugins, settings contributions, and plugin logs.
- Acceptance: enable a bar widget and launcher action without rebuilding core; intentionally crash a plugin and verify session survives.

### Phase 5 — AI provider manager and diagnostics
- Implement provider adapters, tool registry, crash detection, opt-in diagnosis, redaction, proposals, approval, action audit, verification, and rollback where feasible.
- Acceptance: simulate a crash, show notification, diagnose with a configured provider, display an actionable repair proposal, and apply only after approval. Verify offline mode works.

### Phase 6 — FreeBSD system integrations
- Implement and test networking, audio, Bluetooth (when available), power, packages, displays, input, updates, and system health.
- Acceptance: graceful capability detection and no Linux-only assumptions.

### Phase 7 — Packaging, docs, and hardening
- Produce FreeBSD package/build instructions, first-run setup, optional session entry, upgrade/migration paths, safe-mode recovery, plugin developer guide, AI privacy documentation, and CI.
- Acceptance: clean install on a supported FreeBSD target, reproducible build, documented recovery, and passing smoke tests.

## Testing and performance
- Unit tests for direction selection, config migration, permission evaluation, manifest parsing, event routing, and redaction.
- Integration tests for Hyprland IPC parsing, hotplug, launcher actions, plugin lifecycle, crash monitor, agent approvals, and rollback.
- End-to-end manual test script for real FreeBSD graphical sessions; explicitly record what cannot be automated in a VM.
- Profile idle CPU, RSS memory, startup latency, menu latency, shortcut response, and plugin overhead. Set measurable baselines after Phase 0; do not invent performance numbers.
- Ensure shell restart does not terminate the compositor, and AI failure does not terminate the shell.

## Implementation process — begin now
1. Inspect the repository and machine. Do not install packages or alter system configuration until you have shown the commands and obtained approval if changes are privileged or potentially disruptive.
2. Produce a concise architecture decision record and Phase 0 feasibility checklist.
3. Check FreeBSD package/protocol support and test the compositor and pointer-centering path first. Flag any unsupported requirement rather than silently substituting a weaker behavior.
4. Scaffold the Cargo workspace and implement the smallest runnable vertical slice: session + compositor adapter + CLI + Super+arrow directional focus with pointer centering.
5. Continue phase by phase, compiling and testing at each step. Give complete file changes, not partial pseudocode. Prefer working implementation over elaborate mockups.
6. At every stopping point, report what is implemented, what was actually tested, exact commands to run, known issues, and the next step.

## Definition of done
This project is complete only when a user can install and log into an original FreeBSD desktop session; navigate tiled windows using Super+arrows with pointer centering; use a custom top bar, launcher, menus, settings, and shortcuts; install and disable third-party plugins; configure an AI provider; receive an optional crash-diagnosis prompt; approve or reject repairs; recover from broken plugins/configuration; and perform core desktop tasks without KDE/GNOME/XFCE. Every claimed supported architecture must be backed by real test evidence.
