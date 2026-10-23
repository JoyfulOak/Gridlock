# Install the current Gridlock prototype on FreeBSD

Gridlock has no FreeBSD package yet. This guide builds the current source and
starts it manually from a TTY. Hyprland is the only supported compositor in
this project. The graphical shell, launcher, settings UI, notifications, AI
diagnostics, crash monitor, and system integrations remain incomplete.

## 1. Install prerequisites

Check package availability for the target FreeBSD release and architecture
before installing. As root, install the packages:

```sh
pkg install git rust wayland seatd hyprland waybar xterm
```

The locked workspace requires Rust/Cargo 1.85 or newer. Graphics setup,
`evdev` input support, and seat access depend on the machine. Resolve those
through the FreeBSD documentation for the target before starting Hyprland.
Gridlock does not change kernel modules, drivers, services, or system
configuration automatically.

## 2. Build from the repository

Build as the regular user. If the working-tree changes have not been
published, copy this repository folder to the VM instead of cloning the
remote branch.

```sh
cargo build --workspace --locked --release
```

The earlier Sway-based workspace was built on macOS and cross-checked for
FreeBSD targets. The Hyprland adapter has not yet been built on a FreeBSD host
or exercised in a graphical session.

## 3. Install binaries and user configuration

```sh
mkdir -p "$HOME/.local/bin" "$HOME/.config/gridlock"
install -m 755 target/release/gridlock "$HOME/.local/bin/gridlock"
install -m 755 target/release/gridlock-session "$HOME/.local/bin/gridlock-session"
install -m 755 target/release/gridlock-shell "$HOME/.local/bin/gridlock-shell"
install -m 755 target/release/gridlock-plugin-host "$HOME/.local/bin/gridlock-plugin-host"
cp config/defaults/gridlock.toml "$HOME/.config/gridlock/config.toml"
cp config/defaults/hyprland.conf "$HOME/.config/gridlock/hyprland.conf"
cp config/defaults/waybar.jsonc "$HOME/.config/gridlock/waybar.jsonc"
cp config/defaults/waybar.css "$HOME/.config/gridlock/waybar.css"
install -m 755 scripts/gridlock-network-status.sh "$HOME/.local/bin/gridlock-network-status"
```

Add `~/.local/bin` to `PATH`. Hyprland IPC discovery uses
`XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket.sock`; set the
FreeBSD-appropriate runtime directory in your session if the system does not
provide it.

## 4. Start the session

From a local TTY as the regular user:

```sh
gridlock-session
```

Gridlock starts `Hyprland` with `~/.config/gridlock/hyprland.conf`. The
experimental Waybar bar includes workspaces, focused title, clock, route
status, sndio audio, and battery when available. Audio and battery modules
depend on target hardware and Waybar build options. The power menu and native
Gridlock launcher are not implemented yet.

To return to the TTY, use the configured Super+Shift+E binding or switch to
another virtual terminal.

## 5. Check the prototype

Inside the session, open a terminal and run:

```sh
gridlock status
gridlock config validate
gridlock plugin list
```

Directional focus reads `j/clients` from Hyprland IPC, then uses the
`focuswindow` and `movecursor` dispatchers. Pointer-centering and dispatcher
coordinate semantics remain unverified against a FreeBSD Hyprland build. If
centering fails, set `pointer_follow_focus = false` in
`~/.config/gridlock/config.toml`; that disables pointer following and does not
meet the final acceptance criterion.

## 6. Optional: register with SDDM

SDDM provides the graphical login screen; Gridlock is the session selected
after signing in. This does not add a custom greeter or enable auto-login.
Install the release binaries and session entry as root:

```sh
install -m 755 target/release/gridlock /usr/local/bin/gridlock
install -m 755 target/release/gridlock-session /usr/local/bin/gridlock-session
install -m 755 target/release/gridlock-shell /usr/local/bin/gridlock-shell
install -m 755 target/release/gridlock-plugin-host /usr/local/bin/gridlock-plugin-host
install -d /usr/local/share/wayland-sessions
install -m 644 packaging/freebsd/wayland-sessions/gridlock.desktop \
  /usr/local/share/wayland-sessions/gridlock.desktop
pkg install sddm
sysrc sddm_enable="YES"
```

Retain a TTY login path for recovery. Session registration and SDDM have not
been tested on a FreeBSD target. To disable SDDM from a TTY, run as root:

```sh
sysrc sddm_enable="NO"
service sddm stop
```
