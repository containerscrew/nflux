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
- [Useful commands](#useful-commands)
- [License](#license)
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

1. Install rust `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. `rustup install stable && rustup toolchain install nightly --component rust-src`
3. Optional, if using mac or other linux: `LLVM_SYS_180_PREFIX=$(brew --prefix llvm) cargo install --no-default-features bpf-linker`
3. **MANDATORY:** `cargo install bpf-linker`

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

Ok now you can try to open a port in your local machine, for example, port 8081, using a simple docker container:

```shell
docker run -itd --rm --name test -p 8081:80 docker.io/nginx:latest 
```

Run the firewall:

```shell
# if you cloned the repo in the first step
# in the root directory of the project
cargo xtask run --
```

If everything works as expected, you will see the application log. By default, will only allow port `8080`, as the [config.toml](./config.toml) file says.

Let's test it:

From other laptop/device in your network: 

```shell
curl http://192.168.0.X:8081 # the ip where the firewall is running
```

You will be blocked! Try to stop the firewall, change the port to 8081 in the [config.toml](./config.toml) file and run the firewall again.

**Now you can access!**

# Useful commands

```bash
sudo bpftool prog list # show ebpf running programs
ip link show dev wlo1 # xdp attached to your interface
``` 

# License

[LICENSE](./LICENSE)
