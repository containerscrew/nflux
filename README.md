<p align="center">
    <h1 align="center">nflux project</h1>
    <p align="center">Network monitoring tool & TLS/SSL sniffer using eBPF. Powered by Aya-rs 🐝</p>
    <p align="center">Kernel and user space code written entirely in Rust ❤</p>
</p>

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
- [Running `nflux`](#running-nflux)
- [nflux inside a container](#nflux-inside-a-container)
- [Docs](#docs)
- [Contribution](#contribution)
- [License](#license)
<!-- END OF TOC -->


```bash
$ nflux --help

    ███╗   ██╗███████╗██╗     ██╗   ██╗██╗  ██╗
    ████╗  ██║██╔════╝██║     ██║   ██║╚██╗██╔╝
    ██╔██╗ ██║█████╗  ██║     ██║   ██║ ╚███╔╝
    ██║╚██╗██║██╔══╝  ██║     ██║   ██║ ██╔██╗
    ██║ ╚████║██║     ███████╗╚██████╔╝██╔╝ ██╗
    ╚═╝  ╚═══╝╚═╝     ╚══════╝ ╚═════╝ ╚═╝  ╚═╝


Network monitoring tool & TLS/SSL sniffer using eBPF. Powered by Aya-rs 🐝

Usage: nflux [OPTIONS]

Options:
  -l, --log-level <LOG_LEVEL>    Log level for logging tracing. Possible values: info, warn, trace, debug, error. [default: info]
      --log-format <LOG_FORMAT>  Log format for logging tracing. Possible values: text, json. [default: text]
  -i, --interface <INTERFACE>    Interface to attach the program. [default: proton0]
      --enable-egress            Enable egress traffic monitoring. [default: true]
      --enable-ingress           Enable ingress traffic monitoring. [default: false]
      --enable-udp               Enable udp protocol network monitoring. [default: false]
      --enable-icmp              Enable icmp protocol network monitoring. [default: false]
      --enable-tcp               Enable tcp protocol network monitoring. [default: true]
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
sudo nflux netrace
# Or
sudo nflux tlstrace
```

## netrace

By default, `egress/ingress` monitoring are disabled.

### Help

```shell
sudo nflux netrace --help
```

### Sniffing (only) egress traffic

```shell
sudo nflux netrace --enable-egress
```

### Sniffing (only) ingress traffic

```shell
sudo nflux netrace --enable-ingress
```

### Packet logging

By default `nflux netrace` will save the active connection in an `eBPF` map and will log the same connection every `5 seconds`. That means, If I run the command:

```shell
ping 1.1.1.1
```

Packets of type `icmp` to the ip `1.1.1.1` will be logged in the terminal every `5 seconds` by default.

Or:

```shell
curl http://external-ip
```

So, if you want to log every packet, you can should run the command:

```shell
sudo nlux netrace --enable-egress --full-log
```

Or you can also change the `log-interval`:

```shell
sudo nflux netrace --enable-egress --log-inveral 3 # every 3 seconds
```

### Available procotols

By default when you set `--enable-egress` or `--enable-ingress`, the flag `--enable-tcp` is also enabled by default.

To enable protocols like `udp` or `icmp`:

```shell
sudo nflux netrace --enable-ingress --enable-icmp --enable-udp
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


# Running `nflux`

> [!NOTE]
> Setup [local development](./docs/local_dev.md) before using `nflux`. Is the only way by the moment

```shell
# clone the repo
cd nflux/
# Need privilege permissions
make local-install # by default installed in /usr/local/bin/nflux. Check your $PATH.
```
---

```shell
nflux --help
```

# nflux inside a container

```shell
podman run --rm -it --name nflux --privileged --net host docker.io/containerscrew/nflux:latest
```

> By the moment `latest` tag

# Docs

More documentation inside [`docs`](./docs/) folder:

- Todo and features
- Local development
- Old nflux

# Contribution

Any improvement is welcome! If you want to help me improve in Rust and eBPF, I'd be delighted!

# License

**`nflux`** is distributed under the terms of the [AGPL3](./LICENSE) license.
