<p align="center">
    <h1 align="center">nflux project</h1>
    <p align="center">Network monitoring tool and tls/ssl sniffer using eBPF. Powered by Aya-rs 🐝</p>
    <p align="center">Kernel and user space code written entirely in Rust ❤</p>
</p>

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [What is nflux?](#what-is-nflux)
  - [What is ebpf?](#what-is-ebpf)
  - [Traffic control](#traffic-control)
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


Network monitoring tool and tls/ssl sniffer using eBPF. Powered by Aya-rs 🐝

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
