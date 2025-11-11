<p align="center">
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

<p align="center">
    <img src="./examples/example.png" alt="example"/>
</p>

---

<p align="center">
    <h3 align="center">Build your own network monitoring dashboard with Opensearch</h3>
    <img src="./examples/opensearch-dashboard.png" alt="example3"/>
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
