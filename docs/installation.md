<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Compatibility](#compatibility)
- [Installation](#installation)
  - [Supported Platforms](#supported-platforms)
  - [Install latest version](#install-latest-version)
  - [Install specific version](#install-specific-version)
  - [Using cargo](#using-cargo)
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

## Using cargo

> [!WARNING]
> I haven't uploaded any releases to crates.io yet. I'm having some problems pushing a crate with multiple workspaces.

_Remember to install `cargo` with rustup_

```shell
cargo install nflux
```

> If you want to update the tool to a new version, just the run the same command `cargo install nflux` or `cargo install nflux@vX.X.X`.

## Uninstall

Binary:

```shell
sudo rm /usr/local/bin/nflux
```

With cargo:

```shell
cargo uninstall nflux
```
