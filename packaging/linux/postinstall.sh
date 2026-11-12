#!/bin/sh
# Says what installing changes on the system and why, then when no
# interface can send plaa's AVB stream, and stops the services an older
# plaa ran: its ptp4l services, and plaa-link@, which avb-link@ replaced.
cat <<'EOF'
plaa: for sending what plays to AVB speakers and amplifiers, this package
  - depends on avb-endpointd, which runs this computer's AVB entity in
    your avb-endpointd@INTERFACE user service, for plaa and any other
    app to play into through PipeWire. Its avb-netraw holds CAP_NET_RAW
    and opens raw network sockets for the daemon and for plaa's
    controller, so plaa itself runs without root;
  - starts and enables that service when you turn "Send what plays" on,
    so it then starts at each login. The daemon starts gptpd and
    avb-link@ on the interface, which keeps the stream's frames on one
    transmit queue and turns Energy-Efficient Ethernet and PAUSE off,
    reconnecting the link once if they were on.
Nothing on an interface changes until AVB starts there.
EOF
/usr/lib/plaa/avb-check || true
# plaa's ptp4l services gave way to gptpd: any an older plaa started would
# keep gptpd off its interface. plaa-link@'s unit is gone with the
# upgrade; stopping what is left of it changes nothing on the interface.
if [ -d /run/systemd/system ]; then
    systemctl daemon-reload 2>/dev/null || true
    systemctl stop 'plaa-ptp4l@*.service' 'plaa-ptp4l-lite@*.service' \
        'plaa-link@*.service' 2>/dev/null || true
fi
exit 0
