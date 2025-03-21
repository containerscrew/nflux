<p align="center">
    <h1 align="center">nflux project</h1>
    <p align="center">Network monitoring tool & TLS/SSL sniffer using eBPF. Powered by Aya-rs 🐝</p>
    <p align="center">Kernel and user space code written entirely in Rust ❤</p>
</p>

---
![Crates.io Total Downloads](https://img.shields.io/crates/d/nflux?color=orange)
[![License - MIT](https://img.shields.io/github/license/containerscrew/nflux)](/LICENSE)
[![pre-commit](https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit&logoColor=white)](https://github.com/pre-commit/pre-commit)
![Code Size](https://img.shields.io/github/languages/code-size/containerscrew/nflux)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Crates.io Version](https://img.shields.io/crates/v/nflux)
[![Test](https://github.com/containerscrew/nflux/actions/workflows/test.yml/badge.svg)](https://github.com/containerscrew/nflux/actions/workflows/test.yml)
[![Build](https://github.com/containerscrew/nflux/actions/workflows/build.yml/badge.svg)](https://github.com/containerscrew/nflux/actions/workflows/build.yml)
---

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [What is nflux?](#what-is-nflux)
  - [What is ebpf?](#what-is-ebpf)
  - [Traffic control](#traffic-control)
- [Badges](#badges)
- [Installation](#installation)
- [Usage](#usage)
  - [netrace](#netrace)
    - [Help](#help)
    - [Sniffing (only) egress traffic](#sniffing-only-egress-traffic)
    - [Sniffing (only) ingress traffic](#sniffing-only-ingress-traffic)
    - [Packet logging](#packet-logging)
    - [Available procotols](#available-procotols)
  - [tlstrace](#tlstrace)
- [Compatibility](#compatibility)
- [Docs](#docs)
- [Contribution](#contribution)
- [License](#license)
<!-- END OF TOC -->

```shell
$ nflux --help

    ███╗   ██╗███████╗██╗     ██╗   ██╗██╗  ██╗
    ████╗  ██║██╔════╝██║     ██║   ██║╚██╗██╔╝
    ██╔██╗ ██║█████╗  ██║     ██║   ██║ ╚███╔╝
    ██║╚██╗██║██╔══╝  ██║     ██║   ██║ ██╔██╗
    ██║ ╚████║██║     ███████╗╚██████╔╝██╔╝ ██╗
    ╚═╝  ╚═══╝╚═╝     ╚══════╝ ╚═════╝ ╚═╝  ╚═╝


Network monitoring tool & TLS/SSL sniffer using eBPF. Powered by Aya-rs 🐝

Usage: nflux [OPTIONS] [COMMAND]

Commands:
  netrace  Start network traffic monitoring using TC (Traffic Control)
  help     Print this message or the help of the given subcommand(s)

Options:
  -l, --log-level <LOG_LEVEL>    Log level for logging tracing. Possible values: info, warn, trace, debug, error. [default: info]
      --log-format <LOG_FORMAT>  Log format for logging tracing. Possible values: text, json. [default: text]
  -h, --help                     Print help
  -V, --version                  Print version
```

# What is nflux?

Nflux is... (pending to finish)

## What is ebpf?

Provide basic concepts of `ebpf` (pending to finish)

## Traffic control

Provide some diagrams of TC (pending to finish)

# Badges

---

# Installation

```shell
XXXXXX pending
```

# Usage

> [!WARNING]
> By the moment, `nflux netrace` only supports Ipv4 sniffing
> `nflux tlstrace` is being implemented!

Global flags:

```shell
sudo nflux --help
sudo nflux --log-level warn
sudo nflux --log-format json
```

Then:

```shell
sudo nflux netrace FLAGS
# Or
sudo nflux tlstrace FLAGS
```

## netrace

By default, everything is enabled. Which means:

- Egress traffic
- Ingress traffic
- UDP/TCP/ICMP protocols
- Full packet logging

Let's see in the following sections how to customize `nflux netrace`.

### Help

First of all, take a look to the available flags:

```shell
sudo nflux netrace --help
```

### Sniffing (only) egress traffic

```shell
sudo nflux netrace --disable-ingress
```

### Sniffing (only) ingress traffic

```shell
sudo nflux netrace --enable-egress
```

### Packet logging

By default `nflux netrace` will log **all packets** (egress/ingress) entering the NIC (Network Interface). If you use `--disable-full-log`, you can use `--log-interval` to set the time interval in which the same `ip->port` connection will be logged.

For example:

```shell
ping 1.1.1.1
```

Every packet of type `icmp` to the ip `1.1.1.1` will be logged in the terminal.

Or:

```shell
curl http://external-ip
```

So, if you don't want to log every packet, you can should run the command:

```shell
sudo nlux netrace --disable-full-log # default to 5 seconds
```

Or you can also change the `--log-interval`:

```shell
sudo nflux netrace --disable-full-log --log-inveral 3 # every 3 seconds
```

### Available procotols

`UDP/TCP/ICMP` available by default.

To disable protocols like `udp`, `icmp`, `tcp`:

```shell
sudo nflux netrace --disable-udp --disable-icmp --disable-tcp
```

## tlstrace

```shell
sudo nflux tlstrace
```

# Compatibility

|   OS    | ARM64 | AMD64 | Kernel version |
|---------|------|------|------|
| fedora linux   | ✅    | ✅  |`6.13.7-200.fc41.x86_64 ` |

> For example, in Debian12 with kernel version `6.1.0-31-amd64` nflux doest not works. Probably for the version of kernel bpf.

# Docs

More documentation inside [`docs`](./docs/) folder:

- Todo and features
- Local development
- Old nflux

# Contribution

Any improvement is welcome! If you want to help me improve in Rust and eBPF, I'd be delighted!

# License

**`nflux`** is distributed under the terms of the [AGPL3](./LICENSE) license.
