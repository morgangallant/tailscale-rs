[![Crate](https://img.shields.io/crates/v/tailscale.svg)](https://crates.io/crates/tailscale)
[![API](https://docs.rs/tailscale/badge.svg)](https://docs.rs/tailscale)

__tailscale-rs__ is an unofficial client library for [Tailscale](https://tailscale.com). Its purpose is to provide application-level primitives for accessing the Tailscale interface of a machine, as well as utilities for building distributed systems such as automatic peer discovery. Feature requests are welcome and encouraged!

All versions v0.X.X are unofficial, and if Tailscale, Inc. chooses to create an official client library, the ownership of this crate will be transferred to them to release and maintain V1+. At this time, breaking API changes may or may not be introduced.

### Changelog

All changes to this crate over time will be documented here.

#### V0.1.1

- Fixed documentation typos.

#### V0.1.0

- Added the ability to query for a machine's Tailscale interface IPv4 address.
- Published to crates.io.

