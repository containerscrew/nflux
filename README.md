<p align="center">
    <img src="docs/source/_static/logo.png" alt="logo" width="250"/>
    <h1 align="center">The nflux project</h1>
    <p align="center">Simple network monitoring agent tool. Powered by eBPF 🐝</p>
    <p align="center">Kernel and user space code written entirely in Rust ❤</p>
</p>

---

<p align="center" >
    <img alt="pre-commit" src="https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white">
    <img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/containerscrew/nflux">
    <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/containerscrew/nflux">
    <img alt="GitHub issues" src="https://img.shields.io/github/issues/containerscrew/nflux">
    <img alt="GitHub pull requests" src="https://img.shields.io/github/issues-pr/containerscrew/nflux">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/containerscrew/nflux?style=social">
    <img alt="GitHub watchers" src="https://img.shields.io/github/watchers/containerscrew/nflux?style=social">
    <img alt="Release" src="https://img.shields.io/github/v/release/containerscrew/nflux?style=flat&logo=github">
    <img alt="Downloads" src="https://img.shields.io/github/downloads/containerscrew/nflux/total?style=flat&logo=github">
    <img alt="License" src="https://img.shields.io/badge/License-GPLv3-blue.svg">
</p>

---

Nflux is a network agent tool that monitors `incoming` traffic on a Linux system by
attaching a `XDP (Express Data Path)` program using eBPF technology.

The nflux program attaches directly to the physical network interface, enabling packet processing at a very low level, right at the network driver. It supports both `IPv4` and `IPv6` packet processing, as well as core protocols such as `TCP`, `UDP`, and `ICMP`.

> XDP can be used to redirect packets or drop them at the network interface. However, this software is not designed to act as a firewall or to redirect packets at this time. Its sole purpose is to monitor incoming traffic, allowing all packets to pass through without interference. `Multiple attachment` to different interfaces is not yet supported.

> [!NOTE]
> Egress traffic monitoring was available in earlier versions of nflux (prior to `1.0.0`). For versions `1.0.0` and above, it is currently not included, as the main focus of this software is tracking incoming connections of servers which typically handle inbound traffic (e.g, web servers, etc).

---

<h2 align="center">Example of nflux data visualized in OpenSearch Dashboards</h2>

<p align="center">
    <img src="./docs/source/_static/opensearch-dashboard.png" alt="example2"/>
</p>

---

# Documentation

https://nflux.containerscrew.com

# License

**`nflux`** is distributed under the terms of the [GPL3](./LICENSE-GPL3) license.
