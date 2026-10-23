# FreeBSD login and session registration

Gridlock provides one Wayland session entry at
`wayland-sessions/gridlock.desktop`. It starts `gridlock-session`, which
launches Hyprland using the user's
`$XDG_CONFIG_HOME/gridlock/hyprland.conf` or
`~/.config/gridlock/hyprland.conf`.

For a source checkout, an administrator can register the session manually:

```sh
install -d /usr/local/share/wayland-sessions
install -m 644 packaging/freebsd/wayland-sessions/gridlock.desktop \
  /usr/local/share/wayland-sessions/gridlock.desktop
```

A compatible login manager can then list **Gridlock (Wayland)**. Gridlock does
not implement password verification; authentication remains the login
manager's responsibility. Session registration and Hyprland startup have not
been tested on the supported FreeBSD targets.

## Optional SDDM login screen at boot

SDDM provides a graphical login screen. As root, install and enable it:

```sh
pkg install sddm
sysrc sddm_enable="YES"
```

At the next boot, choose **Gridlock (Wayland)** in the session chooser. To
disable SDDM at boot from a TTY, run as root:

```sh
sysrc sddm_enable="NO"
service sddm stop
```
