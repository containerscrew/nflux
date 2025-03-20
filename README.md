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
