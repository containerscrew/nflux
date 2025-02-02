> This tool is under development :)

<p align="center">
    <h3 align="center">nflux</h3>
    <p align="center">Network monitoring and firewall using EBPF, XDP and TC. Powered by Aya-rs</p>
    <p align="center">Built with ❤ in Rust</p>
</p>

<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Nflux architecture](#nflux-architecture)
- [Features](#features)
- [Running `nflux`](#running-nflux)
- [Local development](#local-development)
- [Contribution](#contribution)
- [License](#license)
<!-- END OF TOC -->


# Nflux architecture

Look at what level it works XDP:

![xdp](./xdp.png)

Powerful, right? Same for traffic control (TC).

![tc](./tc.png)

> [!NOTE]
> nflux uses XDP for incoming packet processing (only works with physical interfaces).

# Features

Pending to add...

# Running `nflux`

> [!WARNING]
> In Fedora, where selinux is enforced by default, I'm having some problems.
> Quick fix (not secure): `sudo setenforce 0`

```bash
ip link show # get the name of your PHYSICAL interface
# edit nflux.toml and set your physical interface
```

> Monitoring for VPN interfaces like wireguard, not working properly yet

By the moment, run `nflux` locally (see next [local-dev](https://github.com/containerscrew/nflux?tab=readme-ov-file#local-development))

# Local development

For local development, I don't use containers since the build process is slow. Visit this custom page for [`local development`](./docs/local_dev.md).

# Contribution

Any improvement is welcome! If you want to help me improve in Rust and eBPF, I'd be delighted!

# License

**`nflux`** is distributed under the terms of the [AGPL3](./LICENSE) license.
