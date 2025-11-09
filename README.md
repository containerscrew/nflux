<p align="center">
    <h1 align="center">The nflux project</h1>
    <p align="center">Simple network monitoring agent tool. Powered by eBPF 🐝</p>
    <p align="center">Kernel and user space code written entirely in Rust ❤</p>
</p>

---

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
[![License - GPL3](https://img.shields.io/github/license/containerscrew/nflux)](/LICENSE-GPL3)
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
    <img src="./examples/grafana-map.png" alt="example3"/>
</p>

---

# What is nflux?

Nflux is an [`eBPF`](./docs/what_is_ebpf.md)-based agent tool that monitors `incoming/outgoing` traffic on a Linux system by
attaching a `XDP (Express Data Path)` program using eBPF technology. (... More coming soon!)

> [!WARNING]
I am working on a new agent-mode implementation of `nflux` that can be deployed as a `systemd` service or a Docker container. The goal is to extract network statistics (using `eBPF` and `XDP`) from any Linux server and display them on a centralized `Grafana` dashboard. Version [`0.12.7`](https://github.com/containerscrew/nflux/tree/v0.12.4) is the latest released version that uses `nflux` as a `CLI` using `TC` (Traffic Control).

# Documentation

I'm trying to move all the documentation to this new site: https://containerscrew.github.io/nflux/

# License

**`nflux`** is distributed under the terms of the [GPL3](./LICENSE-GPL3) license.
