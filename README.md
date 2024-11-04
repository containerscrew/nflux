<p align="center">
    <h3 align="center">nflux</h3>
    <p align="center">Network monitoring and firewall using EBPF, XDP and TC. Powered by Aya-rs</p>
    <p align="center">Built with ❤ in Rust</p>
</p>

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Intro](#intro)
- [Features](#features)
  - [Basic XDP firewall](#basic-xdp-firewall)
  - [Outgoing traffic monitoring](#outgoing-traffic-monitoring)
- [Using `nflux`](#using-nflux)
  - [Config.toml](#configtoml)
- [Testing firewall](#testing-firewall)
- [Debugging](#debugging)
<!-- END OF TOC -->

> [!IMPORTANT]
> By the moment this tool is under development

# Intro

Look at what level it works XDP:

![xdp](./xdp.png)

Powerful, right? Same for traffic control (TC).

![tc](./tc.png)

# Features

## Basic XDP firewall

* Block TCP SYN incoming packets
* Allow incoming SYN-ACK incoming packets (for example, you are using your browser)
* Block ICMP incoming packets
* User can allow traffic for specific incoming ports

## Outgoing traffic monitoring

in progress

# Using `nflux`

First of all, clone the repo:

```shell
git clone https://github.com/containerscrew/nflux.git
```

Since this project is under development just for fun and learning, you need to compile the project in your local:

1. Install rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install nightly toolchain:
```
rustup install stable && rustup toolchain install nightly --component rust-src
```

3. Optional, if using mac or other linux:
```bash
LLVM_SYS_180_PREFIX=$(brew --prefix llvm) cargo install --no-default-features bpf-linker
```

3. **MANDATORY:**
```bash
cargo install bpf-linker
```

## Config.toml

You can manage the firewall from the file [config.toml](./config.toml). The most important setting is the network interface.
Set your network interface correctly.

```shell
ip route # check default via
ip link show # then copy the name of the interface and put it in the config.toml
nvim config.toml # change the interface name
```

> [!CAUTION]
> nflux uses XDP for packet processing. Only works with physical interfaces. If you want to use it with a virtual interface, you need to use the `tc` mode which is not implemented yet.
> For example, you want to monitor incoming traffic using a virtual interface like `tun0` (VPN), you need to use the `tc` mode.

# Testing firewall
Now you can try to map some services using docker. For example, let's expose an nginx server (tcp) and bind9 (udp):

```shell
make compose-up
```

Test the exposed services works. (Recommended) test the services from other device in the same network:

* Nginx server:

```bash
curl http://ip:8081 # Welcome to nginx!
```

* Bind9 server:

```bash
dig @ip -p 5053 mycompany.org A
```

> Change `ip` to the ip of the machine where you are running the docker-compose and where you will run the firewall.

Now, since the exposed port of `nginx` for example is `8081`, let's run the firewall without any allowed port:

For example, in `config.toml`:

```toml
[log]
log_level = "info"

[nflux]
interface_name = "wlo1"

[firewall]
# All incoming connections will be blocked by default
# You can specify allowed IP addresses and ports
# This is a basic approach
allowed_ipv4 = [] # Specify IP addresses you want to allow
allowed_ports = [] # Specify ports you want to allow
allow_icmp = false
```

Try again `curl http://ip:8081` and you will see that the connection is blocked.

Change the `interface_name` to your physical interface name, also you can play changing the `allowed_ipv4` and `allowed_ports` to allow some traffic.

# Debugging

```shell
```bash
sudo bpftool prog list # show ebpf running programs
ip link show dev wlo1 # xdp attached to your interface
```

# Contribution

Any improvement is welcome! If you want to help me improve in Rust and eBPF, I'd be delighted!

# License

**`nflow`** is distributed under the terms of the [AGPL3](./LICENSE) license.
