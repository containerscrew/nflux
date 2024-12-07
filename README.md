> [!WARNING]
> Ignore this README. It may change as I develop and adjust configurations.
> The entire tool is under development, while I am learning Rust and eBPF.
> I am not a Rust developer, so I am learning as I go. I am open to any suggestions or improvements.
> Code is not optimized, and I am not following best practices. I am learning and improving as I go.

<p align="center">
    <h3 align="center">nflux</h3>
    <p align="center">Network monitoring and firewall using EBPF, XDP and TC. Powered by Aya-rs</p>
    <p align="center">Built with ❤ in Rust</p>
</p>

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Nflux architecture](#nflux-architecture)
- [Features](#features)
- [Installation](#installation)
- [Contribution](#contribution)
- [License](#license)
<!-- END OF TOC -->


# Nflux architecture

Look at what level it works XDP:

![xdp](./xdp.png)

Powerful, right? Same for traffic control (TC).

![tc](./tc.png)

> [!NOTE]
> nflux uses XDP for incoming packet processing (only works with physical interfaces). For outgoing packets, it uses TC. If you want to use it with a virtual interface, you need to use the `tc` mode which is not implemented yet.

# Features

Basic XDP firewall:

* Block incoming ipv4/ipv6-tcp/udp packets.
* Allow incoming ipv4/ipv6-tcp/udp packets.
* Block incoming ICMP packets.
* Filter outgoing packets.

# Installation

By the moment, the quickest way to install **`nflux`** is using containers. Let's see how to run `nflux` with `docker-compose`.

```bash
git clone https://github.com/containerscrew/nflux.git
make compose-build
```

Before running the container, you need to edit the configuration file [`nflux.toml`](./nflux.toml). The most important configuration is the `interface` name.

```bash
ip link show # get the name of your PHISICAL interface
# Once is changes in the conf file, lets run nflux
make compose-up
```

> By default, nflux will allow SSH (22) connections from any IP. Avoid blocking your SSH connection if testing in remote servers (VPS).

# Contribution

Any improvement is welcome! If you want to help me improve in Rust and eBPF, I'd be delighted!

# License

**`nflux`** is distributed under the terms of the [AGPL3](./LICENSE) license.
