<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [Local development](#local-development)
  - [Requirements](#requirements)
  - [Running nflux](#running-nflux)
  - [Debugging](#debugging)
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
