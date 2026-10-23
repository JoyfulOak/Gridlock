# FreeBSD login and session registration

The Wayland session entry is `wayland-sessions/gridlock.desktop`. A FreeBSD
package should install it as
`/usr/local/share/wayland-sessions/gridlock.desktop` alongside
`gridlock-session`, `gridlock`, and `gridlock-shell` in `/usr/local/bin`.

For a source checkout, an administrator can register the session manually after
the binaries are installed. The desktop entry uses `Exec=gridlock-session`, so
install `gridlock-session` in a directory on the login manager's `PATH`, such
as `/usr/local/bin`. SDDM's default FreeBSD session path includes that
directory; user-local binary paths may not be available in its environment:

```sh
install -d /usr/local/share/wayland-sessions
install -m 644 packaging/freebsd/wayland-sessions/gridlock.desktop \
  /usr/local/share/wayland-sessions/gridlock.desktop
```

The display manager's session chooser can then list **Gridlock (Wayland)**.
After the user authenticates and selects that session, the desktop entry starts
`gridlock-session`, which launches Sway using
`$XDG_CONFIG_HOME/gridlock/sway.conf` or `~/.config/gridlock/sway.conf`.
Gridlock does not implement password verification; authentication remains the
responsibility of the FreeBSD login manager.

## Optional SDDM login screen at boot

SDDM supplies the graphical login/credential screen. Gridlock contributes a
Wayland session entry; it does not implement credential authentication. After
registering the entry above, install and enable SDDM as root:

```sh
pkg install sddm
sysrc sddm_enable="YES"
```

At the next boot, sign in through SDDM and choose **Gridlock (Wayland)** from
the session chooser. SDDM then starts the Gridlock session as the authenticated
user. This procedure does not enable auto-login. SDDM remembers the user's last
selected session by default, and users can select another session in its
chooser. To disable SDDM at boot from a TTY, run as root:

```sh
sysrc sddm_enable="NO"
service sddm stop
```

The entry and SDDM startup flow have not been tested on a FreeBSD target.
Keep a working TTY login until target testing succeeds.
