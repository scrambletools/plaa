//! plaa's helper for raw Ethernet, the one file granted CAP_NET_RAW
//! (`sudo setcap cap_net_raw+ep plaa-netraw`): it opens packet sockets
//! for plaa and passes them over, so plaa itself holds no capability and
//! the desktop's portals still answer it.

#[cfg(target_os = "linux")]
fn main() {
    std::process::exit(avb_net::broker::serve());
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("plaa-netraw is for Linux, which grants raw sockets by capability");
    std::process::exit(2);
}
