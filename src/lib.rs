//! tailscale-rs is a client library for [Tailscale](https://tailscale.com).
//!
//! All versions v0.X.X are unofficial, and if Tailscale, Inc. chooses
//! to create an official client library, the ownership of this crate
//! will be transferred to them to release and maintain V1+. At this
//! time, breaking API changes may or may not be introduced.
extern crate ipnetwork;
extern crate pnet;

use ipnetwork::IpNetwork;
use std::net::IpAddr;

fn maybe_tailscale(s: &str) -> bool {
    s.starts_with("wg")
        || s.starts_with("ts")
        || s.starts_with("tailscale")
        || s.starts_with("utun")
}

/// Retrieve the IP address of the current machine's Tailscale interface, if any.
/// ```
/// let iface = tailscale::interface().expect("no tailscale interface found");
/// ```
pub fn interface() -> Option<IpAddr> {
    let ifaces = pnet_datalink::interfaces();
    let netmask: IpNetwork = "100.64.0.0/10".parse().unwrap();
    ifaces
        .iter()
        .filter(|iface| maybe_tailscale(&iface.name))
        .flat_map(|iface| iface.ips.clone())
        .filter(|ipnet| ipnet.is_ipv4() && netmask.contains(ipnet.network()))
        .map(|ipnet| ipnet.ip())
        .next()
}
