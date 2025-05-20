<p align="center">
    <h1 align="center">The nflux project</h1>
    <p align="center">Network monitoring tool. Powered by eBPF 🐝</p>
    <p align="center">Kernel and user space code written entirely in Rust ❤</p>
</p>

---
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![License - MIT](https://img.shields.io/github/license/containerscrew/nflux)](/LICENSE)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)
![Code Size](https://img.shields.io/github/languages/code-size/containerscrew/nflux)
[![Test Pipeline](https://github.com/containerscrew/nflux/actions/workflows/test.yml/badge.svg)](https://github.com/containerscrew/nflux/actions/workflows/test.yml)
[![Build Pipeline](https://github.com/containerscrew/nflux/actions/workflows/build.yml/badge.svg)](https://github.com/containerscrew/nflux/actions/workflows/build.yml)
[![Release Pipeline](https://github.com/containerscrew/nflux/actions/workflows/release.yml/badge.svg)](https://github.com/containerscrew/nflux/actions/workflows/release.yml)
[![Release](https://img.shields.io/github/release/containerscrew/nflux)](https://github.com/containerscrew/nflux/releases/latest)
[![GitHub Releases Stats](https://img.shields.io/github/downloads/containerscrew/nflux/total.svg?logo=github)](https://somsubhra.github.io/github-release-stats/?username=containerscrew&repository=nflux)
---
<p align="center">
    <h3 align="center">$ nflux</h3>
    <img src="./examples/example.png" alt="example"/>
</p>

---

# What is nflux?

Nflux is an [`eBPF`](./docs/what_is_ebpf.md)-based tool that monitors `incoming/outgoing` traffic on a Linux system by attaching a `TC (Traffic Control)` program using eBPF technology. It can be attached to both physical and virtual interfaces (wireguard), allowing us to obtain networking data at a very low level.

> Supports only ethernet interfaces
> _Supports only Ipv4 packet sniffing (Ipv6 is being implemented)_.

# Installation

Read the [installation](./docs/installation.md) doc.

# Usage

Read the [usage](./docs/usage.md) doc.

# License

**`nflux`** is distributed under the terms of the [GPL3](./LICENSE) license.
