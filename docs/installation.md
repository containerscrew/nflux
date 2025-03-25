<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Compatibility](#compatibility)
- [Installation](#installation)
  - [Supported Platforms](#supported-platforms)
  - [Install latest version](#install-latest-version)
  - [Install specific version](#install-specific-version)
  - [From source](#from-source)
  - [Uninstall](#uninstall)
<!-- END OF TOC -->
# Compatibility

**Nflux has been created and tested in:**

|   OS    | ARM64 | x86_64 | Kernel version |
|---------|------|------|------|
| fedora linux   | ? (not tested)    | ✅  |`6.13.7-200.fc41.x86_64 ` |

> For example, in Debian12 with kernel version `6.1.0-31-amd64` nflux doest not works. Probably for the version of kernel bpf implementation. Missing some bpf helper functions.

> [!TIP]
> If you want to try nflux on your Mac OSX, check out the ['Using Mac OSX section' in local dev doc](./local_dev.md).

# Installation

## Supported Platforms

| Arch    | arm64 | x86_64 |
|---------|------|------|
| linux   | ✅    | ✅  |

> Remember: `eBPF` is native for Linux.

## Install latest version

```shell
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/containerscrew/nflux/main/install.sh | sh
```

## Install specific version

```shell
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/containerscrew/nflux/main/install.sh | sh -s -- -v "v0.2.0"
```

## From source

> First of all [setup your local environment](./local_dev.md)

```shell
git clone https://github.com/containerscrew/nflux.git && cd nflux/
make local-install
```

## Uninstall

Binary:

```shell
sudo rm /usr/local/bin/nflux
```
