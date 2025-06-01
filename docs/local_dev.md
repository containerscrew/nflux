<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Local development](#local-development)
  - [Requirements](#requirements)
  - [Running nflux](#running-nflux)
  - [Debugging](#debugging)
- [Using Mac OSX?](#using-mac-osx)
  - [Fedora](#fedora)
  - [Ubuntu](#ubuntu)
  - [Debian](#debian)
<!-- END OF TOC -->

# Local development

For more information, please visit [official documentation](https://aya-rs.dev/book/).

## Requirements

1. Install rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install nightly toolchain:
```
rustup install stable && rustup toolchain install nightly --component rust-src
rustup default nightly
```

3. Optional, if using mac or other linux:

```shell
LLVM_SYS_180_PREFIX=$(brew --prefix llvm) cargo install --no-default-features bpf-linker
```

3. **MANDATORY:**
```shell
cargo install bpf-linker
```

## Running nflux

```shell
cargo run --release --config 'target."cfg(all())".runner="sudo -E"'
```

Build:

```shell
cargo check
cargo build --release
```

## Debugging

```shell
sudo bpftool prog list # show ebpf running programs
ip link show dev wlo1 # xdp attached to your interface
```

# Using Mac OSX?

For development in a Mac OSX (Apple Silicon) environment, you can use [**lima**](https://github.com/lima-vm/lima) (VM using qemu or Apple silicion virtualization).

See [`examples`](../lima/).

## Fedora

Take a look at the `fedora.yml` configuration file. Change the relevant directories to your local path where the nflux code is located. Creating an SSH key is optional.

Deploy the machine:

```shell
limactl start --name fedora lima/fedora.yml
limactl shell fedora
cd /your/mapped/directory/with/your/code
# Example
cd /Users/dcr/Documents/Code/Personal/nflux # this is my mapped path from OSX local to the fedora VM machine
```

Cleanup:

```shell
limactl stop fedora && limactl delete fedora
```

## Ubuntu

Using `ubuntu` (debian based):

```shell
limactl start --name ubuntu lima/ubuntu.yml
limactl shell ubuntu
# Reproduce same commands from above, like in fedora ...
limactl stop ubuntu && limactl delete ubuntu
```

## Debian

Using `debian`:

```shell
limactl start --name debian lima/debian.yml
limactl shell debian
# Reproduce same commands from above
limactl stop debian && limactl delete debian
```
