<p align="center">
    <h1 align="center">The nflux project</h1>
    <p align="center">eBPF network monitoring tool 🐝</p>
    <p align="center">Kernel and user space code written entirely in Rust ❤</p>
</p>

---
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![License - MIT](https://img.shields.io/github/license/containerscrew/nflux)](/LICENSE)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)
![Code Size](https://img.shields.io/github/languages/code-size/containerscrew/nflux)
[![Test Pipeline](https://github.com/containerscrew/nflux/actions/workflows/test.yml/badge.svg)](https://github.com/containerscrew/nflux/actions/workflows/test.yml)
[![Build Pipeline](https://github.com/containerscrew/nflux/actions/workflows/build.yml/badge.svg)](https://github.com/containerscrew/nflux/actions/workflows/build.yml)
[![Lint Pipeline](https://github.com/containerscrew/nflux/actions/workflows/lint.yml/badge.svg)](https://github.com/containerscrew/nflux/actions/workflows/lint.yml)
[![Release Pipeline](https://github.com/containerscrew/nflux/actions/workflows/release.yml/badge.svg?event=push)](https://github.com/containerscrew/nflux/actions/workflows/release.yml)
[![Release](https://img.shields.io/github/release/containerscrew/nflux)](https://github.com/containerscrew/nflux/releases/latest)
[![GitHub Releases Stats](https://img.shields.io/github/downloads/containerscrew/nflux/total.svg?logo=github)](https://somsubhra.github.io/github-release-stats/?username=containerscrew&repository=nflux)
---
<p align="center">
    <img src="./examples/example.png" alt="example"/>
</p>

---
<p align="center">
    <img src="./examples/example2.png" alt="example2"/>
</p>

---

# What is nflux?

Nflux is an [`eBPF`](./docs/what_is_ebpf.md)-based tool that monitors `incoming/outgoing` traffic on a Linux system by
attaching a `TC (Traffic Control)` program using eBPF technology. It can be attached to both physical and virtual
interfaces, allowing us to obtain networking data at a very low level. You can also monitor dropped packets using the
tracepoint
`tracepoint/skb/kfree_skb`.

# Installation

Quick installation:

```shell
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/containerscrew/nflux/main/scripts/install.sh | sh
```

Read the [installation](https://github.com/containerscrew/nflux/wiki/Installation) doc.

# Usage

```shell
sudo nflux --help
sudo nflux traffic-control
sudo nflux dropped-pkt
```

Read the [usage](https://github.com/containerscrew/nflux/wiki/Usage) doc for more options.

# Wiki

Read the [wiki](https://github.com/containerscrew/nflux/wiki)

# License

**`nflux`** is distributed under the terms of the [GPL3](./LICENSE) license.
