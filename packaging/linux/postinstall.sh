#!/bin/sh
# Lets plaa send and receive raw Ethernet for AVB without root, through
# plaa-netraw, the one program granted CAP_NET_RAW, which opens its raw
# sockets. Then says when no interface can send plaa's AVB stream.
program=/usr/lib/plaa/plaa-netraw
if ! setcap cap_net_raw+ep "$program"; then
    echo "plaa: could not grant $program CAP_NET_RAW; run: sudo setcap cap_net_raw+ep $program" >&2
fi
/usr/lib/plaa/avb-check || true
exit 0
