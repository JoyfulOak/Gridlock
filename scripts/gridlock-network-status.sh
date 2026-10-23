#!/bin/sh

# Waybar custom module for FreeBSD. Report the interface selected by the
# system routing table; do not assume Linux /sys or NetworkManager paths.
interface=$(route -n get default 2>/dev/null | awk '$1 == "interface:" { print $2; exit }')

if [ -n "$interface" ] && ifconfig "$interface" 2>/dev/null | grep -q 'status: active'; then
    printf '{"text":"%s","alt":"connected","tooltip":"Connected via %s","class":"connected"}\n' "$interface" "$interface"
else
    printf '{"text":"offline","alt":"disconnected","tooltip":"No active default network interface","class":"disconnected"}\n'
fi
