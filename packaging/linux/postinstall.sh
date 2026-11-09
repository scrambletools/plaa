#!/bin/sh
# Lets plaa send and receive raw Ethernet for AVB without root, through
# plaa-netraw, the one program granted CAP_NET_RAW, which opens its raw
# sockets. Says what installing changes on the system and why, then when
# no interface can send plaa's AVB stream, and stops the ptp4l services
# an older plaa ran.
program=/usr/lib/plaa/plaa-netraw
if ! setcap cap_net_raw+ep "$program"; then
    echo "plaa: could not grant $program CAP_NET_RAW; run: sudo setcap cap_net_raw+ep $program" >&2
fi
cat <<'EOF'
plaa: for sending what plays to AVB speakers and amplifiers, this package
  - gives /usr/lib/plaa/plaa-netraw CAP_NET_RAW, so only that small
    helper opens raw network sockets and plaa itself runs without root;
  - relies on RealtimeKit (rtkit), as PipeWire does, to run plaa's AVB
    stream threads at realtime priority, so a busy computer doesn't
    hold up their audio;
  - installs plaa-link@.service and a polkit rule that lets you run it.
    When plaa starts AVB on an interface, it keeps plaa's frames on one
    transmit queue, under fq_codel, and turns Energy-Efficient
    Ethernet and PAUSE off, reconnecting the link once if they were on.
Nothing on an interface changes until plaa starts AVB there.
EOF
/usr/lib/plaa/avb-check || true
# plaa's ptp4l services gave way to gptpd: any an older plaa started would
# keep gptpd off its interface.
if [ -d /run/systemd/system ]; then
    systemctl stop 'plaa-ptp4l@*.service' 'plaa-ptp4l-lite@*.service' 2>/dev/null || true
fi
exit 0
