#!/bin/sh
# Lets plaa send and receive raw Ethernet for AVB without root, through
# plaa-netraw, the one program granted CAP_NET_RAW, which opens its raw
# sockets. Then says when no interface can send plaa's AVB stream, and
# stops the ptp4l services an older plaa ran.
program=/usr/lib/plaa/plaa-netraw
if ! setcap cap_net_raw+ep "$program"; then
    echo "plaa: could not grant $program CAP_NET_RAW; run: sudo setcap cap_net_raw+ep $program" >&2
fi
/usr/lib/plaa/avb-check || true
# plaa's ptp4l services gave way to gptpd: any an older plaa started would
# keep gptpd off its interface.
if [ -d /run/systemd/system ]; then
    systemctl stop 'plaa-ptp4l@*.service' 'plaa-ptp4l-lite@*.service' 2>/dev/null || true
fi
exit 0
