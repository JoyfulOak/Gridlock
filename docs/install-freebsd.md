# Install the current Gridlock prototype on FreeBSD

Gridlock does not have a FreeBSD package yet. This guide builds the current
source tree and starts it manually from a TTY. The current shell is a Swaybar
status provider; the GTK shell, launcher, settings UI, notifications, AI
diagnostics, crash monitor, and system integrations are not implemented.

## 1. Install build and Wayland prerequisites

As root, install the package manager dependencies:

```sh
pkg install git rust wayland seatd sway xterm
```

The locked workspace currently requires Rust/Cargo 1.85 or newer. Check the
installed compiler before building with `rustc --version` and `cargo --version`.

The FreeBSD Wayland setup also requires graphics to be configured, `evdev`
support in the kernel, and the login user in the `video` group. Add the user
using the actual account name, then log out and back in so the new group applies:

```sh
pw groupmod video -m YOUR_USER
sysrc seatd_enable="YES"
service seatd start
```

The exact graphics driver depends on the FreeBSD release and hardware. Resolve
that before starting Sway; this project does not change kernel modules, drivers,
or system configuration automatically.

## 2. Build from the repository

Use the current Gridlock source checkout and build as your regular user. Cargo
will fetch Rust crate dependencies from the configured registry. If these
working-tree changes have not yet been published, copy this repository folder
to the VM instead of cloning the remote branch.

```sh
cargo build --workspace --locked --release
cargo test --workspace --locked
```

The workspace has passed tests on macOS and `cargo check` for the FreeBSD amd64
and aarch64 targets, but it has not been built or run on FreeBSD. If either
Cargo command fails on the VM, keep the diagnostic output; do not proceed to
login-manager setup until the build and tests pass.

## 3. Install binaries and user configuration

```sh
mkdir -p "$HOME/.local/bin" "$HOME/.config/gridlock"
install -m 755 target/release/gridlock "$HOME/.local/bin/gridlock"
install -m 755 target/release/gridlock-session "$HOME/.local/bin/gridlock-session"
install -m 755 target/release/gridlock-shell "$HOME/.local/bin/gridlock-shell"
install -m 755 target/release/gridlock-plugin-host "$HOME/.local/bin/gridlock-plugin-host"
cp config/defaults/gridlock.toml "$HOME/.config/gridlock/config.toml"
cp config/defaults/sway.conf "$HOME/.config/gridlock/sway.conf"
```

Add `~/.local/bin` to `PATH` in your shell startup file. In the current shell:

```sh
export PATH="$HOME/.local/bin:$PATH"
```

On FreeBSD 14.1 and newer, `XDG_RUNTIME_DIR` is set automatically. On older
releases, set it to `/var/run/user/$(id -u)` for the session.

Before starting Sway, check that the configuration parses:

```sh
sway -C -c "$HOME/.config/gridlock/sway.conf"
```

## 4. Start Gridlock manually

From a local TTY as your regular user:

```sh
gridlock-session "$HOME/.config/gridlock/sway.conf"
```

This runs Sway in the foreground. A login manager session entry is provided at
`packaging/freebsd/wayland-sessions/gridlock.desktop`; see
`packaging/freebsd/README.md` to install it into the system session directory.
The entry lets a compatible login manager launch Gridlock after user
authentication. To show a graphical login screen at boot, follow the optional
SDDM setup below.

The current configuration provides
Super+Arrow directional focus, Super+Shift+Arrow window movement, workspace
shortcuts, close/fullscreen/floating actions, Super+Enter for the configured
`xterm`, and a top Swaybar with workspaces, focused title, and time.

To return to the TTY, press Super+Shift+E or switch to another virtual terminal.

## 5. Check the prototype

Inside Sway, open a terminal and run:

```sh
gridlock status
gridlock config validate
gridlock plugin list
```

Directional focus currently uses Sway's `seat seat0 cursor set` command to
center the pointer. Upstream marks that command deprecated in favor of a
virtual-pointer protocol, and the FreeBSD packaged Sway behavior has not been
tested. If focus works but pointer movement does not, set
`pointer_follow_focus = false` in `~/.config/gridlock/config.toml` while testing
the Sway command on your release. That disables pointer following and does not
meet the final acceptance criterion.

## 6. Optional: show a login screen at boot with SDDM

SDDM provides the graphical username/password screen; Gridlock is the session
you select after signing in. This does not add a custom Gridlock greeter or
enable passwordless auto-login. First install the release binaries into
`/usr/local/bin` and register the session entry. From the repository directory,
run these as root:

```sh
install -m 755 target/release/gridlock /usr/local/bin/gridlock
install -m 755 target/release/gridlock-session /usr/local/bin/gridlock-session
install -m 755 target/release/gridlock-shell /usr/local/bin/gridlock-shell
install -m 755 target/release/gridlock-plugin-host /usr/local/bin/gridlock-plugin-host
install -d /usr/local/share/wayland-sessions
install -m 644 packaging/freebsd/wayland-sessions/gridlock.desktop \
  /usr/local/share/wayland-sessions/gridlock.desktop
```

Then install and enable SDDM as root:

```sh
pkg install sddm
sysrc sddm_enable="YES"
```

Reboot to show the SDDM login screen. Choose **Gridlock (Wayland)** in its
session chooser, then sign in. SDDM starts `gridlock-session` for the
authenticated user, which starts Sway using that user's Gridlock Sway config.
The SDDM Wayland session search path includes
`/usr/local/share/wayland-sessions` on FreeBSD 15.1. SDDM and Gridlock session
registration have not been tested together on a FreeBSD target, so retain a
TTY login path for recovery.

Enabling SDDM makes it the graphical login manager at boot. To undo that
setting from a TTY, run as root:

```sh
sysrc sddm_enable="NO"
service sddm stop
```
