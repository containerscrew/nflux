# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## 0.12.5 - 2025-09-01
#### Documentation
- update project_notes.md - (3e61f43) - containerscrew
- new file project_notes.md - (d73036e) - containerscrew
#### Miscellaneous Chores
- **(deps)** bump tracing-subscriber from 0.3.19 to 0.3.20 - (d3998b6) - dependabot[bot]
- **(deps)** bump clap from 4.5.45 to 4.5.46 - (4007fc7) - dependabot[bot]
- **(deps)** bump dns-lookup from 2.1.0 to 3.0.0 - (ae4f3a3) - dependabot[bot]
- **(deps)** bump network-types from 0.0.8 to 0.1.0 - (05bc1dc) - dependabot[bot]
- **(deps)** bump async-trait from 0.1.88 to 0.1.89 - (48e9b9a) - dependabot[bot]
- **(deps)** bump anyhow from 1.0.98 to 1.0.99 - (352a09d) - dependabot[bot]
- **(deps)** bump clap from 4.5.44 to 4.5.45 - (a8fb32b) - dependabot[bot]
- **(deps)** bump clap from 4.5.43 to 4.5.44 - (8c52dd0) - dependabot[bot]
- **(deps)** bump slab from 0.4.10 to 0.4.11 - (d84e3c7) - dependabot[bot]
- **(deps)** bump libc from 0.2.174 to 0.2.175 - (01bfb05) - dependabot[bot]
- **(deps)** bump sysinfo from 0.36.1 to 0.37.0 - (c05429a) - dependabot[bot]
- **(deps)** bump clap from 4.5.42 to 4.5.43 - (4801b90) - dependabot[bot]
- **(deps)** bump dns-lookup from 2.0.4 to 2.1.0 - (2d0e09d) - dependabot[bot]
- **(deps)** bump tokio from 1.47.0 to 1.47.1 - (b6253dd) - dependabot[bot]
- **(deps)** bump clap from 4.5.41 to 4.5.42 - (10f8867) - dependabot[bot]
- **(deps)** bump tokio from 1.46.1 to 1.47.0 - (c60930f) - dependabot[bot]
- **(deps)** bump sysinfo from 0.36.0 to 0.36.1 - (d7d21bb) - dependabot[bot]
- **(version)** 0.12.5 - (0e4ff16) - containerscrew
- updating containerd implementation - (df99e42) - containerscrew
- update cog.toml - (108e671) - containerscrew
- update changelog for v0.12.4 - (a229ee0) - containerscrew
- update changelog for v0.12.2 - (93fb9bd) - containerscrew
- update changelog for v0.12.2 - (68f4c39) - containerscrew
- update changelog for v0.12.2 - (f14aa8a) - containerscrew
- update changelog for v0.12.2 - (7ffb0b3) - containerscrew
#### Refactoring
- **(TcEvent)** allow Ipv4 and Ipv6 in TcEvent struct & refactor handle_packet code) - (164d7ac) - containerscrew
- **(cli)** about message - (6e21810) - containerscrew
- **(cli)** help message & cargo fmt - (14e0831) - containerscrew
- **(cli)** help message & cargo fmt - (1ea0416) - containerscrew
- **(cli)** help message & cargo fmt - (02c63a1) - containerscrew
- **(ebpf-logger)** comment ebpf-logger - (24806c6) - containerscrew
- update edition in Cargo.toml - (531b233) - containerscrew
- delete changelog.md - (1517c94) - containerscrew
- new nflux version with ebpf code refactor - (a37df56) - containerscrew
- cog and pre-commit script - (1d02385) - containerscrew
- cog and pre-commit script - (7f11925) - containerscrew
- cog and pre-commit script - (e74ef43) - containerscrew
- cog and pre-commit script - (0122500) - containerscrew
- edit cog.toml - (80e9d0a) - containerscrew
- refactoring - (82d0c24) - containerscrew
- refactoring tc ebpf code & implement ARP packet sniffing - (112500f) - containerscrew
- refactoring ebpf dpkt code - (df0a9a9) - containerscrew
- refactoring - (256cd17) - containerscrew
- refactoring tc ebpf code & implement ARP packet sniffing - (2c6627a) - containerscrew
- refactoring ebpf code folder structure - (cd2e0d4) - containerscrew
- unused imports and utils::is_ipv4_private_address - (3ccfe22) - containerscrew
- cgroup skb attach type - (289b41a) - containerscrew
- generic process_ring_buffer function - (125c550) - containerscrew
- renaming folder structure - (8f8fac7) - containerscrew
- pre-commit and cargo package release metadata - (e5ba4cc) - containerscrew
- cli subcommands & file structure - (283d4e8) - containerscrew
- rename some functions & delete comments in nflux/src/main.rs - (f13d17a) - containerscrew
- folder structure - (b81e97f) - containerscrew
- handle_packet function - (70bca98) - containerscrew
#### ➕ Additional features
- **(doc)** add new examples section - (854acaf) - containerscrew
- **(exclude-ports)** new feature flag --exlude-ports from the logger) - (94be6a7) - containerscrew
- **(filter-ports)** implement cli flag --filter-ports - (836da65) - containerscrew
- **(listen-port)** implement flag --listen-port - (bd8b85d) - containerscrew
- **(listen-ports)** filtering listen-ports in ebpf code - (bedf338) - containerscrew
- **(logger)** change logger pkt_len field - (78cabeb) - containerscrew
- **(logger)** change logger pkt_len field - (b12fb57) - containerscrew
- **(tcp_flags)** implement tcp flags & remove pid and command track for egress connections - (15b44a1) - containerscrew
- implementing containerd cgroup support - (d5c9357) - containerscrew
- implementing containerd cgroup support - (d544ad6) - containerscrew
- implementing containerd cgroup support - (2c501c2) - containerscrew
- adding ebpf-cgroups for container networking - (9c70976) - containerscrew
- wip: adding support for cgroup sniffing - (44ff1b1) - containerscrew
- new version v0.12.4 - (462e82d) - containerscrew
- add verbosity for unknown procotols - (a34a2ba) - containerscrew
- refactor to eliminate high stack allocation, reducing verifier stack usage - (51d746b) - containerscrew
- nflux subcommands & file and code refactor - (a1aa5a3) - containerscrew
- implement new feature to detect dropped packets - (1303fbd) - containerscrew
- some print messagess - (d4b4ad3) - containerscrew
#### 🪲 Releases
- **(cli)** help message in --log-inverval flag - (0ca6c1b) - containerscrew
- **(cli)** merge conflix cli.rs - (eeb460c) - containerscrew
- **(doc)** typo in usage.md - (7e3af55) - containerscrew
- **(install.sh)** relocate install.sh and fix script - (f65fbb5) - containerscrew
- **(install.sh)** shebang and welcome message - (2a6e148) - containerscrew
- **(test)** is_root_user - (5099c5f) - containerscrew
- sudo permissions in test pipeline - (1dd19a7) - containerscrew
- protobuf-compiler dependency in test.yml github workflow - (09c09dd) - containerscrew
- protobuf-compiler dependency in github ci - (e7008ec) - containerscrew
- protobuf-compiler dependency in github ci - (1c67bb2) - containerscrew
- convert_protocol helper function - (74696e9) - containerscrew
- cargo deb metadata - (4990f01) - containerscrew
- process_tc_events json logger - (2723de4) - containerscrew
- test cli::test::test_print_help_message - (0e50431) - containerscrew
- test & update README.md - (66f05d2) - containerscrew
- cargo deb metadata - (1a425a6) - containerscrew
- release pipeline - (b0c1e8f) - containerscrew
- release pipeline - (24bc7c9) - containerscrew
- test to_ip_addr - (06bb8dc) - containerscrew
- split ebpf code to avoid errors in stack size - (c4d282f) - containerscrew
- nostd alloc for impl Display in TcpFlags struct - (4d182f1) - containerscrew
- rustfmt - (addc081) - containerscrew
- reimplement --disable-tcp|udp|icmp flag - (cd11ffa) - containerscrew
- error message when programs runs without sudo - (57bd224) - containerscrew
- error when no default iface (no connectivity) - (cb2db98) - containerscrew

- - -

## 0.12.5 - 2025-09-01
#### Refactoring
- update edition in Cargo.toml - (531b233) - containerscrew

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).