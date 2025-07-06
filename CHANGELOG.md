# Changelog

All notable changes to this project will be documented in this file.

## [0.12.2] - 2025-07-06

- Change licensing

- Update Makefile

- Fix package deb metadata

- Temporary comment some Tcp flags to avoid stack limit size

## [0.12.1] - 2025-07-06

- Update CHANGELOG.md for release v0.12.0

- Update CHANGELOG.md

- Update build.yml pipeline

- Update cargo deb metadata

- New version v0.12.1 including .deb package

- Update handle TcpFlags

- Add cargo deb in release pipeline

## [0.12.0] - 2025-07-06

- Update CHANGELOG.md for release v0.11.0

- Update release pipeline

- Update git hooks

- Update CHANGELOG.md

- Update git hook pre-push

- Update pre-push git hook

- Fix: split ebpf code to avoid errors in stack size

- Fix: test to_ip_addr

- Fix: release pipeline

## [0.11.0] - 2025-07-05

- Update CHANGELOG.md for release v0.10.0

- Bump tokio from 1.45.1 to 1.46.0

Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.45.1 to 1.46.0.
- [Release notes](https://github.com/tokio-rs/tokio/releases)
- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.45.1...tokio-1.46.0)

---
updated-dependencies:
- dependency-name: tokio
  dependency-version: 1.46.0
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #64 from containerscrew/dependabot/cargo/tokio-1.46.0

Bump tokio from 1.45.1 to 1.46.0

- Bump tokio from 1.46.0 to 1.46.1

Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.46.0 to 1.46.1.
- [Release notes](https://github.com/tokio-rs/tokio/releases)
- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.46.0...tokio-1.46.1)

---
updated-dependencies:
- dependency-name: tokio
  dependency-version: 1.46.1
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #65 from containerscrew/dependabot/cargo/tokio-1.46.1

Bump tokio from 1.46.0 to 1.46.1

- Change subcommands

- Remove tokio thread in function process_tc_events

- Update dependency

- Add more tcp flags & IpFamily impl

- Update examples

- Rename example2

- Update pre-commit

- Update README.md

- Update CHANGELOG.md for release v0.11.0

- New nflux version v0.11.0

## [0.10.0] - 2025-06-28

- New infra/ folder to store compose and docker files

- Update CHANGELOG.md for release v0.9.0

- Fix loki config

- Setup monitoring stack

- Update README.md

- Adding more socket information in dropped-pkt

- Update Makefile

- Fix SkFamily data in dropped-pkt

- Add vmlinux.rs & remove unused imports

- Update CHANGELOG.md for release v0.10.0

- New release v0.10.0

## [0.9.0] - 2025-06-26

- Update CHANGELOG.md for release v0.8.0

- Update code comments in nflux-ebpf

- Remove docs/ folder in favour of Github Wiki

- Change imports rustfmt

- Refactor: rename some functions & delete comments in nflux/src/main.rs

- Fix(test): is_root_user

- Feat: implement new feature to detect dropped packets

- Update doc and examples

- Update README.md

- Add new example

- Wip: dropped packets feature

- Fix event struct in nflux-ebpf

- Refactor: cli subcommands & file structure

- Json logging for dropped-pkt & skip unknown reasons

- Feat: nflux subcommands & file and code refactor

- Update examples

- Add full example

- Revert "Add full example"

This reverts commit 4bd5bfb15f96a38fe805aa5d0499a5f22cce4659.

## [0.8.0] - 2025-06-18

- Update CHANGELOG.md for release v0.7.0

- Update example images

- Change fn_params_layout carfo fmt to Vertical

- Update rustfmt.toml

- Update usage.md

- Update Cargo.toml version

## [0.7.0] - 2025-06-18

- Update CHANGELOG.md for release v0.6.1

- Feat(tcp_flags): implement tcp flags & remove pid and command track for egress connections

- Fmt code

- Fix: nostd alloc for impl Display in TcpFlags struct

- Update Cargo.toml version

## [0.6.1] - 2025-06-17

- Update CHANGELOG.md for release v0.6.0

- Fix pid and process field in json log

## [0.6.0] - 2025-06-17

- Update CHANGELOG.md for release v0.5.0

- Refactor: folder structure

- Add new example

- Update README

- Improve ActiveConnectiont tracker & cli help message

- Update Cargo.toml version

- Update Cargo.lock version

## [0.5.0] - 2025-06-17

- Update CHANGELOG.md for release v0.4.0

- Fix(install.sh): relocate install.sh and fix script

- Fix build pipeline

- Improve gracefull shutdown & update log egress event

- Bump libc from 0.2.173 to 0.2.174

Bumps [libc](https://github.com/rust-lang/libc) from 0.2.173 to 0.2.174.
- [Release notes](https://github.com/rust-lang/libc/releases)
- [Changelog](https://github.com/rust-lang/libc/blob/0.2.174/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/libc/compare/0.2.173...0.2.174)

---
updated-dependencies:
- dependency-name: libc
  dependency-version: 0.2.174
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #63 from containerscrew/dependabot/cargo/libc-0.2.174

Bump libc from 0.2.173 to 0.2.174

- Fix release pipeline

## [0.4.0] - 2025-06-14

- Update CHANGELOG.md for release v0.2.1

- Pre-commit & CHANGELOG.md

- Bump clap from 4.5.33 to 4.5.34

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.33 to 4.5.34.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.33...clap_complete-v4.5.34)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #44 from containerscrew/dependabot/cargo/clap-4.5.34

Bump clap from 4.5.33 to 4.5.34

- Bump sysinfo from 0.33.1 to 0.34.1

Bumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.33.1 to 0.34.1.
- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)
- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)

---
updated-dependencies:
- dependency-name: sysinfo
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #45 from containerscrew/dependabot/cargo/sysinfo-0.34.1

Bump sysinfo from 0.33.1 to 0.34.1

- Setting up cargo deb

- Setup ubuntu machine using lima

- Setup debian dpkg package & refactor doc

- Wip: dpkg package

- Modify fedora.yml lima vm

- Change package description and delete package.metadata.deb

- Wip: rpm package

- Update readme

- Bump clap from 4.5.34 to 4.5.35

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.34 to 4.5.35.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.34...clap_complete-v4.5.35)

---
updated-dependencies:
- dependency-name: clap
  dependency-version: 4.5.35
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #46 from containerscrew/dependabot/cargo/clap-4.5.35

Bump clap from 4.5.34 to 4.5.35

- Fix build_deb.sh

- Bump sysinfo from 0.34.1 to 0.34.2

Bumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.34.1 to 0.34.2.
- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)
- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits)

---
updated-dependencies:
- dependency-name: sysinfo
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #47 from containerscrew/dependabot/cargo/sysinfo-0.34.2

Bump sysinfo from 0.34.1 to 0.34.2

- Bump tokio from 1.44.1 to 1.44.2

Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.44.1 to 1.44.2.
- [Release notes](https://github.com/tokio-rs/tokio/releases)
- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.44.1...tokio-1.44.2)

---
updated-dependencies:
- dependency-name: tokio
  dependency-version: 1.44.2
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #48 from containerscrew/dependabot/cargo/tokio-1.44.2

Bump tokio from 1.44.1 to 1.44.2

- Bump clap from 4.5.35 to 4.5.36

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.35 to 4.5.36.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.35...clap_complete-v4.5.36)

---
updated-dependencies:
- dependency-name: clap
  dependency-version: 4.5.36
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #49 from containerscrew/dependabot/cargo/clap-4.5.36

Bump clap from 4.5.35 to 4.5.36

- Bump anyhow from 1.0.97 to 1.0.98

Bumps [anyhow](https://github.com/dtolnay/anyhow) from 1.0.97 to 1.0.98.
- [Release notes](https://github.com/dtolnay/anyhow/releases)
- [Commits](https://github.com/dtolnay/anyhow/compare/1.0.97...1.0.98)

---
updated-dependencies:
- dependency-name: anyhow
  dependency-version: 1.0.98
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #50 from containerscrew/dependabot/cargo/anyhow-1.0.98

Bump anyhow from 1.0.97 to 1.0.98

- Bump libc from 0.2.171 to 0.2.172

Bumps [libc](https://github.com/rust-lang/libc) from 0.2.171 to 0.2.172.
- [Release notes](https://github.com/rust-lang/libc/releases)
- [Changelog](https://github.com/rust-lang/libc/blob/0.2.172/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/libc/compare/0.2.171...0.2.172)

---
updated-dependencies:
- dependency-name: libc
  dependency-version: 0.2.172
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #51 from containerscrew/dependabot/cargo/libc-0.2.172

Bump libc from 0.2.171 to 0.2.172

- Bump network-types from 0.0.7 to 0.0.8

Bumps [network-types](https://github.com/vadorovsky/network-types) from 0.0.7 to 0.0.8.
- [Release notes](https://github.com/vadorovsky/network-types/releases)
- [Changelog](https://github.com/vadorovsky/network-types/blob/main/CHANGELOG.md)
- [Commits](https://github.com/vadorovsky/network-types/compare/v0.0.7...v0.0.8)

---
updated-dependencies:
- dependency-name: network-types
  dependency-version: 0.0.8
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #52 from containerscrew/dependabot/cargo/network-types-0.0.8

Bump network-types from 0.0.7 to 0.0.8

- Bump clap from 4.5.36 to 4.5.37

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.36 to 4.5.37.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.36...clap_complete-v4.5.37)

---
updated-dependencies:
- dependency-name: clap
  dependency-version: 4.5.37
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #53 from containerscrew/dependabot/cargo/clap-4.5.37

Bump clap from 4.5.36 to 4.5.37

- Update doc and Cargo.toml version

- Bump chrono from 0.4.40 to 0.4.41

Bumps [chrono](https://github.com/chronotope/chrono) from 0.4.40 to 0.4.41.
- [Release notes](https://github.com/chronotope/chrono/releases)
- [Changelog](https://github.com/chronotope/chrono/blob/main/CHANGELOG.md)
- [Commits](https://github.com/chronotope/chrono/compare/v0.4.40...v0.4.41)

---
updated-dependencies:
- dependency-name: chrono
  dependency-version: 0.4.41
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #54 from containerscrew/dependabot/cargo/chrono-0.4.41

Bump chrono from 0.4.40 to 0.4.41

- Bump sysinfo from 0.34.2 to 0.35.0

Bumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.34.2 to 0.35.0.
- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)
- [Commits](https://github.com/GuillaumeGomez/sysinfo/commits/v0.35.0)

---
updated-dependencies:
- dependency-name: sysinfo
  dependency-version: 0.35.0
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #55 from containerscrew/dependabot/cargo/sysinfo-0.35.0

Bump sysinfo from 0.34.2 to 0.35.0

- Bump tokio from 1.44.2 to 1.45.0

Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.44.2 to 1.45.0.
- [Release notes](https://github.com/tokio-rs/tokio/releases)
- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.44.2...tokio-1.45.0)

---
updated-dependencies:
- dependency-name: tokio
  dependency-version: 1.45.0
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #56 from containerscrew/dependabot/cargo/tokio-1.45.0

Bump tokio from 1.44.2 to 1.45.0

- Bump clap from 4.5.37 to 4.5.38

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.37 to 4.5.38.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.37...clap_complete-v4.5.38)

---
updated-dependencies:
- dependency-name: clap
  dependency-version: 4.5.38
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #57 from containerscrew/dependabot/cargo/clap-4.5.38

Bump clap from 4.5.37 to 4.5.38

- Removing tlstrace code

- Wip

- Removing tlstrace

- Add some useful comments

- Add src and dest mac

- Refactor nflux/ package

- Add cargo nextest

- Rollback cargo nextest

- Refactor: handle_packet function

- Add format_mac unit test

- Cargo fmt & update doc

- Merge branch 'refactor/remove-tlstrace-feature'

- Update Cargo.toml version

- Change license to GPL3

- Fix proto & update doc

- Update doc

- Update install script

- Update CHANGELOG.md for release v0.3.0-beta.1

- Update README.md

- Add debian lima machine

- Update installation.md

- Split pipelines & organize lima files

- Update test pipeline

- Update lint.yml pipeline

- Wip: lint pipeline

- Comment lint pipeline

- Refactor handle_packet

- Implementing prometheus metrics

- Reimplement disable_full_log

- Remove prometheus metrics

- Improving logging

- Fix: error message when programs runs without sudo

- Bump clap from 4.5.38 to 4.5.39

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.38 to 4.5.39.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.38...clap_complete-v4.5.39)

---
updated-dependencies:
- dependency-name: clap
  dependency-version: 4.5.39
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #59 from containerscrew/dependabot/cargo/clap-4.5.39

Bump clap from 4.5.38 to 4.5.39

- Change nflux logger

- Add todo.md doc

- Change logger

- Pre-commit

- Allow user to disable timestamp and other logger improvements

- Update usage.md

- Fix test format_mac

- Change nflux project description

- Rust fmt

- Change cache in pipeline test

- Add cache build pipeline

- Change cli after_help message

- Refactor(TcEvent): allow Ipv4 and Ipv6 in TcEvent struct & refactor handle_packet code)

- Refactor(ebpf-logger): comment ebpf-logger

- Refactor(cli): help message & cargo fmt

- Doc(readme): update README

- Cargo fmt

- Bump sysinfo from 0.35.1 to 0.35.2

Bumps [sysinfo](https://github.com/GuillaumeGomez/sysinfo) from 0.35.1 to 0.35.2.
- [Changelog](https://github.com/GuillaumeGomez/sysinfo/blob/master/CHANGELOG.md)
- [Commits](https://github.com/GuillaumeGomez/sysinfo/compare/v0.35.1...v0.35.2)

---
updated-dependencies:
- dependency-name: sysinfo
  dependency-version: 0.35.2
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #60 from containerscrew/dependabot/cargo/sysinfo-0.35.2

Bump sysinfo from 0.35.1 to 0.35.2

- Feat(logger): change logger pkt_len field

- Doc(todo): add new todo

- Feat(filter-ports): implement cli flag --filter-ports

- Feat(listen-ports): filtering listen-ports in ebpf code

- Feat(listen-port): implement flag --listen-port

- Merge branch feat/filter-ports

- Fix(cli): merge conflix cli.rs

- Update CHANGELOG.md for release v0.3.0-beta.2

- Feat(exclude-ports): new feature flag --exlude-ports from the logger)

- Mtoc

- Fix(install.sh): shebang and welcome message

- Update CHANGELOG.md for release v0.3.0-beta.3

- Fix(install.sh)

- Mtoc in Changelog.md

- Fix: reimplement --disable-tcp|udp|icmp flag

- Fix: rustfmt

- Add cargo fmt in pre-commit config

- Change welcome messager install.sh

- Update CHANGELOG.md for release v0.3.0-beta.4

- Add mtoc in changelog release pipeline

- Update docs/todo.md

- Update mtoc in CHANGELOG.md

- Fix(cli): help message in --log-inverval flag

- Update CHANGELOG.md for release v0.3.0-beta.5

- Bump clap from 4.5.39 to 4.5.40

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.39 to 4.5.40.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.39...clap_complete-v4.5.40)

---
updated-dependencies:
- dependency-name: clap
  dependency-version: 4.5.40
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #61 from containerscrew/dependabot/cargo/clap-4.5.40

Bump clap from 4.5.39 to 4.5.40

- Fix(doc): typo in usage.md

- Update doco.md

- Feat(doc): add new examples section

- Add TCP state examples

- Handle TCP state flags

- Refactor(cli): about message

- Update todo.md

- Fix pipeline zip package

- Change after help message

- Bump libc from 0.2.172 to 0.2.173

Bumps [libc](https://github.com/rust-lang/libc) from 0.2.172 to 0.2.173.
- [Release notes](https://github.com/rust-lang/libc/releases)
- [Changelog](https://github.com/rust-lang/libc/blob/0.2.173/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/libc/compare/0.2.172...0.2.173)

---
updated-dependencies:
- dependency-name: libc
  dependency-version: 0.2.173
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #62 from containerscrew/dependabot/cargo/libc-0.2.173

Bump libc from 0.2.172 to 0.2.173

- Implement pid and process name for egress traffic

- New v0.4.0 & fix unused imports

## [0.2.1] - 2025-03-27

- Update CHANGELOG.md for release v0.2.0

- Bump clap from 4.5.32 to 4.5.33

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.32 to 4.5.33.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.32...clap_complete-v4.5.33)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #43 from containerscrew/dependabot/cargo/clap-4.5.33

Bump clap from 4.5.32 to 4.5.33

- Fix: error when no default iface (no connectivity)

## [0.2.0] - 2025-03-25

- Update CHANGELOG.md for release v0.1.0

- Add cargo.toml metadata

- Add versioning to Cargo.toml

- Fix LICENSE file

- Add lib.rs in netrace-ebpf

- Update doc

- Bump log from 0.4.26 to 0.4.27

Bumps [log](https://github.com/rust-lang/log) from 0.4.26 to 0.4.27.
- [Release notes](https://github.com/rust-lang/log/releases)
- [Changelog](https://github.com/rust-lang/log/blob/master/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/log/compare/0.4.26...0.4.27)

---
updated-dependencies:
- dependency-name: log
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #42 from containerscrew/dependabot/cargo/log-0.4.27

Bump log from 0.4.26 to 0.4.27

- Updating doc structure

- Update badges

- Update rustfmt

- Change test pipeline

- Add comm in netrace logger

- Comment cargo publish in release.yml pipeline

- Update nflux/Cargo.toml

- Update Cargo.toml

- Strip nflux release

- Delete workspace dependency for nflux-common

- Delete crates.io references

- Update installation.md

- Update CHANGELOG.md for release v0.2.0

- Cargo fmt

- Delete changelog

- Change cargo version

## [0.1.0] - 2025-03-24

- Initial commit

- First approach: deny all incoming traffic and allow syn-ack

- Implement config.toml and add new doc

- Update doc

- Reduce logging info for same ip

- Implement ipv4 access

- Rename the project to nflux

- Change cli description

- Fix incoming connection

- Add image examples

- Pre-commit

- Using perfbuff

- Create new testing tcp/udp services with compose

- Update README & wip ebpf program

- Add logging type (text,json)

- First stable version

- Update README

- Refactor logging

- Implementing AsyncPerfEventArrayBuffer

- Wip

- Implement new BPF_MAP_ARRAY for global app config & fix dpkg installation

- Rollback nflux.conf to nflux.toml file

- Update README.md

- Fix denied syn packets

- Feat: some print messagess

- Add new pipelines & some basic tests

- Modify pipeline tests.yml

- Working with firewall.rules config structure

- Populating ipv4 rules

- Working with rule implementation

- Change config format & some tests

- Bump clap from 4.5.21 to 4.5.22

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.21 to 4.5.22.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.21...clap_complete-v4.5.22)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #3 from containerscrew/dependabot/cargo/clap-4.5.22

Bump clap from 4.5.21 to 4.5.22

- Bump anyhow from 1.0.93 to 1.0.94

Bumps [anyhow](https://github.com/dtolnay/anyhow) from 1.0.93 to 1.0.94.
- [Release notes](https://github.com/dtolnay/anyhow/releases)
- [Commits](https://github.com/dtolnay/anyhow/compare/1.0.93...1.0.94)

---
updated-dependencies:
- dependency-name: anyhow
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #2 from containerscrew/dependabot/cargo/anyhow-1.0.94

Bump anyhow from 1.0.93 to 1.0.94

- Refactor tests

- Fmt

- Change pre-commit

- Change nflux.toml

- Fix prefix_len dynamically

- Update nflux.toml

- Permit tcp syn-ack packets & config testing

- Implement icmp enable/disable

- Nflux running in containers

- Implementing ipv6

- Change auto-merge pipeline

- Bump clap from 4.5.22 to 4.5.23

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.22 to 4.5.23.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.22...clap_complete-v4.5.23)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #4 from containerscrew/dependabot/cargo/clap-4.5.23

Bump clap from 4.5.22 to 4.5.23

- Refactor ebpf code & logging

- Change nflux.toml config

- Change docker-compose file

- Adding TC egress control

- Implement egress traffic monitoring

- Working with TC egress implementation

- Fix version of github action

- Bump serde from 1.0.215 to 1.0.216

Bumps [serde](https://github.com/serde-rs/serde) from 1.0.215 to 1.0.216.
- [Release notes](https://github.com/serde-rs/serde/releases)
- [Commits](https://github.com/serde-rs/serde/compare/v1.0.215...v1.0.216)

---
updated-dependencies:
- dependency-name: serde
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #6 from containerscrew/dependabot/cargo/serde-1.0.216

Bump serde from 1.0.215 to 1.0.216

- Bump libc from 0.2.167 to 0.2.168

Bumps [libc](https://github.com/rust-lang/libc) from 0.2.167 to 0.2.168.
- [Release notes](https://github.com/rust-lang/libc/releases)
- [Changelog](https://github.com/rust-lang/libc/blob/0.2.168/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/libc/compare/0.2.167...0.2.168)

---
updated-dependencies:
- dependency-name: libc
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #5 from containerscrew/dependabot/cargo/libc-0.2.168

Bump libc from 0.2.167 to 0.2.168

- Refactor ipv4 traffic

- Improve tcp states

- Working with ingress traffic

- Refactoring ipv4 tcp ingress proto

- Working with TCP state flags

- Fix config tests

- Some performance in nflux running inside a container

- Bump libc from 0.2.168 to 0.2.169

Bumps [libc](https://github.com/rust-lang/libc) from 0.2.168 to 0.2.169.
- [Release notes](https://github.com/rust-lang/libc/releases)
- [Changelog](https://github.com/rust-lang/libc/blob/0.2.169/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/libc/compare/0.2.168...0.2.169)

---
updated-dependencies:
- dependency-name: libc
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #7 from containerscrew/dependabot/cargo/libc-0.2.169

Bump libc from 0.2.168 to 0.2.169

- Bump env_logger from 0.11.5 to 0.11.6

Bumps [env_logger](https://github.com/rust-cli/env_logger) from 0.11.5 to 0.11.6.
- [Release notes](https://github.com/rust-cli/env_logger/releases)
- [Changelog](https://github.com/rust-cli/env_logger/blob/main/CHANGELOG.md)
- [Commits](https://github.com/rust-cli/env_logger/compare/v0.11.5...v0.11.6)

---
updated-dependencies:
- dependency-name: env_logger
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #8 from containerscrew/dependabot/cargo/env_logger-0.11.6

Bump env_logger from 0.11.5 to 0.11.6

- Bump anyhow from 1.0.94 to 1.0.95

Bumps [anyhow](https://github.com/dtolnay/anyhow) from 1.0.94 to 1.0.95.
- [Release notes](https://github.com/dtolnay/anyhow/releases)
- [Commits](https://github.com/dtolnay/anyhow/compare/1.0.94...1.0.95)

---
updated-dependencies:
- dependency-name: anyhow
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #9 from containerscrew/dependabot/cargo/anyhow-1.0.95

Bump anyhow from 1.0.94 to 1.0.95

- Refactor nflux.toml & implement egress monitoring for vpn virtual interfaces

- Support multiple interfaces

- Organice code

- Bump serde from 1.0.216 to 1.0.217

Bumps [serde](https://github.com/serde-rs/serde) from 1.0.216 to 1.0.217.
- [Release notes](https://github.com/serde-rs/serde/releases)
- [Commits](https://github.com/serde-rs/serde/compare/v1.0.216...v1.0.217)

---
updated-dependencies:
- dependency-name: serde
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #10 from containerscrew/dependabot/cargo/serde-1.0.217

Bump serde from 1.0.216 to 1.0.217

- First initial working version

- Edit log in xdp_firewall events

- Update order for TCP IpProto in firewall ebpf code

- Make podman compatible

- Bump tempfile from 3.14.0 to 3.15.0

Bumps [tempfile](https://github.com/Stebalien/tempfile) from 3.14.0 to 3.15.0.
- [Changelog](https://github.com/Stebalien/tempfile/blob/master/CHANGELOG.md)
- [Commits](https://github.com/Stebalien/tempfile/compare/v3.14.0...v3.15.0)

---
updated-dependencies:
- dependency-name: tempfile
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #11 from containerscrew/dependabot/cargo/tempfile-3.15.0

Bump tempfile from 3.14.0 to 3.15.0

- Bump clap from 4.5.23 to 4.5.24

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.23 to 4.5.24.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.23...clap_complete-v4.5.24)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #12 from containerscrew/dependabot/cargo/clap-4.5.24

Bump clap from 4.5.23 to 4.5.24

- Bump tokio from 1.42.0 to 1.43.0

Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.42.0 to 1.43.0.
- [Release notes](https://github.com/tokio-rs/tokio/releases)
- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.42.0...tokio-1.43.0)

---
updated-dependencies:
- dependency-name: tokio
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #13 from containerscrew/dependabot/cargo/tokio-1.43.0

Bump tokio from 1.42.0 to 1.43.0

- Working with egress log

- Bump clap from 4.5.24 to 4.5.26

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.24 to 4.5.26.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.24...clap_complete-v4.5.26)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #14 from containerscrew/dependabot/cargo/clap-4.5.26

Bump clap from 4.5.24 to 4.5.26

- Logging pid for every egress packet

- Working with egress connections

- Wip: egress connections

- Fix get_process_name func

- Bump log from 0.4.22 to 0.4.25

Bumps [log](https://github.com/rust-lang/log) from 0.4.22 to 0.4.25.
- [Release notes](https://github.com/rust-lang/log/releases)
- [Changelog](https://github.com/rust-lang/log/blob/master/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/log/compare/0.4.22...0.4.25)

---
updated-dependencies:
- dependency-name: log
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #15 from containerscrew/dependabot/cargo/log-0.4.25

Bump log from 0.4.22 to 0.4.25

- Stable: egress sniff for physical and virtual interfaces

- Delete unused files, log when ipv6 packets in egress filter

- Wip: egress connection

- Remove xtask for binary compilation

- Remove xtask folder

- Remove .cargo/ root folder

- Modify bpf_get_current_pid func

- Wip: docker builds

- Bump cargo_metadata from 0.18.1 to 0.19.1

Bumps [cargo_metadata](https://github.com/oli-obk/cargo_metadata) from 0.18.1 to 0.19.1.
- [Release notes](https://github.com/oli-obk/cargo_metadata/releases)
- [Changelog](https://github.com/oli-obk/cargo_metadata/blob/main/CHANGELOG.md)
- [Commits](https://github.com/oli-obk/cargo_metadata/compare/0.18.1...0.19.1)

---
updated-dependencies:
- dependency-name: cargo_metadata
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #16 from containerscrew/dependabot/cargo/cargo_metadata-0.19.1

Bump cargo_metadata from 0.18.1 to 0.19.1

- Remove pid tracking for traffic control connections & organize code

- Cargo fmt

- Improve egress/ingress traffic control

- Fix nflux/src/egress.rs

- Improve debian package

- Fix logger for src and dest ip

- Wip: implement prometheus metrics

- Wip: modify prometheus metrics counters

- Wip: prometheus metrics

- Bump tempfile from 3.15.0 to 3.16.0

Bumps [tempfile](https://github.com/Stebalien/tempfile) from 3.15.0 to 3.16.0.
- [Changelog](https://github.com/Stebalien/tempfile/blob/master/CHANGELOG.md)
- [Commits](https://github.com/Stebalien/tempfile/compare/v3.15.0...v3.16.0)

---
updated-dependencies:
- dependency-name: tempfile
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #17 from containerscrew/dependabot/cargo/tempfile-3.16.0

Bump tempfile from 3.15.0 to 3.16.0

- Reorganize code

- Bump bytes from 1.9.0 to 1.10.0

Bumps [bytes](https://github.com/tokio-rs/bytes) from 1.9.0 to 1.10.0.
- [Release notes](https://github.com/tokio-rs/bytes/releases)
- [Changelog](https://github.com/tokio-rs/bytes/blob/master/CHANGELOG.md)
- [Commits](https://github.com/tokio-rs/bytes/compare/v1.9.0...v1.10.0)

---
updated-dependencies:
- dependency-name: bytes
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #18 from containerscrew/dependabot/cargo/bytes-1.10.0

Bump bytes from 1.9.0 to 1.10.0

- Bump toml from 0.8.19 to 0.8.20

Bumps [toml](https://github.com/toml-rs/toml) from 0.8.19 to 0.8.20.
- [Commits](https://github.com/toml-rs/toml/compare/toml-v0.8.19...toml-v0.8.20)

---
updated-dependencies:
- dependency-name: toml
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #19 from containerscrew/dependabot/cargo/toml-0.8.20

Bump toml from 0.8.19 to 0.8.20

- Comment test pipeline

- Creating the cli with clap

- Refactoring ingress/egress with TC

- Change logger timestamp

- Pipeline docker build: only when tags

- Creating new config bpf_map for tc

- Add pid and command track to the log

- Disable udp logging by default

- Wip: implement non ethernet packets

- Implement wireguard iface tunnel monitoring

- Log every N seconds & change perfeventarray and implement ringbuffer

- Add .vscode folder

- Change todo

- Add ipgeoinfo

- Update deps

- Delete unused files/folder

- Delete unused Makefile targets

- Update todo.md

- Comment/delete unused code/files

- Bump clap from 4.5.28 to 4.5.29

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.28 to 4.5.29.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.28...clap_complete-v4.5.29)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #20 from containerscrew/dependabot/cargo/clap-4.5.29

Bump clap from 4.5.28 to 4.5.29

- Improve logger: track active connections storing pid:dest_ip

- Remove pid and command from tc

- Cargo clippy

- Bump clap from 4.5.29 to 4.5.30

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.29 to 4.5.30.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.29...clap_complete-v4.5.30)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #21 from containerscrew/dependabot/cargo/clap-4.5.30

Bump clap from 4.5.29 to 4.5.30

- Bump tempfile from 3.16.0 to 3.17.1

Bumps [tempfile](https://github.com/Stebalien/tempfile) from 3.16.0 to 3.17.1.
- [Changelog](https://github.com/Stebalien/tempfile/blob/master/CHANGELOG.md)
- [Commits](https://github.com/Stebalien/tempfile/compare/v3.16.0...v3.17.1)

---
updated-dependencies:
- dependency-name: tempfile
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #22 from containerscrew/dependabot/cargo/tempfile-3.17.1

Bump tempfile from 3.16.0 to 3.17.1

- Bump anyhow from 1.0.95 to 1.0.96

Bumps [anyhow](https://github.com/dtolnay/anyhow) from 1.0.95 to 1.0.96.
- [Release notes](https://github.com/dtolnay/anyhow/releases)
- [Commits](https://github.com/dtolnay/anyhow/compare/1.0.95...1.0.96)

---
updated-dependencies:
- dependency-name: anyhow
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #23 from containerscrew/dependabot/cargo/anyhow-1.0.96

Bump anyhow from 1.0.95 to 1.0.96

- Bump log from 0.4.25 to 0.4.26

Bumps [log](https://github.com/rust-lang/log) from 0.4.25 to 0.4.26.
- [Release notes](https://github.com/rust-lang/log/releases)
- [Changelog](https://github.com/rust-lang/log/blob/master/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/log/compare/0.4.25...0.4.26)

---
updated-dependencies:
- dependency-name: log
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #24 from containerscrew/dependabot/cargo/log-0.4.26

Bump log from 0.4.25 to 0.4.26

- Bump clap from 4.5.30 to 4.5.31

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.30 to 4.5.31.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/clap_complete-v4.5.30...v4.5.31)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #25 from containerscrew/dependabot/cargo/clap-4.5.31

Bump clap from 4.5.30 to 4.5.31

- Bump libc from 0.2.169 to 0.2.170

Bumps [libc](https://github.com/rust-lang/libc) from 0.2.169 to 0.2.170.
- [Release notes](https://github.com/rust-lang/libc/releases)
- [Changelog](https://github.com/rust-lang/libc/blob/0.2.170/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/libc/compare/0.2.169...0.2.170)

---
updated-dependencies:
- dependency-name: libc
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #26 from containerscrew/dependabot/cargo/libc-0.2.170

Bump libc from 0.2.169 to 0.2.170

- Bump chrono from 0.4.39 to 0.4.40

Bumps [chrono](https://github.com/chronotope/chrono) from 0.4.39 to 0.4.40.
- [Release notes](https://github.com/chronotope/chrono/releases)
- [Changelog](https://github.com/chronotope/chrono/blob/main/CHANGELOG.md)
- [Commits](https://github.com/chronotope/chrono/compare/v0.4.39...v0.4.40)

---
updated-dependencies:
- dependency-name: chrono
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #27 from containerscrew/dependabot/cargo/chrono-0.4.40

Bump chrono from 0.4.39 to 0.4.40

- Adding ttl, total_len only for tcp packets

- Comment some functions

- Wip: cli

- Wip: README and & cargo fmt

- Wip: refactoring handlers

- Setting default iface if not set

- Wip: modify log

- Bump anyhow from 1.0.96 to 1.0.97

Bumps [anyhow](https://github.com/dtolnay/anyhow) from 1.0.96 to 1.0.97.
- [Release notes](https://github.com/dtolnay/anyhow/releases)
- [Commits](https://github.com/dtolnay/anyhow/compare/1.0.96...1.0.97)

---
updated-dependencies:
- dependency-name: anyhow
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #28 from containerscrew/dependabot/cargo/anyhow-1.0.97

Bump anyhow from 1.0.96 to 1.0.97

- Bump cargo_metadata from 0.19.1 to 0.19.2

Bumps [cargo_metadata](https://github.com/oli-obk/cargo_metadata) from 0.19.1 to 0.19.2.
- [Release notes](https://github.com/oli-obk/cargo_metadata/releases)
- [Changelog](https://github.com/oli-obk/cargo_metadata/blob/main/CHANGELOG.md)
- [Commits](https://github.com/oli-obk/cargo_metadata/compare/0.19.1...0.19.2)

---
updated-dependencies:
- dependency-name: cargo_metadata
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #29 from containerscrew/dependabot/cargo/cargo_metadata-0.19.2

Bump cargo_metadata from 0.19.1 to 0.19.2

- Bump bytes from 1.10.0 to 1.10.1

Bumps [bytes](https://github.com/tokio-rs/bytes) from 1.10.0 to 1.10.1.
- [Release notes](https://github.com/tokio-rs/bytes/releases)
- [Changelog](https://github.com/tokio-rs/bytes/blob/master/CHANGELOG.md)
- [Commits](https://github.com/tokio-rs/bytes/compare/v1.10.0...v1.10.1)

---
updated-dependencies:
- dependency-name: bytes
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #30 from containerscrew/dependabot/cargo/bytes-1.10.1

Bump bytes from 1.10.0 to 1.10.1

- Bump tempfile from 3.17.1 to 3.18.0

Bumps [tempfile](https://github.com/Stebalien/tempfile) from 3.17.1 to 3.18.0.
- [Changelog](https://github.com/Stebalien/tempfile/blob/master/CHANGELOG.md)
- [Commits](https://github.com/Stebalien/tempfile/compare/v3.17.1...v3.18.0)

---
updated-dependencies:
- dependency-name: tempfile
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #31 from containerscrew/dependabot/cargo/tempfile-3.18.0

Bump tempfile from 3.17.1 to 3.18.0

- Bump ring from 0.17.8 to 0.17.13

Bumps [ring](https://github.com/briansmith/ring) from 0.17.8 to 0.17.13.
- [Changelog](https://github.com/briansmith/ring/blob/main/RELEASES.md)
- [Commits](https://github.com/briansmith/ring/commits)

---
updated-dependencies:
- dependency-name: ring
  dependency-type: indirect
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #32 from containerscrew/dependabot/cargo/ring-0.17.13

Bump ring from 0.17.8 to 0.17.13

- Bump tokio from 1.43.0 to 1.44.0

Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.43.0 to 1.44.0.
- [Release notes](https://github.com/tokio-rs/tokio/releases)
- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.43.0...tokio-1.44.0)

---
updated-dependencies:
- dependency-name: tokio
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #33 from containerscrew/dependabot/cargo/tokio-1.44.0

Bump tokio from 1.43.0 to 1.44.0

- Bump clap from 4.5.31 to 4.5.32

Bumps [clap](https://github.com/clap-rs/clap) from 4.5.31 to 4.5.32.
- [Release notes](https://github.com/clap-rs/clap/releases)
- [Changelog](https://github.com/clap-rs/clap/blob/master/CHANGELOG.md)
- [Commits](https://github.com/clap-rs/clap/compare/v4.5.31...clap_complete-v4.5.32)

---
updated-dependencies:
- dependency-name: clap
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #34 from containerscrew/dependabot/cargo/clap-4.5.32

Bump clap from 4.5.31 to 4.5.32

- Bump reqwest from 0.12.12 to 0.12.13

Bumps [reqwest](https://github.com/seanmonstar/reqwest) from 0.12.12 to 0.12.13.
- [Release notes](https://github.com/seanmonstar/reqwest/releases)
- [Changelog](https://github.com/seanmonstar/reqwest/blob/master/CHANGELOG.md)
- [Commits](https://github.com/seanmonstar/reqwest/compare/v0.12.12...v0.12.13)

---
updated-dependencies:
- dependency-name: reqwest
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #35 from containerscrew/dependabot/cargo/reqwest-0.12.13

Bump reqwest from 0.12.12 to 0.12.13

- Bump libc from 0.2.170 to 0.2.171

Bumps [libc](https://github.com/rust-lang/libc) from 0.2.170 to 0.2.171.
- [Release notes](https://github.com/rust-lang/libc/releases)
- [Changelog](https://github.com/rust-lang/libc/blob/0.2.171/CHANGELOG.md)
- [Commits](https://github.com/rust-lang/libc/compare/0.2.170...0.2.171)

---
updated-dependencies:
- dependency-name: libc
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #36 from containerscrew/dependabot/cargo/libc-0.2.171

Bump libc from 0.2.170 to 0.2.171

- Bump reqwest from 0.12.13 to 0.12.14

Bumps [reqwest](https://github.com/seanmonstar/reqwest) from 0.12.13 to 0.12.14.
- [Release notes](https://github.com/seanmonstar/reqwest/releases)
- [Changelog](https://github.com/seanmonstar/reqwest/blob/v0.12.14/CHANGELOG.md)
- [Commits](https://github.com/seanmonstar/reqwest/compare/v0.12.13...v0.12.14)

---
updated-dependencies:
- dependency-name: reqwest
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #37 from containerscrew/dependabot/cargo/reqwest-0.12.14

Bump reqwest from 0.12.13 to 0.12.14

- Bump tokio from 1.44.0 to 1.44.1

Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.44.0 to 1.44.1.
- [Release notes](https://github.com/tokio-rs/tokio/releases)
- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.44.0...tokio-1.44.1)

---
updated-dependencies:
- dependency-name: tokio
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #38 from containerscrew/dependabot/cargo/tokio-1.44.1

Bump tokio from 1.44.0 to 1.44.1

- Bump tempfile from 3.18.0 to 3.19.0

Bumps [tempfile](https://github.com/Stebalien/tempfile) from 3.18.0 to 3.19.0.
- [Changelog](https://github.com/Stebalien/tempfile/blob/master/CHANGELOG.md)
- [Commits](https://github.com/Stebalien/tempfile/compare/v3.18.0...v3.19.0)

---
updated-dependencies:
- dependency-name: tempfile
  dependency-type: direct:production
  update-type: version-update:semver-minor
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #39 from containerscrew/dependabot/cargo/tempfile-3.19.0

Bump tempfile from 3.18.0 to 3.19.0

- Bump reqwest from 0.12.14 to 0.12.15

Bumps [reqwest](https://github.com/seanmonstar/reqwest) from 0.12.14 to 0.12.15.
- [Release notes](https://github.com/seanmonstar/reqwest/releases)
- [Changelog](https://github.com/seanmonstar/reqwest/blob/master/CHANGELOG.md)
- [Commits](https://github.com/seanmonstar/reqwest/compare/v0.12.14...v0.12.15)

---
updated-dependencies:
- dependency-name: reqwest
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #40 from containerscrew/dependabot/cargo/reqwest-0.12.15

Bump reqwest from 0.12.14 to 0.12.15

- Fix cli: help usage

- Add fedora.yml for mac osx lima

- Adding more log fields

- Doc & move func to utils.rs

- Updatge todo_and_features.md

- Remove multiple interface program attachment

- Bump tempfile from 3.19.0 to 3.19.1

Bumps [tempfile](https://github.com/Stebalien/tempfile) from 3.19.0 to 3.19.1.
- [Changelog](https://github.com/Stebalien/tempfile/blob/master/CHANGELOG.md)
- [Commits](https://github.com/Stebalien/tempfile/compare/v3.19.0...v3.19.1)

---
updated-dependencies:
- dependency-name: tempfile
  dependency-type: direct:production
  update-type: version-update:semver-patch
...

Signed-off-by: dependabot[bot] <support@github.com>

- Merge pull request #41 from containerscrew/dependabot/cargo/tempfile-3.19.1

Bump tempfile from 3.19.0 to 3.19.1

- Change test pipeline

- Fix tests

- Fix cargo clippy

- Cargo update

- Remove prometheus dependencies

- Delete old pacman PKGDBUILD

- Add old doc

- Edit user space logger

- Update doc and build pipeline

- Massive update

- Fmt and clippy

- Updates

- Modify Dockerfile

- Delete compose.yml

- Restructure cli code

- Implementing netrace subcommand

- Massive refactor

- Rename ebpf code to netrace-ebpf

- Add build pipeline

- Wip: build pipeline

- Adding ebpf code for tlstrace

- Update build.yml pipeline

- Wip: tlstrace command

- Implement tlstrace

- Update README.md and cli.rs

- Updating doc

- Improve TC log event

- Add git changelog

- Update netrace logger

- Providing installation methods

- Pipeline updates

- Wip: build pipelines

- Update badges

- Wip: release and build pipeline

- Release pipeline

- Fix zip packagein Makefile

- Strip binary

<!-- generated by git-cliff -->
