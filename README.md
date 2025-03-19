<p align="center">
    <h1 align="center">nflux project</h1>
    <p align="center">Network monitoring tool using eBPF. Powered by Aya-rs 🐝</p>
    <p align="center">Built with ❤ in Rust</p>
</p>

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [What is nflux?](#what-is-nflux)
  - [What is ebpf?](#what-is-ebpf)
  - [Traffic control](#traffic-control)
- [Running `nflux`](#running-nflux)
- [nflux inside a container](#nflux-inside-a-container)
- [Local development](#local-development)
- [TODO](#todo)
- [old nflux](#old-nflux)
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


Network monitoring tool using eBPF. Powered by Aya-rs 🐝

Usage: nflux [OPTIONS] --interfaces <INTERFACES>...

Options:
  -l, --log-level <LOG_LEVEL>       Log level for logging tracing. Possible values: info, warn, trace, debug, error. Default: info [default: info]
  -i, --interfaces <INTERFACES>...  List of interfaces to attach the program
      --disable-egress              Disable egress
      --enable-ingress              Enable ingress traffic monitoring
      --disable-private-ips         Disable private ips network monitoring
      --enable-udp                  Enable udp network monitoring
  -h, --help                        Print help
  -V, --version                     Print version
```

# What is nflux?

Nflux is... (pending to finish)

## What is ebpf?

Provide basic concepts of `ebpf` (pending to finish)

## Traffic control

Provide some diagrams of TC (pending to finish)

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

Pending

# Local development

For local development, I don't use containers since the build process is slow. Visit this custom page for [`local development`](./docs/local_dev.md).

# TODO

If you want to take a look at the features I'll be trying to implement as much as possible, check out this doc [`todo_and_features.md`](./docs/todo_and_features.md).

# old nflux

> [!NOTE]
> [In this branch](https://github.com/containerscrew/nflux/tree/old-20250206) I tried to implement firewall functions using XDP, which I have now removed due to complexity given my initial knowledge of eBPF and control of TCP states (syn/ack/rst...etc) in ipv4, ipv6, among other problems. I will change the tool to be a simple cli to monitor eggress/ingress traffic using TC. As a first feature to learn how to use eBPF with Aya-rs, it is enough for someone who is learning.

# Contribution

Any improvement is welcome! If you want to help me improve in Rust and eBPF, I'd be delighted!

# License

**`nflux`** is distributed under the terms of the [AGPL3](./LICENSE) license.
