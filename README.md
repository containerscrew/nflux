> [!WARNING]
> This tool is under development :)

> [!NOTE]
> [In this branch](https://github.com/containerscrew/nflux/tree/old-20250206) I tried to implement firewall functions using XDP, which I have now removed due to complexity given my initial knowledge of eBPF and control of TCP states (syn/ack/rst...etc) in ipv4, ipv6, among other problems. I will change the tool to be a simple cli to monitor eggress/ingress traffic using TC. As a first feature to learn how to use eBPF with Aya-rs, it is enough for someone who is learning.

<p align="center">
    <h3 align="center">nflux</h3>
    <p align="center">Network monitoring and firewall using EBPF, XDP and TC. Powered by Aya-rs</p>
    <p align="center">Built with ❤ in Rust</p>
</p>

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [nflux](#nflux)
- [Running `nflux`](#running-nflux)
- [Features](#features)
- [Local development](#local-development)
- [Contribution](#contribution)
- [License](#license)
<!-- END OF TOC -->

# nflux

```bash
2025-02-07-10:56:48  INFO Starting nflux with pid 61345
2025-02-07-10:56:48  INFO Metrics server running at http://0.0.0.0:8080
2025-02-07-10:56:48  INFO tc_egress program attached to interfaces: ["enp0s20f0u4"]
2025-02-07-10:56:48  INFO tc_ingress program attached to interfaces: ["enp0s20f0u4"]
2025-02-07-10:56:48  INFO Waiting for Ctrl-C...
2025-02-07-10:56:48  INFO ingress protocol=udp, src_ip=185.76.11.17, dst_ip=192.168.0.173, src_port=443, dst_port=43548
2025-02-07-10:56:48  INFO egress protocol=icmp, src_ip=192.168.0.173, dst_ip=185.76.11.17, src_port=0, dst_port=0
2025-02-07-10:56:49  INFO ingress protocol=udp, src_ip=185.76.11.17, dst_ip=192.168.0.173, src_port=443, dst_port=43548
2025-02-07-10:56:49  INFO egress protocol=icmp, src_ip=192.168.0.173, dst_ip=185.76.11.17, src_port=0, dst_port=0
2025-02-07-10:56:49  INFO egress protocol=udp, src_ip=192.168.0.173, dst_ip=239.255.255.250, src_port=46230, dst_port=1900
2025-02-07-10:56:49  INFO ingress protocol=udp, src_ip=192.168.0.13, dst_ip=192.168.0.173, src_port=1900, dst_port=46230
2025-02-07-10:56:49  INFO ingress protocol=udp, src_ip=192.168.0.23, dst_ip=192.168.0.173, src_port=1900, dst_port=46230
2025-02-07-10:56:49  INFO ingress protocol=udp, src_ip=185.76.11.17, dst_ip=192.168.0.173, src_port=443, dst_port=43548
2025-02-07-10:56:49  INFO egress protocol=icmp, src_ip=192.168.0.173, dst_ip=185.76.11.17, src_port=0, dst_port=0
```

# Running `nflux`

> [!NOTE]
> Setup [local development](./docs/local_dev.md) before using `nflux`. Is the only way by the moment

```bash
git clone https://github.com/containerscrew/nflux
make local-build
sudo ./target/release/nflux -i wlo1
sudo ./target/release/nflux -l info -i proton0 --enable-udp # for iface using wireguard
```

# Features

Pending to add...

# Local development

For local development, I don't use containers since the build process is slow. Visit this custom page for [`local development`](./docs/local_dev.md).

# Contribution

Any improvement is welcome! If you want to help me improve in Rust and eBPF, I'd be delighted!

# License

**`nflux`** is distributed under the terms of the [AGPL3](./LICENSE) license.
