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

**Nflux has been created/tested in:**

| OS                        | ARM64 | x86_64 | Kernel version            |
|---------------------------|-------|--------|---------------------------|
| Linux fedora 41           | ✅     | ✅      | `6.13.7-200.fc41.x86_64 ` |
| Linux ubuntu 24.10-server | ✅     | ✅      | `6.11.0-19-generic `      |
| Linux debian 12           | ✅     | ✅      | `6.1.0-35-cloud-arm64`    |

> [!TIP]
> If you want to try nflux on your Mac OSX, check out the ['Using Mac OSX section' in local dev doc](./local_dev.md).

# Installation

## Supported Platforms

| Arch  | arm64 | x86_64 |
|-------|-------|--------|
| linux | ✅     | ✅      |

> Remember: `eBPF` is native for Linux (I think they are implementing it in Windows)

## Install latest version

```shell
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/containerscrew/nflux/main/build/install.sh | sh
```

## Install specific version

```shell
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/containerscrew/nflux/main/build/install.sh | sh -s -- -v "v0.2.0"
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
