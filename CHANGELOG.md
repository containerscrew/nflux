#### Miscellaneous Chores
- 2ff6c05a2981617a0ab1368711409b260a191784 - **(version)** 0.12.6 - containerscrew
- 4262ce2ede165701ddb4e72852936272bbc93da6 - add CHANGELOG.md - containerscrew

- 5effb1b2a2e748c169eefc3f198a318741dd807f - delete changelog - containerscrew

- 87a72d675dbf060d81c90fb6b8ed98ab6dfa081d - update CHANGELOG.md - containerscrew

#### Refactoring
- 590cc4c50bd683915466bbe7d9623e574a301d43 - formatting & fix aya helpers - containerscrew


- - -

#### Bug Fixes
- e7fd1018839d8f700254ea94826e86bb55ade8de - **(cli)** help message in --log-inverval flag - containerscrew
- 0ed5577d34b7788fb76179004bbecaa16c969a18 - **(cli)** merge conflix cli.rs - containerscrew
- 2ffa6a50775df6d42e696f411af17c571ca50991 - **(doc)** typo in usage.md - containerscrew
- 7324f74399dd11300459665f15b3211392f514ba - **(install.sh)** relocate install.sh and fix script - containerscrew
- 6581a37d9620f7bf929db04e7794250bb14d8277 - **(install.sh)** shebang and welcome message - containerscrew
- cae7557d157de21181ac17bfd0470f9a0e90dc68 - **(test)** is_root_user - containerscrew
- 3ff9f5e55a36b29a24fb052b4b65e637be40ada7 - release pipeline prefix - containerscrew

- 18b966ea9bcbfb96b2c9848b0eead75c7ffe05ea - sudo permissions in test pipeline - containerscrew

- b9ac4ab25e28020c8a533badd63ab4470afee55f - protobuf-compiler dependency in test.yml github workflow - containerscrew

- 67197baf5e1bddf27819d6caec6d27ab4da4e7bb - protobuf-compiler dependency in github ci - containerscrew

- 7feffb64d0742c747d977fcc4ab47f4b42daba8b - protobuf-compiler dependency in github ci - containerscrew

- 686965a8260625bf2077fa29df75c5d04aeaef6e - convert_protocol helper function - containerscrew

- 6705f37ab1871705a15abf96e9201acd8420dc31 - cargo deb metadata - containerscrew

- c031565453033bbbe31f5e2c7f5e74118e8064f9 - process_tc_events json logger - containerscrew

- 8615bd15e7f75dc099af69419fda5cf3d6dea2cd - test cli::test::test_print_help_message - containerscrew

- 741b902663e9f75a328061b3007b1e037244fc5f - test & update README.md - containerscrew

- 1f5d86703ce481d256a7dc43dbccd8e249c1a711 - cargo deb metadata - containerscrew

- 6b089ed650ec20d1be986bb6ce676bad5a5bf6c8 - release pipeline - containerscrew

- db5fc60447bd96e5a96279abc09f68a0477de14a - release pipeline - containerscrew

- 002d0323e7164b1eb0773f777c72914390a74bca - test to_ip_addr - containerscrew

- d9189a3f0739da084243e170836d1ef5de796f20 - split ebpf code to avoid errors in stack size - containerscrew

- a59c64490126fee5375d153070697b49e1122b43 - nostd alloc for impl Display in TcpFlags struct - containerscrew

#### Documentation
- afbfa5dd6abb92ea13e65c001f0fe9380c62c725 - update project_notes.md - containerscrew

- a4deed90f8f5b6c9fe24799c76e90e82dcc583d7 - new file project_notes.md - containerscrew

#### Features
- e07de1aa33b7848a59d1499d96cb1298b8b9a317 - **(doc)** add new examples section - containerscrew
- 3a6d49cb78812aa67f2498e9de12db68acd973d2 - **(exclude-ports)** new feature flag --exlude-ports from the logger) - containerscrew
- ee4b8b95d7b9ed6328c05abb4c68277ce130fd11 - **(filter-ports)** implement cli flag --filter-ports - containerscrew
- 3dfb5a7a424b09f82fcc6a6937ecad7e68edd53b - **(listen-port)** implement flag --listen-port - containerscrew
- cd2b2ac7eab93d1eb0e73343b171d22a81e2d3ee - **(listen-ports)** filtering listen-ports in ebpf code - containerscrew
- ecc0aef65a47d017c0cdf9ab32f031e93e76d23d - **(logger)** change logger pkt_len field - containerscrew
- 02d445d246750f379309596cf4c409055bb6a3ff - **(logger)** change logger pkt_len field - containerscrew
- c89d1d25484f449dfedb066803b872239061f9e7 - **(tcp_flags)** implement tcp flags & remove pid and command track for egress connections - containerscrew
- 26f0ccd3b84f0404609442a18509dc285e03105a - implementing containerd cgroup support - containerscrew

- 057ad2afe0ac02264ca6ff99230c11a6d646ddd3 - implementing containerd cgroup support - containerscrew

- a1da67ff07465052662c69e2f2ce89170095ee64 - implementing containerd cgroup support - containerscrew

- e0c0bef02ca114bc2ac0f222cb45b57cbc825434 - adding ebpf-cgroups for container networking - containerscrew

- 9beb4022159824a58c48320821832832b0e95ff1 - wip: adding support for cgroup sniffing - containerscrew

- 1f327511062190a0078a64c2a37f7d4e38a9f4e9 - new version v0.12.4 - containerscrew

- 9e712bc463880ac5bcbc75b9ec74aef351dafcb7 - add verbosity for unknown procotols - containerscrew

- 5d1deb2aba0930e4aa93cc09ca751feacdf34821 - refactor to eliminate high stack allocation, reducing verifier stack usage - containerscrew

- 12ba61a02c2cf55e19fafcdbfbfc3ac14eeb3214 - nflux subcommands & file and code refactor - containerscrew

- 4d4f19e2fda451da34f56ebf4b619f48fea72c9c - implement new feature to detect dropped packets - containerscrew

#### Miscellaneous Chores
- d7fb2d14323a233492dc9b11cd58e9659f3a550a - **(deps)** bump tracing-subscriber from 0.3.19 to 0.3.20 - dependabot[bot]
- 85234095a9ba76ab9d041d81d8de29a551716c04 - **(deps)** bump clap from 4.5.45 to 4.5.46 - dependabot[bot]
- 13b0402be46c5edfe4558678bcf7026a96493ec9 - **(deps)** bump dns-lookup from 2.1.0 to 3.0.0 - dependabot[bot]
- 3681dabf66cf8443f4b9a146377fa4c0cf039009 - **(deps)** bump network-types from 0.0.8 to 0.1.0 - dependabot[bot]
- 9899708671dbf01750b540b4ffe6fd72d99f9efb - **(deps)** bump async-trait from 0.1.88 to 0.1.89 - dependabot[bot]
- 9d3061c1b3321c03590dd927fd4e4ec19b069f3c - **(deps)** bump anyhow from 1.0.98 to 1.0.99 - dependabot[bot]
- d2b607d1bc42411b7f410ace44f99adae95b12f1 - **(deps)** bump clap from 4.5.44 to 4.5.45 - dependabot[bot]
- 46bd9a44bfe7a8e211b1473ed4017230c47b80b9 - **(deps)** bump clap from 4.5.43 to 4.5.44 - dependabot[bot]
- 4757a49eac2d95432160e9d49a995bb80b83403a - **(deps)** bump slab from 0.4.10 to 0.4.11 - dependabot[bot]
- f5d62ab55cb2fa29043ce56d1bdfe6313d1344e0 - **(deps)** bump libc from 0.2.174 to 0.2.175 - dependabot[bot]
- f0c7ab04648c7853f5e1acfaede19520f9fdb652 - **(deps)** bump sysinfo from 0.36.1 to 0.37.0 - dependabot[bot]
- f6ac5741fed92737efe916e7c2db8529468ee83f - **(deps)** bump clap from 4.5.42 to 4.5.43 - dependabot[bot]
- 7ca368acbf58f93ef61dd830810811d3f5bf34fd - **(deps)** bump dns-lookup from 2.0.4 to 2.1.0 - dependabot[bot]
- 090f4617332276588209b013494378a212f37230 - **(deps)** bump tokio from 1.47.0 to 1.47.1 - dependabot[bot]
- f0a64368b1ec7843b0c519d3df71a036747da6c2 - **(deps)** bump clap from 4.5.41 to 4.5.42 - dependabot[bot]
- eb1be5de3f91dd9f3fbb40e2afb799b95a03d6cd - **(deps)** bump tokio from 1.46.1 to 1.47.0 - dependabot[bot]
- 6f1c132ab55ecabf818395dd4edb578035c7468f - **(deps)** bump sysinfo from 0.36.0 to 0.36.1 - dependabot[bot]
- 3a0ab2a6e1ccf4d131deea30d1613734a3679074 - **(version)** 0.12.5 - containerscrew
- ce3263276e593e93361057b9c18e720647367b13 - **(version)** 0.12.5 - containerscrew
- 1887670c7488b6480a95632067e42130f3c35acf - **(version)** 0.12.5 - containerscrew
- a1b549626759c7fb999358731c66704bc369c5e9 - **(version)** 0.12.5 - containerscrew
- 6d1c83f1af492cefb6efd62e54fc6535dbba4f22 - **(version)** 0.12.5 - containerscrew
- 5b8f2ce55be3ab81257533fde6dd7abe35a297d1 - delete changelog - containerscrew

- 469c1bb59392811d8ccc15a7003813a8ff83e55f - update CHANGELOG.md - containerscrew

- 1449af8f44a083e5050d6ef31df1935397b0c17e - add CHANGELOG.md - containerscrew

- b5c0946def4617b9ebf65eeb6a32672e5d32a6cf - delete changelog - containerscrew

- a75dd55963dad41e5fe0f9aee84a64e9e1386eff - add new script for git history rewrite - containerscrew

- 1a644aafd60e99544040ca915eccab9ef4eb16ee - remove cgroups feature - containerscrew

- 29a054da96650a480cba6a62f452022351119643 - remove cgroups feature - containerscrew

- 6cb738971c8b1e5360c19014d2564a3d9a066f24 - remove cgroups feature - containerscrew

- ff5e7a1312c17ba58fac1267aa6ecf1bbe1ce8f8 - remove cgroups feature - containerscrew

- 259fdb2dcdcb68213d018f6b4a0bf47e76f11dc9 - Merge pull request #84 from containerscrew/dependabot/cargo/tracing-subscriber-0.3.20 - github-actions[bot]

- a9d6f60f8c23f84269f6077aaf6847b407c21d62 - Merge pull request #83 from containerscrew/dependabot/cargo/clap-4.5.46 - github-actions[bot]

- 2849db32863e37666727e16aa4f35d92139ef385 - Merge pull request #82 from containerscrew/dependabot/cargo/dns-lookup-3.0.0 - github-actions[bot]

- 6e23152ad3c0bcd1f20d069b0ee62e47f4f681f7 - Merge pull request #81 from containerscrew/dependabot/cargo/network-types-0.1.0 - github-actions[bot]

- 33850f6e82bb407e5a0aab09da40c77a4ed36997 - Merge pull request #80 from containerscrew/dependabot/cargo/async-trait-0.1.89 - github-actions[bot]

- e0012441bbcd40a22d6e2f67000133f2c11ca854 - Merge pull request #79 from containerscrew/dependabot/cargo/anyhow-1.0.99 - github-actions[bot]

- 80c6eede425663b78952b66f0d6646676a97948c - Merge pull request #78 from containerscrew/dependabot/cargo/clap-4.5.45 - github-actions[bot]

- 6adb86ae3f40a2951d101a9f038741e6e53371a4 - Merge pull request #76 from containerscrew/dependabot/cargo/clap-4.5.44 - github-actions[bot]

- 58d4084dce18b6f91e8e8a5efaee0b2cef8b5e10 - Merge pull request #77 from containerscrew/dependabot/cargo/slab-0.4.11 - github-actions[bot]

- 2d3fa8b3e392493c62491cca41a4ca72cd9c5f5d - Merge pull request #75 from containerscrew/dependabot/cargo/libc-0.2.175 - github-actions[bot]

- f92f8ed5bb500e5ae8dbe2b92492ab7e665f87dc - Merge pull request #74 from containerscrew/dependabot/cargo/sysinfo-0.37.0 - github-actions[bot]

- af9eaacb40242ce5fccedf8923013879f6611321 - Merge pull request #73 from containerscrew/dependabot/cargo/clap-4.5.43 - github-actions[bot]

- 27bf5ac158d3fd8fa56c9bdd4975edea57902ba2 - Merge pull request #72 from containerscrew/dependabot/cargo/dns-lookup-2.1.0 - github-actions[bot]

- fda7482346e3681652fc5b3c0035c45e0dc2fa58 - Merge pull request #71 from containerscrew/dependabot/cargo/tokio-1.47.1 - github-actions[bot]

- 9d160ae8a06007eb10b3bea38754df32bb992494 - Merge pull request #70 from containerscrew/dependabot/cargo/clap-4.5.42 - github-actions[bot]

- f62c4eda1e3d442651357c5e3ee9757f432d7ecc - Merge pull request #69 from containerscrew/dependabot/cargo/tokio-1.47.0 - github-actions[bot]

- c78b76f76ca78b85e5f43961c237ca37f39f32b1 - Merge pull request #68 from containerscrew/dependabot/cargo/sysinfo-0.36.1 - github-actions[bot]

- d6b07c708483372a960aafb9335b0e5647d2840c - updating containerd implementation - containerscrew

- e6eabd4889ea7acab4323bd03733836b5694161f - update cog.toml - containerscrew

- 41d0f5bdfdab01c7fd37721cfc6a15d683384779 - Merge pull request #67 from containerscrew/dependabot/cargo/sysinfo-0.36.0 - github-actions[bot]

- 419c47c9977b0f7d14ca09656d1efc6b160b61b8 - Bump sysinfo from 0.35.2 to 0.36.0 - dependabot[bot]

- 299f4cae4e118733beaceb1819638f444fbca905 - Merge pull request #66 from containerscrew/dependabot/cargo/clap-4.5.41 - github-actions[bot]

- dbc1c1fbeb120a69b29341d66f47488c6f850d27 - Bump clap from 4.5.40 to 4.5.41 - dependabot[bot]

- a31e3a6144d196939800aa45ec3e05d8f0719156 - update readme - containerscrew

- 20f0d3fdd507d176d243c03f4d17849ced26c51b - rollback convert_protocol helper - containerscrew

- ea848de73be5e9d09618827517b159ae60fb5f10 - adding more information for unknown protocol - containerscrew

- ac44d40290bc6015e275af8865225065d8663ac0 - update README - containerscrew

- 0b6b88f9b277f19dd59c05979eee2ab43be3c104 - update Makefile - containerscrew

- 998b7a75b949b8ab53fd8e375ba7beef915552bd - update changelog for v0.12.4 - containerscrew

- c292ba714db31eaa37ff3189501dce9d5a45f224 - new v0.12.4 - containerscrew

- 3a21e42db67d30f8f1238ad0cfa1108944a46bc3 - update Makefile - containerscrew

- 97c4242382cdc582d365086fca06dd975f2e5052 - Generate CHANGELOG.md - containerscrew

- 0c27332158e0cfc290a124a465f82d37a7e90a0f - update README.md - containerscrew

- cfabf91b0fc4d1ec88b009741e5e19cdfd3a3bec - update README.md - containerscrew

- 5b3ed3fe2558a4920cd791b728f018c40193b8c4 - update README.md - containerscrew

- e91b67de4616f39db2d706d12ebb5a8222e464ad - update README.md - containerscrew

- 779a9f09cd63b1c9adf2ae492c37a34fde8d4342 - add cargo deb revision - containerscrew

- 8039bbceb29b96fb724d435e66284ea1e38e4c73 - release(v0.12.3): fix stack size allocation - containerscrew

- cd043373cd45345c7b1fa2ed9293a58afaba244e - Merge branch 'feat/reduce-stack-size' - containerscrew

- 54554af92f3530724ab6e1ae1bd90b2d63b8324e - remove comments and unnecessary dependencies - containerscrew

- 1560fb5b7e17af74ec417e7c5a28b96133cedffc - ebpf: refactor to eliminate TcEvent stack allocation, reducing verifier stack usage - containerscrew

- 8001b4076fdf62e14b1c9de24cdbb70620e62d64 - ebpf: refactor to eliminate tc_config stack allocation, reducing verifier stack usage - containerscrew

- ae48125f83ef5c131a4a69da9c483d64c6ab5cf5 - remove comments and unnecessary dependencies - containerscrew

- 9766d296a3f87b743b42a604610bf6b58d085f77 - ebpf: refactor to eliminate TcEvent stack allocation, reducing verifier stack usage - containerscrew

- 4a7fb77d0872f97f67be25639434393bc9056c0f - remove unnecessary dependencies - containerscrew

- c32529d6d7d66375e58f0c40f3a635fd4225e8e5 - rollback: full tcpflags - containerscrew

- 568b3883eab819240dd3cb5f49085e2393f02c35 - update changelog for v0.12.2 - containerscrew

- 1b393df7c11418a839f1cb28aa3fa08230e59f8f - Add changelog - containerscrew

- 254c7726ca87d1e56ccc8f4323facedc301f9275 - update changelog for v0.12.2 - containerscrew

- 273839673084c538d14c957dfcd320df10cf7efc - Create changelog - containerscrew

- 019f105ff5d3ab0d4760259fb98966f5c20b72b4 - update changelog for v0.12.2 - containerscrew

- 3413625de84a43846dec067c7dd7c26721c003d1 - Fix Makefile - containerscrew

- d258d80665eab067e31dcb3d3c66451a55fd7c77 - update changelog for v0.12.2 - containerscrew

- 7232d060a1aaea2545d442ccbcf473d85f6dd316 - Update Makefile - containerscrew

- 4f17977a0fdabb71d0dfc487dcec673aa27138a6 - Comment mtoc hook - containerscrew

- 7e3ae99ddf1abd07c7b513cd88e4557ddcd667e9 - Temporary comment some Tcp flags to avoid stack limit size - containerscrew

- ef5dd39047247d3a71aa424161b8d31282455260 - Fix package deb metadata - containerscrew

- 3a832f73524b6690ce1fda056d4fba7229ef2aac - Update Makefile - containerscrew

- 456c934a6f6b5324b1909320c33a133f073cb938 - Change licensing - containerscrew

- 203c0a21aa72f6c82c033b7a55c7f523d0122fc1 - Add cargo deb in release pipeline - containerscrew

- e7256759197f851fc40ef7dedae62b21a2c01534 - Update handle TcpFlags - containerscrew

- 44c8528bafba688fe9d3bc164f7151691cce4d2c - New version v0.12.1 including .deb package - containerscrew

- 38a055e1e9c5a35d72dcbce83d03fa2ab5d18d77 - Update cargo deb metadata - containerscrew

- 4feaf132b53f791395096c319dc81454048ca8ce - Update build.yml pipeline - containerscrew

- 2bc5f09b57a1d4384747a68cf8071ec990191eaf - Update CHANGELOG.md - containerscrew

- 83dcfcfa502e2e3fe7c8c88bb06032d6b6bc0c9d - Update CHANGELOG.md - containerscrew

- b5d6bb1e0155682f66f07418abb53365eb3e6dd3 - Update CHANGELOG.md for release v0.12.0 - containerscrew

- bb672306f50e04778aaf15e5ad8a96d716c497db - Update CHANGELOG.md for release v0.12.0 - containerscrew

- 1c8918571bb115cacf68760ffc3f535be7e25602 - Update CHANGELOG.md - containerscrew

- a391ea632051fe378e71ce7031f140dfe6aa3ea5 - Update CHANGELOG.md - containerscrew

- 43ca411b4b3dc383730e5f10fcb496428a95db5b - Update release pipeline - containerscrew

- 4170b90f001cb5b6d5bcf048881f23df03b39e1c - Update CHANGELOG.md - containerscrew

- cefc25d49f7caa656a22bd7c2af01277750bc6d4 - Update CHANGELOG.md - containerscrew

- f94f99595bb807e1020a07a6d7efa8468a69fc55 - Update CHANGELOG.md - containerscrew

- e5994b925157b40dd955e1fe5d6b3460f5d5e9c9 - Update pre-push git hook - containerscrew

- aeb82a08f9b3bbcb0f135e5b0b2463052be0ba62 - Update CHANGELOG.md - containerscrew

- cabea27de8168116e655513cd824a16642a2ac34 - Update CHANGELOG.md - containerscrew

- 3ce4cd543d1032a61aadb0857458109dfe3e4c89 - Update CHANGELOG.md - containerscrew

- 4f02ff35c5d3ad63beeeee23212ae6121b972207 - Update git hook pre-push - containerscrew

- 45bf75f760bb4b9761c9ae3a980db7f49fbe6700 - Update CHANGELOG.md - containerscrew

- a441a167a90ee91abdfbd2941b029a6f1a779c78 - Update CHANGELOG.md - containerscrew

- fdfcf19bddeb08f7cdabbf8f9a2a46178f4283e1 - Update git hooks - containerscrew

- 661a7e9a0b8e0acee2dbd33655726f5cea73a4a8 - Update release pipeline - containerscrew

- ee73d3c2d3d161791d8ab958fc86d2709582801c - Update CHANGELOG.md for release v0.11.0 - containerscrew

- 13856c38891ae9e4ed79540379a2ca4aee0f29de - new nflux version v0.11.0 - containerscrew

- ad98f201a1ff810c02e060233a47fb8bdad5cb12 - Update CHANGELOG.md for release v0.11.0 - containerscrew

- 4016ae6d9d9911977ce0bd87bbb5cc1d43f74888 - Update README.md - containerscrew

- 267d528530be23c63fb57a329a789f4f9f5d29fb - Update pre-commit - containerscrew

- d79ffea36642cc97d9ba56cbd4dd9524600f8247 - Rename example2 - containerscrew

- 59b02b57c0347f3ef59e45eb33300e769727284a - Update examples - containerscrew

- 6ab6f3ae95f8a64cf2b7bc27c120016375a16a79 - add more tcp flags & IpFamily impl - containerscrew

- 5c202190d58ecf3f639c10640c8edf157b1e2921 - Update dependency - containerscrew

- f56801a05eba06b56ec992402ba7acb09071ae3e - Remove tokio thread in function process_tc_events - containerscrew

- b1cc90abd7cb8c375b05feeac376b296f7c3ea4a - Change subcommands - containerscrew

- a8c4069ecc552f596fca35687227949da57e254b - Merge pull request #65 from containerscrew/dependabot/cargo/tokio-1.46.1 - github-actions[bot]

- 31d060a7b8b319220ebdb8f8109bd9d5c716c85c - Bump tokio from 1.46.0 to 1.46.1 - dependabot[bot]

- 0b57b73a15ac08579510d5129ad6e5038f4cef86 - Merge pull request #64 from containerscrew/dependabot/cargo/tokio-1.46.0 - github-actions[bot]

- 2148abd32638b871698955084e51fef387d6f747 - Bump tokio from 1.45.1 to 1.46.0 - dependabot[bot]

- d5768968ac6a510d008da7b2e23799a74311ba14 - Update CHANGELOG.md for release v0.10.0 - containerscrew

- bc8a78466bbeb659e98d5dcf44b9409bff6509df - New release v0.10.0 - containerscrew

- 233d7013dc847dc354d8e9c8e74afb04efac550b - Update CHANGELOG.md for release v0.10.0 - containerscrew

- 20492c6b18386ee2153925aaa18ec67bc5036d8d - Add vmlinux.rs & remove unused imports - containerscrew

- eaf1614fb44a18c909c7cf73cfaaf2ba768ad9c1 - Fix SkFamily data in dropped-pkt - containerscrew

- dfee4e9192bb049e10e253e16c484b27475f69fc - Update Makefile - containerscrew

- 7842476b4a6e1a8e4361be80679101f2a729b644 - Adding more socket information in dropped-pkt - containerscrew

- a2fae6a4ace682deea45f59892a885377939d8b5 - Update README.md - containerscrew

- 890e853b0a7e7be1a332afbad7b01e20627b9711 - Update README.md - containerscrew

- 0fffa4cb7e300fd51eb7db11045cf540d03b565c - Setup monitoring stack - containerscrew

- c983ddd287034c33c0baefca480751e9fa885656 - Fix loki config - containerscrew

- 598df35ba023838ed1ba59302f8bf0fc9efe4057 - Update CHANGELOG.md for release v0.9.0 - containerscrew

- a677b9fb496205af33285e0741e2281e4f2a2aa9 - New infra/ folder to store compose and docker files - containerscrew

- a3e1600ed1e7f939adce464e949fe3c711023cc8 - Revert "Add full example" - containerscrew

- 847652d7ab1a492b6ab5e81a1a60a9c3955448e5 - Add full example - containerscrew

- 170367092d4b9e40455bb7780e4f1bbe863eebf5 - update examples - containerscrew

- 57f45a770de46f38416bd3511fec054c5d901ab7 - json logging for dropped-pkt & skip unknown reasons - containerscrew

- f3b3636ae03ecefb9481e47852e29674a590a798 - Refactor: cli subcommands & file structure - containerscrew

- f428d90cc9f726ed83afc5c1b5d14fcc1dd6d5f0 - Fix event struct in nflux-ebpf - containerscrew

- 3f8801fe0ed61a08e12cc99d9ae0b1fe83a5c821 - Wip: dropped packets feature - containerscrew

- 2be79dbe010815851c95268658191c85cccec331 - Add new example - containerscrew

- 4c4e9cfa18eac5707eaaf1a93e68cc2846375533 - Update README.md - containerscrew

- 0a16389f5082e66d9857673b269180b8c47bdc1d - update doc and examples - containerscrew

- eccfedf4280537ffc22e4ed4414bea7513ce7f61 - Change imports rustfmt - containerscrew

- a4ebde0a0d920e8a3365b26a3131e05adb7588cc - Remove docs/ folder in favour of Github Wiki - containerscrew

- 2451f61dc67ef98434da3159ffa66b944fb739ea - update code comments in nflux-ebpf - containerscrew

- 8811e9f31599ae9328f7a0e186011cd47e16e1eb - Update CHANGELOG.md for release v0.8.0 - containerscrew

- 36d6d70f8d93ccbb72c1ac8156dda9b93cbedd5a - Update Cargo.toml version - containerscrew

- bc1984bebf9e1ea7396d4f65bfe6e066707eebad - Update usage.md - containerscrew

- e14b67ddff7c5bc9c4147f521e6805f4c05aba34 - Update usage.md - containerscrew

- 9509e394bee486b0144e6db890503894437a1244 - Update rustfmt.toml - containerscrew

- fb320802f4e7fcdeb733c8cbdbf6925fe7f06746 - Change fn_params_layout carfo fmt to Vertical - containerscrew

- a938fa41bcaa26d532224d086c08a7f377444e70 - Update example images - containerscrew

- 8b9cd5d1853ec779f58a415245cac681ac9eebb9 - Update CHANGELOG.md for release v0.7.0 - containerscrew

- 645891e07ac4ceaf95e380077f440371d83b147b - Update Cargo.toml version - containerscrew

- d086cd80f36f9b36ed3997da2484df90d70f9ea5 - Update Cargo.toml version - containerscrew

- 0f1eb26684eef27e781d532366464e5f75ffa7a7 - fmt code - containerscrew

- 7a92677ddc45783fb8c1c70b93e119a2c810dc37 - Update CHANGELOG.md for release v0.6.1 - containerscrew

- 1d50b012b7218a57b8074f15c0b5b1f93d985765 - Fix pid and process field in json log - containerscrew

- c0aeab34687947947da0466d261fd54a44f83ea0 - Update CHANGELOG.md for release v0.6.0 - containerscrew

- 9034b7d7b94f091f647f6baf1692372ecc1ff2e5 - Update Cargo.lock version - containerscrew

- 1a6967832523c168a3764afb8b12d8801cf44624 - Update Cargo.toml version - containerscrew

- e481358b0a82afd8105760cdade3e529d2e37386 - Improve ActiveConnectiont tracker & cli help message - containerscrew

- 7088c25544392859dc460ffcadfabcb932cb5679 - Update README - containerscrew

- 24b28f695da04242910ecad98e0b795d232e6ad1 - Add new example - containerscrew

- dd236fb5eebbd77dba2c993c6d68462e87618800 - Refactor: folder structure - containerscrew

- 678b2f011a66ff4b61f4df09a29b9e2d41ae0821 - Update CHANGELOG.md for release v0.5.0 - containerscrew

- bb8d0f033f35f65e1da7fdf8db1a6efe04265ee3 - Fix release pipeline - containerscrew

- 395523492cb85b77c932576288386a16e04e7204 - Improve gracefull shutdown & update log egress event - containerscrew

- 2bab4ddbd3ff9ee2556ff39be975f9a28e7b6078 - Merge pull request #63 from containerscrew/dependabot/cargo/libc-0.2.174 - github-actions[bot]

- 2d247a917ad016fcd00611331424b8632b8e332d - Bump libc from 0.2.173 to 0.2.174 - dependabot[bot]

- 1fe055817e4ed38b87fe939b08473cfdb7615db8 - Improve gracefull shutdown & update log egress event - containerscrew

- bfa8e6a35598fb4fe4407db2ff911ba7301a055c - fix build pipeline - containerscrew

- 7571a42a1c8fa5d40eb0831180cb8502ca01e97b - fix build pipeline - containerscrew

- e7913869d086836ff61d9fb2863f6d20dfceb76d - Update CHANGELOG.md for release v0.4.0 - containerscrew

- 52588947e623fa509fc3beb3f6da5c9057d84241 - cargo fmt - containerscrew

- 1733fcc9875208d4daa0ddab41b705907f2d05b8 - New v0.4.0 & fix unused imports - containerscrew

- 696b9a8d7fb77fc0a67969b2a5d270a74c11152f - Implement pid and process name for egress traffic - containerscrew

- c57ea184a8cefe00b1e50d37b7fb3337828fe79a - Merge pull request #62 from containerscrew/dependabot/cargo/libc-0.2.173 - github-actions[bot]

- 30b1ae922ad526baf99e4eb6fd61b8e23cc29bef - Bump libc from 0.2.172 to 0.2.173 - dependabot[bot]

- f62525b80526e8f3c901aa6311120f3c3e6adeec - change after help message - containerscrew

- 414f5af31d264e8268e275c90c25bd5f7ee2625f - Fix pipeline zip package - containerscrew

- cd45a5bef05feb4cf0d67fd023a416b6cf8e604b - update todo.md - containerscrew

- 435fc70c4451812062b9e67495c5b0d37f3fe2c5 - Handle TCP state flags - containerscrew

- 341a2b35344bc425ad5588f2db7ebed55960ecec - Add TCP state examples - containerscrew

- 36b8ff9f35d1c50669388a66615e596145be8958 - update doco.md - containerscrew

- 1d5069a9c134a67855a0370cbc42efc4e7adc0b0 - Merge pull request #61 from containerscrew/dependabot/cargo/clap-4.5.40 - github-actions[bot]

- bdcf78644b04cc98ad7e5beec8c0da31301080d0 - Bump clap from 4.5.39 to 4.5.40 - dependabot[bot]

- 84cb46ad755153e446ce19b4c731889f7813d240 - Update CHANGELOG.md for release v0.3.0-beta.5 - containerscrew

- 73712d1fa8effc563fac5c2a27821ee9daceda5e - Update mtoc in CHANGELOG.md - containerscrew

- a7d50140dec1a9cfcec7a275a2dc5da97008fdb9 - Update docs/todo.md - containerscrew

- 84d39f5ab44bea1e4784ff832a562328d4157656 - Add mtoc in changelog release pipeline - containerscrew

- 99533c5dc29d3068180a26700b2d2362631b736f - Update CHANGELOG.md for release v0.3.0-beta.4 - containerscrew

- cab105166e2e243fc04430cef36c4103556ea342 - Change welcome messager install.sh - containerscrew

- 10ea2c9008610915aa152c15265eca7e50c5a60e - Add cargo fmt in pre-commit config - containerscrew

- 5a7009f210211518596335f2351651b466a1b06d - Fix: rustfmt - containerscrew

- b40d0b59e3fde8172c968513c6c93015e69e7703 - Fix: reimplement --disable-tcp|udp|icmp flag - containerscrew

- d0517405a11da19a9a68b305c4fb6dd2eb9150aa - mtoc in Changelog.md - containerscrew

- 46ca0a20be3fc7615725be18485b8eaa52e4ae1a - fix(install.sh) - containerscrew

- 7b78dc6b2a94fd811e52441105b679ecade2c68e - Update CHANGELOG.md for release v0.3.0-beta.3 - containerscrew

- f0251212d0c46bf3e52310ee0abd571d35fd6da7 - mtoc - containerscrew

- 357171ffeef362878816f2486df488bee3812720 - Update CHANGELOG.md for release v0.3.0-beta.2 - containerscrew

- 74661221dee3c83a0b165ce1ca77b6be8218e7b7 - Merge branch feat/filter-ports - containerscrew

- 0d1aea0970c58d7429c3b1e2fe641751284a76b8 - doc(todo): add new todo - containerscrew

- 4c22abcc3ef42e11c4bccbf898dcb2908e5b97a7 - Bump sysinfo from 0.35.1 to 0.35.2 - dependabot[bot]

- e4ae070209a5bd1928ffdba550c924033e50e517 - cargo fmt - containerscrew

- 5247048e58dad4585206109630b3365526ffc3f2 - doc(readme): update README - containerscrew

- 7e44c846bb91a020f7dd23044dfe418f7960161b - doc(todo): add new todo - containerscrew

- 844e6a729b8c1b3fc87f5c1dd465014d3c9d0ecb - Merge pull request #60 from containerscrew/dependabot/cargo/sysinfo-0.35.2 - github-actions[bot]

- 65afedc2e8fd2b83e1f5490b11a6c47cea2482da - Bump sysinfo from 0.35.1 to 0.35.2 - dependabot[bot]

- 259fbf56b9aabba559ad1d95ab4a9d29ce566392 - cargo fmt - containerscrew

- 1ae5c4a9f2c52819750fdcc6ffffd4785b5d5e17 - doc(readme): update README - containerscrew

- ef2fbea749b5eaaf9183139463b08648be8a0a23 - Change cli after_help message - containerscrew

- 6d5629d818f17bace3c743c6e30db104a480b43a - Update doc - containerscrew

- 84b8f89110fffdec2907484c89addee4d522fb5d - Add cache build pipeline - containerscrew

- e98189aa2470ab105256ecb4df7cfa7ba6d5a367 - Change cache in pipeline test - containerscrew

- ae31855db5501818ccd665c44af276585236f46d - Rust fmt - containerscrew

- 0bde0d02d7239eb40cbc8e0baeea2e0811248e71 - Change nflux project description - containerscrew

- 809a04395118bace23d9d6a973ed549baf4fb4a9 - Fix test format_mac - containerscrew

- ab8b256ee8500d97a05bd987908e995788d6cb75 - Update usage.md - containerscrew

- 93dde908622a2017781c54303a4fa46c4c157aad - Allow user to disable timestamp and other logger improvements - containerscrew

- af24bea98c4781f6b7ebb160f10358d8b47a5ff7 - pre-commit - containerscrew

- 335980adcbedb5739acee6a31881b8ca08341d7d - Change logger - containerscrew

- dee4896d5d97f670ca302a5a5d8fae9f99e3f4e9 - Add todo.md doc - containerscrew

- 41d13a9cb55b92eaa835eaa7c48072f66adda319 - Change nflux logger - containerscrew

- a42b843017ff7db280db96a5983933414d63b7d0 - Merge pull request #59 from containerscrew/dependabot/cargo/clap-4.5.39 - github-actions[bot]

- 5eaa89b5e48d699fb1a18940db5dcbb2cf0658bb - Bump clap from 4.5.38 to 4.5.39 - dependabot[bot]

- f6f634d2a1527e88b25d9f46546e735e89604179 - Fix: error message when programs runs without sudo - containerscrew

- 941746630e38494dc8175459f799cb44eaff3289 - Improving logging - containerscrew

- 62ecc049be87f1cab3d02c4dd79d921f9dde599c - Remove prometheus metrics - containerscrew

- 63647d6c7888eb1a32c0928f4e9496869367a5db - Reimplement disable_full_log - containerscrew

- a5ad4146fdda6d9427fc07d53d37a2c4c3685f29 - Implementing prometheus metrics - containerscrew

- 4573cc42494d277523d25ab4249edc56c0637e47 - Refactor handle_packet - containerscrew

- b60df5555dd470f31a10d9674f4c4884d2e7d98c - Update README - containerscrew

- aa4073c2210ef5a4525cfeffb18c6c56cf83c3d0 - Comment lint pipeline - containerscrew

- df7386a4b0c12272d183a0b01e0ca17a5d13b852 - Wip: lint pipeline - containerscrew

- 6f65916747ac1396c268156c03a7e20975d8ae84 - Update lint.yml pipeline - containerscrew

- ed20a7fb319ff5e8bfc040dcbe2561ca29de4cc1 - Update lint.yml pipeline - containerscrew

- 194ebdb9d19a4fc15e38ee9b99e43dfeddcad326 - Update test pipeline - containerscrew

- fa3a446507f9c7320be6befa9a8a842321bac185 - Update test pipeline - containerscrew

- 5b554c7c96d4d1ac3b003604735f34020ba43503 - Update test pipeline - containerscrew

- a0cc17e0a289ccaf4d4c5b6c2559658ec844b91a - Update test pipeline - containerscrew

- efd1eb2c9beba62b876d5a125dfdd0db5f5d5aaf - Update test pipeline - containerscrew

- 442aa5b6629083e208c65d6250d56405151b274f - Split pipelines & organize lima files - containerscrew

- 77da66803b4c94155dd5cb24b0383740bcb6dd8b - Update installation.md - containerscrew

- 580cdaf4fad31da7def45b977c2ee3853a0e348d - Add debian lima machine - containerscrew

- 120d9723281d11a3b390682ace70fb9a59964080 - Update README.md - containerscrew

- be550f5c356a65331accdcc77dc1e0e93ae6fd3e - Update README.md - containerscrew

- 8491bfea63ec07237d4be88f0934e962fbc66626 - Update CHANGELOG.md for release v0.3.0-beta.1 - containerscrew

- 9835154cba1a5c3474d97b24b04185b3cb19baf1 - Update install script - containerscrew

- 829c403b35bb6a66fcc56aa78468ee467476da9f - Update install script - containerscrew

- 2fb908fd5ae9431e228dfe6ce1c3f014f9d04948 - Update doc - containerscrew

- 546b70423bc39ebec648b0518ba5035f44e78774 - Update doc - containerscrew

- 5f1cdb6b07c552f723d8071bff39cef2ef70ccff - Update doc - containerscrew

- 053ff1e6534e07583b3feb102b68c11e28faa0b1 - Fix proto & update doc - containerscrew

- 28b76fab675cdd67d3406e67e0b1e84f8e213518 - Change license to GPL3 - containerscrew

- 4ccbee764c713fad285bfb3ca602a5968b8d0fc4 - Update Cargo.toml version - containerscrew

- 59f818f235bb3c22f7761a5395d9ba3654447a4d - Merge branch 'refactor/remove-tlstrace-feature' - containerscrew

- 58436a833cb6183082f78f7347a21a554b64bb71 - cargo fmt & update doc - containerscrew

- 8dfd6dd2d75a566e44fe739bf41dd5b66383abb1 - Add format_mac unit test - containerscrew

- e47a120527be021d2eced2bb8d1a81bd27952797 - Refactor: handle_packet function - containerscrew

- 8dcddb8bac269b92b2f70ce9183a12a37d7dc51d - Rollback cargo nextest - containerscrew

- c6f584a4a0b8743f99a7adf547e5677d4a555b7a - Add cargo nextest - containerscrew

- 375a3fa7c4c17fb50326c6b4cb6c3f0e2d29cb67 - wip - containerscrew

- 316594e1c1013bd294f695346ddeea4905d836d9 - Refactor nflux/ package - containerscrew

- 6a2009fb84d9b98bb7c28d6ccbfba142a4321fef - Add src and dest mac - containerscrew

- 55fa808c30852588731a4fb65e0e98c7fdc63ed4 - Add some useful comments - containerscrew

- 2ba8edd1b5d9357e096f5a7f0fef06b15790c1ee - Merge pull request #57 from containerscrew/dependabot/cargo/clap-4.5.38 - github-actions[bot]

- 9507ad12a5e7bd355d1b169b79afb061cd198c5b - Bump clap from 4.5.37 to 4.5.38 - dependabot[bot]

- 7ae17c538c721ef7c2a7a949bd5edf0c44ed0625 - Wip - containerscrew

- 6c4bc44a91917301bcc21c7465e9ff58c0cb13c3 - Wip - containerscrew

- ddf38ad7f6a28454749d05cd7d65325a46c30658 - Removing tlstrace - containerscrew

- 15786272db8669fbacc5182164ef0b3d830dbf12 - Wip - containerscrew

- 3dc18a0bb617189b73654fb69d2a3c42a5d35ab1 - Merge pull request #56 from containerscrew/dependabot/cargo/tokio-1.45.0 - github-actions[bot]

- cebd55c8c3df232be8aa4bf7500ab8936da6fe51 - Bump tokio from 1.44.2 to 1.45.0 - dependabot[bot]

- 41a9e5f63026b3650de81d067f7d58a21f218de5 - Removing tlstrace code - containerscrew

- 9fa47aa8f09b3b91894c745a66b2c9dad001ba0e - Merge pull request #55 from containerscrew/dependabot/cargo/sysinfo-0.35.0 - github-actions[bot]

- 4be99f3aaf8e4719303a2816ce84b48861fa03b9 - Bump sysinfo from 0.34.2 to 0.35.0 - dependabot[bot]

- b7b536dd50bae767d138e2364eb49faae7edd7d8 - Merge pull request #54 from containerscrew/dependabot/cargo/chrono-0.4.41 - github-actions[bot]

- 7623bd680988778ade70d3e5e59d78bddd5ae38f - Bump chrono from 0.4.40 to 0.4.41 - dependabot[bot]

- 0eaccf99681c3e4eb76456222b379a950620072f - Update doc and Cargo.toml version - containerscrew

- d5b2eec767017b6ae04b01e48bd7fa5bd1281fd5 - Merge pull request #53 from containerscrew/dependabot/cargo/clap-4.5.37 - github-actions[bot]

- 2ea3b783804723f87187b392f9876edc37b306df - Bump clap from 4.5.36 to 4.5.37 - dependabot[bot]

- 864ea8576a719e71a60294f623cda39b0223d09c - Merge pull request #52 from containerscrew/dependabot/cargo/network-types-0.0.8 - github-actions[bot]

- 44b73580fa17d1f311b399b79b75e3888d3345cf - Bump network-types from 0.0.7 to 0.0.8 - dependabot[bot]

- 0da764b415efad49c1c13d75186dcc59441460ce - Merge pull request #51 from containerscrew/dependabot/cargo/libc-0.2.172 - github-actions[bot]

- 4b530f92c7f157b91c640e027aa2e0f97c22105d - Bump libc from 0.2.171 to 0.2.172 - dependabot[bot]

- b2fe159a5aee363af7d7ef6da107ee21a89b87ac - Merge pull request #50 from containerscrew/dependabot/cargo/anyhow-1.0.98 - github-actions[bot]

- 2a88da0ca4218627340520df72335b98ba9d5a31 - Bump anyhow from 1.0.97 to 1.0.98 - dependabot[bot]

- 187680bd022230e3503488668d669d23eb834ce3 - Merge pull request #49 from containerscrew/dependabot/cargo/clap-4.5.36 - github-actions[bot]

- 1cad578bd1c832ce470681a872d49a6389b43ad6 - Bump clap from 4.5.35 to 4.5.36 - dependabot[bot]

- 2925b9e70db20db2cc1a54b8e4c79d009a46994b - Merge pull request #48 from containerscrew/dependabot/cargo/tokio-1.44.2 - github-actions[bot]

- d94c34ff69240bfed18ffa0c887f7480a9ba2711 - Bump tokio from 1.44.1 to 1.44.2 - dependabot[bot]

- a1058a565b3100d6a77ede1f8e9486ba7b97b59b - Merge pull request #47 from containerscrew/dependabot/cargo/sysinfo-0.34.2 - github-actions[bot]

- d49a9ea41a30c7658397482468c3d4a25b84a094 - Bump sysinfo from 0.34.1 to 0.34.2 - dependabot[bot]

- 9976d28f38bec743d863465c5c380e4d44768157 - Fix build_deb.sh - containerscrew

- 5f8d0f94347a2f6de26758ea586b42f90c2a654f - Merge pull request #46 from containerscrew/dependabot/cargo/clap-4.5.35 - github-actions[bot]

- d6e0055a521037ade8a2f19dc1297f4b292d552c - Bump clap from 4.5.34 to 4.5.35 - dependabot[bot]

- 395080fcd808cd228042e2221cb11c7e3d44c8cc - Update readme - containerscrew

- 82b98f49b042a5382017b1b7338010cf41b717f2 - Wip: rpm package - containerscrew

- f11247ae27a37cdf82be53a300c5e29764b60898 - Change package description and delete package.metadata.deb - containerscrew

- 84b7809d2263005d78f9fecae0ff8f53ae15a1bf - Modify fedora.yml lima vm - containerscrew

- a92a808e1887334d404c07b5caf15db4e1c21aa2 - Wip: dpkg package - containerscrew

- a8d927cda51fefa2ca75bd12cabf9798a83f0fb0 - Setup debian dpkg package & refactor doc - containerscrew

- b416d97a0946296c336052356368c8d3e33257d4 - Setup ubuntu machine using lima - containerscrew

- a2bdb12635cd65e824e842a30b9b5cdab4c19549 - Setting up cargo deb - containerscrew

- e711b512ae47f66e053f566a9952672f92e04b2a - Merge pull request #45 from containerscrew/dependabot/cargo/sysinfo-0.34.1 - github-actions[bot]

- 39bffcec1bd722f4650457087e6ba8db12bc10bd - Bump sysinfo from 0.33.1 to 0.34.1 - dependabot[bot]

- 0d689174a4c950e1e44732b3000bb732a2eb1c22 - Merge pull request #44 from containerscrew/dependabot/cargo/clap-4.5.34 - github-actions[bot]

- ead051869aebdfb2171de59229fa5611050b04de - Bump clap from 4.5.33 to 4.5.34 - dependabot[bot]

- a4c4eaae96bc052a5099ffc35fffe8d41d4eb888 - Pre-commit & CHANGELOG.md - containerscrew

- a3263a1425d473946bc630c312c3f8cc320b603d - Update CHANGELOG.md for release v0.2.1 - containerscrew

- 8bb1e7511e08e5c99c66697f76a1006e6975cc6a - Fix: error when no default iface (no connectivity) - containerscrew

- 6cf4fbcfeaf3b317ebd1eed2110a752b90ac4f85 - Merge pull request #43 from containerscrew/dependabot/cargo/clap-4.5.33 - github-actions[bot]

- 53de09690d0bebb75d69ce647f929e75c68d51e9 - Bump clap from 4.5.32 to 4.5.33 - dependabot[bot]

- b302eacbf19b5db064ac7e28c26b2a601f556c16 - Update CHANGELOG.md for release v0.2.0 - containerscrew

- 856a891803500853a0deb8bfd135f80287289a6a - Change cargo version - containerscrew

- fd6528a79225b3c1062e7fd72b1d324e414482fa - Delete changelog - containerscrew

- 919731d548a25ba482c745ef8ffbd57ecdce9fb0 - cargo fmt - containerscrew

- fea3fd51d8a2c80ab097b2699df3ad9c9da45af6 - Update installation.md - containerscrew

- 197c1f19bbac32564132687de12ec2036c12fcdf - Update CHANGELOG.md for release v0.2.0 - containerscrew

- c5ad0adb5f42fc9c283c8eee2894d63286d27452 - Update installation.md - containerscrew

- 1a43dbfa6ab172a6ddc0de4aa8bc5c4ea5b13bcd - Delete crates.io references - containerscrew

- ec8a726347dd7161d78631cea1ae6c05b13a2063 - Delete workspace dependency for nflux-common - containerscrew

- 77543047b383af3c81bb4a3e8c191924dfe7cffb - Strip nflux release - containerscrew

- 9037ef402d033f9dadd846deda46d956a687de06 - Update Cargo.toml - containerscrew

- 76846e4cae90aa9985d34494f3a1c664a1a9ad1b - Update Cargo.toml - containerscrew

- 75eb602b9495d774aad37ffef676cc36f47b92a9 - Update Cargo.toml - containerscrew

- 49d1f15ce483cd6720269139f2e8155763a14623 - Update Cargo.toml - containerscrew

- e4064c8540619a9201de6a27986533325c18607a - Update Cargo.toml - containerscrew

- b9381bdb7459e38c0338c826da7fe4ba951e0ba8 - Update Cargo.toml - containerscrew

- 96540679c21854583c836b424175cd451d75fb18 - Update nflux/Cargo.toml - containerscrew

- ddd0917cb4f62026d8f8890a55a37417a7e51de4 - Update nflux/Cargo.toml - containerscrew

- 013e2a3088f6098db26ac4379c4a060d430ec4db - Comment cargo publish in release.yml pipeline - containerscrew

- e6a9183340f594a5d5673070d9124a5952073695 - Add comm in netrace logger - containerscrew

- b00c0840ccc959ffc76cbd8b01c4f416948a84d0 - Update doc - containerscrew

- c4f6cbff8d0b2426e8cff92bf76c460b7b3c502f - Update doc - containerscrew

- d5ed9c54ccca97c8acfc5fd8044808243bab656a - Change test pipeline - containerscrew

- eca092ee5d8fb81f84b78daf4e61eba4e1ff01d4 - Update rustfmt - containerscrew

- 6b7d62778d22e02c5a45721010b1350406a3273a - Update badges - containerscrew

- bfc69ef56909976eb249d6abcf45cb9ccfe49aff - Update doc - containerscrew

- 929350e5f57f038660c9f4dcd040567ff0901590 - Update doc - containerscrew

- b7f4cc1b7913907c4056075ba3bb0e3c3bf2260c - Update doc - containerscrew

- 79450841af2205b74863a6f2a27b17e610bff246 - Update doc - containerscrew

- f1cf8601da818a37e8a21d5a78b9728a5725c754 - Update doc - containerscrew

- 54b331c15599bb9216da2e61e2f3705569c0ea2a - Update doc - containerscrew

- daf439c869d2a006e62438768372e333f3aac9b9 - Update doc - containerscrew

- 1989013a1c57fedf2683d327ac5181645f4fcc6c - Update doc - containerscrew

- 2f678b1955777e957c8fd11bb923270a06579a76 - Updating doc structure - containerscrew

- fd5e185efbd493dba6dcd30553f4bf5767eb816e - Merge pull request #42 from containerscrew/dependabot/cargo/log-0.4.27 - github-actions[bot]

- c6659e5b459c85168fa9192a6f70477fbdf8ab5a - Bump log from 0.4.26 to 0.4.27 - dependabot[bot]

- 068f463f2437c6463738958baf094769c9b8e0ff - Update doc - containerscrew

- c001d4637a165f2412f731d320dd1f3bb7d08bdd - Add lib.rs in netrace-ebpf - containerscrew

- 5044ad4f06c4ba6327b615e1d91ccb1c8ea2ef73 - Fix LICENSE file - containerscrew

- 53ce49074f3844cda4fdced16814179ed96523d4 - Add versioning to Cargo.toml - containerscrew

- 691558542802c2621aea98350d5cfebfea94b357 - Add cargo.toml metadata - containerscrew

- de8e03821fe1bb7024128d2552273a7d48e40caf - Update CHANGELOG.md for release v0.1.0 - containerscrew

- 4ce19aac01a48fc687590d771e3b6a5db8640ee3 - Strip binary - containerscrew

- 18eb085eaf1eac13a154b1cb8b94f7c531c68d02 - Fix zip packagein Makefile - containerscrew

- b8a5ca7f4b57989af48cf256aff6e5224ea93633 - Release pipeline - containerscrew

- 674c6ac4faaa26199dcfabb7629c761462a70551 - Wip: release and build pipeline - containerscrew

- f8278456c66f47334350c12d8f74ad96dc59f698 - Update badges - containerscrew

- 16c1e92a22b6a3c9559bc4e9f2b1b2390653aa72 - Wip: build pipelines - containerscrew

- 810bc37fdbf01cf5cfb01c0c326808c767a9f27b - Wip: build pipeline - containerscrew

- f17b6aa6b05a724820dd29151b114dd10dc05fb2 - Wip: build pipeline - containerscrew

- 62a20f46e0c16dc17223c634faaada57445c7b14 - Wip: build pipeline - containerscrew

- b4238e362042823bad6ecdc44cb409738cecd889 - Wip: build pipelines - containerscrew

- b4b5c84519d525031dcf05c5b540533fc6c88c73 - Pipeline updates - containerscrew

- 1d4e6ef1b86dfc68c9cb690611c68fd21d02c223 - Update doc - containerscrew

- 11287a964c28aba081a06949f2f75a0c90bf0dde - Providing installation methods - containerscrew

- 4d2c7f726bf56e70041c9bbf710444f2039495b9 - Update doc - containerscrew

- 7ca644cf9a3e3f1d18f03b1a5292f4b063656bb0 - Update doc - containerscrew

- 40217e1063359a44e546d57c6ca76a7bf84dde41 - Update doc - containerscrew

- 736f4a177c466d648ef538866dd40cc45294bd05 - Update doc - containerscrew

- bfe44c2a16c2a13d5c0ed20f23c23f6d4e22d4d1 - Update netrace logger - containerscrew

- 3184920f16681dd63793dc2f3266f84c4a92b89f - Add git changelog - containerscrew

- 574fc342819d6a91b021fada9d1c98f26d48f291 - Improve TC log event - containerscrew

- 8479601d957e94cc8418410de1bf57c962e72894 - Updating doc - containerscrew

- 72c0fa20170e2029cedc8bce055fdefb9f529132 - Updating doc - containerscrew

- 2a3e2315feb32b84014a791de33a9dab2bcd4320 - Updating doc - containerscrew

- f179063c4d609858180d93909b59d0d75bf953a8 - Update README.md and cli.rs - containerscrew

- cabd7add4965e3529e43295d6875578cdedb430c - Update README.md and cli.rs - containerscrew

- 4041bb049bd00e9ddccb63b92daa815d04557bfc - Update README.md and cli.rs - containerscrew

- 66f6177b7f1f5f18412d4a4ffcf693b5f79e9cf4 - Update README.md - containerscrew

- a0e3deca650d0b34fa288307a3c6323e1edd7c41 - Update README.md - containerscrew

- 147f7f4876bb0f1be7e978c049a49e6d34ec0701 - Update README.md - containerscrew

- d412a76299212455ff9d9d62eca36703fcd5071a - Update README.md - containerscrew

- 64007b76bb1261059e1f62b9f1d89e50b6e17598 - fmt - containerscrew

- cc50d6de6e5f5172fae0b7f6d33bd1ba0c059d16 - Implement tlstrace - containerscrew

- 1f84db51edca3128e3fe8725a8a52fb8208784b4 - fmt - containerscrew

- a399ff0326ab1d83b253eb2c4cbf370160ad28d9 - Wip: tlstrace command - containerscrew

- 536aebf108680e57ef0c594b5c535c63c558a15e - Update build.yml pipeline - containerscrew

- c6e1131558e2f36681324e3bb30e7b542f1fe7d6 - Adding ebpf code for tlstrace - containerscrew

- 09c2d58c5d07e69b7b25eb2853e313c42f951360 - Wip: build pipeline - containerscrew

- 0e951c7b7612b9ff7637e7fda8f17507c3060025 - Wip: build pipeline - containerscrew

- 90d2eb0652c31f3cf675e72a1f313fe74d2cfd91 - Add build pipeline - containerscrew

- 3737c57c6e175069248d1403d347b2c1210e5fc0 - Update doc - containerscrew

- 77713f4d9536d1057a52ab7a8800d8aa6d553ea3 - Update doc - containerscrew

- b4462a5d8db8abd0f60464a0929c079b4f996b6c - Rename ebpf code to netrace-ebpf - containerscrew

- 82e79f3e45bfd5b3b43be5c3bd465ed7bee3a594 - Cargo fmt - containerscrew

- 10a5165047066af5f7157b3713a542f0f4bd90e4 - Massive refactor - containerscrew

- b033b3faccc72c421674a53bd82b0f657f4c606b - wip - containerscrew

- 4f5214fd763f0b09d203f14ce6a3ce0cf34a6b47 - wip - containerscrew

- e123db93d45110da0fa7f4791a441768f237aff5 - wip - containerscrew

- 2f682699cb47318cb33b7d0abb285da9a1df1dad - pre-commit - containerscrew

- eb4c7eb00ee3eb9842edeaddfa094ef581fabde8 - Implementing netrace subcommand - containerscrew

- eff8d61026bef84915433e280c56980f5a7d21fb - Implementing netrace subcommand - containerscrew

- 1575629635ea1593327fd978c59c34d2ee527d93 - Implementing netrace subcommand - containerscrew

- bac2eefb25c0fb0710881abde57d279ec3d9c864 - Restructure cli code - containerscrew

- 2b86c6c1910b70216c6bd6a26a32ba94fe0cda37 - Delete compose.yml - containerscrew

- 0258a66aedf118030f7424528bdea5b026428f01 - Update doc - containerscrew

- c871519cf4df95e837f271aa7ae23d660560c563 - Modify Dockerfile - containerscrew

- f152d3586fa44f207c36fae4a8dbd584a028b484 - Updates - containerscrew

- 3b36e5f2054c14d98f562adb6ab4215ad601cc1b - Wip - containerscrew

- de1b0ad885ae4d8d12a166e1edb0196d210d7572 - fmt and clippy - containerscrew

- 36ce90bde8975c0376866e2214628248dff74a91 - Massive update - containerscrew

- 3bf759fc6aec6fa92e506757a520720cfdc0966f - Update README.md - containerscrew

- a2e5ea2dcb4d27220e26bdea2638a91c4841ba3c - Update doc and build pipeline - containerscrew

- eeeea3e3129514d90c598ab03516689039ba258a - Edit user space logger - containerscrew

- 57b9a28cc921328ebb4358b9391e7bb5c36939a2 - Add old doc - containerscrew

- af2c70aa6d9c378ee4174b22cb33c2da6a8023af - Delete old pacman PKGDBUILD - containerscrew

- aabee3ee0d313f44fdd4322e06f9d22ba46e3b85 - Wip - containerscrew

- 61de22a52fd1fced3d370b143ea7b7e9e955a7a5 - Remove prometheus dependencies - containerscrew

- 5c48d9d56ba9de9caf2fb1a9472c8c68643929eb - cargo update - containerscrew

- c402db4a26eb4da974f0af4a88de1dfe1c74b55f - Fix cargo clippy - containerscrew

- 02aaba3b311b6bfd2dc4e07d61e2767ad48c3e83 - Fix tests - containerscrew

- d0b1fbdd4e6c12f49df567d4c68f1fc5838ba6cc - Cargo fmt - containerscrew

- 2e8dc502fdf211d0bf2dd0f7f52e3b0b6f73044c - Change test pipeline - containerscrew

- 300bb9bf39201cbdc026e068ee0499723f9ea80c - Change test pipeline - containerscrew

- 5a2b31d74190a1e5ea7c631b350477e9e2f78616 - Update README.md - containerscrew

- e3e5024ce9ef02ecc4e9927d9cef3a06f59ec730 - Merge pull request #41 from containerscrew/dependabot/cargo/tempfile-3.19.1 - github-actions[bot]

- 0f096a9d0155bc650f3f4fd91ce9d9b00e01c847 - Bump tempfile from 3.19.0 to 3.19.1 - dependabot[bot]

- 5f6b0ac36d61bd1a3dcc4139ce156cc349289ffa - Update README.md - containerscrew

- c06c9860778b883283915e5d685e0e6c03dcee61 - Update README.md - containerscrew

- 19220cfdcabb4d5cfc00f3b1c7acb0d0fd66cff6 - Remove multiple interface program attachment - containerscrew

- c26cda6e990ad925972f35b0c20816404a25a987 - Update README.md - containerscrew

- ae207899dd2b78f7c5137e0caaa495cf82037def - Updatge todo_and_features.md - containerscrew

- 319493c0c31ca1aaced3960b843368ddbf7190e6 - Doc & move func to utils.rs - containerscrew

- 2e057052bf7ce3da907a721f22ac83ba363d00e0 - Adding more log fields - containerscrew

- f03576c5d92ede0446cff8a1537554175062ae90 - Add fedora.yml for mac osx lima - containerscrew

- 6441ad7b51d8b0215d01282df84f16429dfb8ee4 - Fix cli: help usage - containerscrew

- 38eee33b9d7153c03246a7072ae5570ab3ef81b1 - Merge pull request #40 from containerscrew/dependabot/cargo/reqwest-0.12.15 - github-actions[bot]

- 8baefb4df3b6b7202b4623fd192d4fc2604477bb - Bump reqwest from 0.12.14 to 0.12.15 - dependabot[bot]

- d23a63a622687d29415c9f1c5982b5bf1ed0f2cb - Merge pull request #39 from containerscrew/dependabot/cargo/tempfile-3.19.0 - github-actions[bot]

- cfac4e88981aca57c6b9d1520b820c6a8e20385d - Bump tempfile from 3.18.0 to 3.19.0 - dependabot[bot]

- 4322c9b9ce1272ad6b1b44d5ed669bda6f7e7c0d - Merge pull request #38 from containerscrew/dependabot/cargo/tokio-1.44.1 - github-actions[bot]

- 8163a3f5912bd684070af9d44351e8da6480fb2f - Bump tokio from 1.44.0 to 1.44.1 - dependabot[bot]

- 2cac2b8a19a37c0cddb33fb5b29167f164d8f5a6 - Merge pull request #37 from containerscrew/dependabot/cargo/reqwest-0.12.14 - github-actions[bot]

- 039706b49a6ab455c866f31f1c6ba8ede589ad70 - Bump reqwest from 0.12.13 to 0.12.14 - dependabot[bot]

- 883e038cc7c01fce18c3a87c729a43d5f4699ccf - Merge pull request #36 from containerscrew/dependabot/cargo/libc-0.2.171 - github-actions[bot]

- 838a66e2594a158bd1b766ab7f71e9e58b1c8129 - Merge pull request #35 from containerscrew/dependabot/cargo/reqwest-0.12.13 - github-actions[bot]

- 7754cf1f59fb11fb404f61babb3330bf07b5afbe - Bump libc from 0.2.170 to 0.2.171 - dependabot[bot]

- 83f3dfa694ed3e859cdb4bf005816a1c6c6bbc73 - Bump reqwest from 0.12.12 to 0.12.13 - dependabot[bot]

- a0ee6dfe9f87deb8ad108d098331d9d947bd104c - Merge pull request #34 from containerscrew/dependabot/cargo/clap-4.5.32 - github-actions[bot]

- c6cd8e27fa9187dd6243355ee748740dfc67a8be - Bump clap from 4.5.31 to 4.5.32 - dependabot[bot]

- 5b4c3703aa136d53be979e3d9aac0ebfb32cc421 - Merge pull request #33 from containerscrew/dependabot/cargo/tokio-1.44.0 - github-actions[bot]

- b31961e80162d8e2e2771bc82fbf95b611b9a32b - Bump tokio from 1.43.0 to 1.44.0 - dependabot[bot]

- 9b330a0825e65e861d1180145201655a24798558 - Merge pull request #32 from containerscrew/dependabot/cargo/ring-0.17.13 - github-actions[bot]

- 5617010a2a3e304a834dd07a7a9f5bf492064acc - Bump ring from 0.17.8 to 0.17.13 - dependabot[bot]

- 9c51e9e0f70d42029521f80ac4f470fb2306b665 - Merge pull request #31 from containerscrew/dependabot/cargo/tempfile-3.18.0 - github-actions[bot]

- 6fb2e9a47a87130591ab8d4633e32fb1284b041d - Bump tempfile from 3.17.1 to 3.18.0 - dependabot[bot]

- 94f7548bc55631caf6065f8ff849e21635e14244 - Merge pull request #30 from containerscrew/dependabot/cargo/bytes-1.10.1 - github-actions[bot]

- 02d182f704b702044d4c207d9e5f2ba8c9b471ce - Bump bytes from 1.10.0 to 1.10.1 - dependabot[bot]

- fbf66412fc394fcb38a3fdd6445f299e4a6e08d2 - Merge pull request #29 from containerscrew/dependabot/cargo/cargo_metadata-0.19.2 - github-actions[bot]

- 5032474deac9a34995243a13dbeba9b3090c3750 - Bump cargo_metadata from 0.19.1 to 0.19.2 - dependabot[bot]

- 32b09329eb173d49cb0aacd001c8d917ff949c10 - Merge pull request #28 from containerscrew/dependabot/cargo/anyhow-1.0.97 - github-actions[bot]

- 6838bf69b24f0f786b2bb4d229b148d721ccd52f - Bump anyhow from 1.0.96 to 1.0.97 - dependabot[bot]

- e06abd19a28fb3b3a70224ad30ce1b09cef73410 - Wip: modify log - containerscrew

- 61be904c3b53ff94439bab9e69018ad8383a839b - Setting default iface if not set - containerscrew

- 68b57bc26384d8428e6d520a07ec4b6389869f38 - Wip: refactoring handlers - containerscrew

- b3b87b68c38ba47268e3fa36537e44089451e077 - Wip: README and & cargo fmt - containerscrew

- 667afb0e3888b2becaa88224d98f6c7fd831900a - Wip: cli - containerscrew

- e52ae7b5d161f524bdf5c2cf3846c4c804e85cbc - Comment some functions - containerscrew

- 00cfb7bc02c5d6da32417ba9c24ab2099191e806 - Adding ttl, total_len only for tcp packets - containerscrew

- f159b32474880d6f9d084bcda0813d0e415ebb80 - Merge pull request #27 from containerscrew/dependabot/cargo/chrono-0.4.40 - github-actions[bot]

- de0135556078b70b3e23022c0711c807ebb285bc - Bump chrono from 0.4.39 to 0.4.40 - dependabot[bot]

- 41827c4d91374bcf3e625fbb461ade21dc6ada49 - Merge pull request #26 from containerscrew/dependabot/cargo/libc-0.2.170 - github-actions[bot]

- 721778f50f3b8dc1628c9f2982b385c4742842c7 - Bump libc from 0.2.169 to 0.2.170 - dependabot[bot]

- 6f2645c5db78b28c96ce1d02840598f2a9c0cfd0 - Merge pull request #25 from containerscrew/dependabot/cargo/clap-4.5.31 - github-actions[bot]

- 16ee54adb0a9788b809aab044b9b24f6d6c35d48 - Bump clap from 4.5.30 to 4.5.31 - dependabot[bot]

- fde20333ddbf2b0639eb5180ea38271aff943374 - Merge pull request #24 from containerscrew/dependabot/cargo/log-0.4.26 - github-actions[bot]

- 364bc0d988d768b54dba8c88873d492f4b2887a8 - Bump log from 0.4.25 to 0.4.26 - dependabot[bot]

- 9e74813dd8cd7fa9ddf5b469588b774ad6ead4b5 - Merge pull request #23 from containerscrew/dependabot/cargo/anyhow-1.0.96 - github-actions[bot]

- 5932cb2e51861594809eaa193bcac0fd223ac6c4 - Bump anyhow from 1.0.95 to 1.0.96 - dependabot[bot]

- 2eaadf489e87aae93530e27b262766226d6e3915 - Merge pull request #22 from containerscrew/dependabot/cargo/tempfile-3.17.1 - github-actions[bot]

- ec0913a778930ed9f4ecb4a3ed0a2edf8b996d01 - Merge pull request #21 from containerscrew/dependabot/cargo/clap-4.5.30 - github-actions[bot]

- 5ffa70bba8b9b2af13b43e92f227160d0e53cf6c - Bump tempfile from 3.16.0 to 3.17.1 - dependabot[bot]

- 4a3ea1b4e445beffcbe3371822ea8fa0c92e7df4 - Bump clap from 4.5.29 to 4.5.30 - dependabot[bot]

- 9de940f113bf4a2949e1befc14569e6ee1066583 - Cargo clippy - containerscrew

- 803f30cf6fc9cb1d0fec89e05aa086bea6728190 - Cargo clippy - containerscrew

- 40d8e38b0d77f0cf9073ff89012ae91754153151 - Cargo fmt - containerscrew

- 3efc603190e3f534e4790d16270385ee34bddf4a - Remove pid and command from tc - containerscrew

- b47bdf0477bc94e8ed28f26df475c13d2840e0cc - fmt - containerscrew

- 00f91c07e16f88554113dfab98299c4495dec54b - Improve logger: track active connections storing pid:dest_ip - containerscrew

- 8deeabc926da0abf499660edf5fb84fda7248415 - Merge pull request #20 from containerscrew/dependabot/cargo/clap-4.5.29 - github-actions[bot]

- f30d3f69773a6f43ca2920b59ec193dde86524e4 - Bump clap from 4.5.28 to 4.5.29 - dependabot[bot]

- bc3a912fe0828468f9f7e58db8145fb9c7d543f6 - Comment/delete unused code/files - containerscrew

- e5e9b984161914cb8d4814669db9b6d7dd97232a - Update todo.md - containerscrew

- e9e3f30508e48b1438d22505ad923470c079953e - Delete unused Makefile targets - containerscrew

- e6e85b9f7fec70f20cf85673238218b21f519d3c - Delete unused files/folder - containerscrew

- a5077ca7937e7e90a85e2ebf634c804bc4195785 - Update deps - containerscrew

- 8b6da47456fe9705782841f0e2d9d013a5ab5361 - Add ipgeoinfo - containerscrew

- d352bf0e5ad7f53b623edcfc67d08908a4ea4bd7 - Change todo - containerscrew

- dd1cd2450d8c264c27ac43b0958eb6dd580dcfdb - Add .vscode folder - containerscrew

- 3b35409e9332cd924aaeeaa0e33533e2c242183e - Fmt - containerscrew

- 8f690ee8dfe46978ac2e0d7ef53a948aae3a08cb - Cargo fmt - containerscrew

- 845d498269a54abe9e761724d8a2c67921507520 - Log every N seconds & change perfeventarray and implement ringbuffer - containerscrew

- 36dd3f415218211fb7c0aa25efcbaeb54b6da5bc - Update README.md - containerscrew

- 22032db022b6baa22c7ad7d2e2efb1e7137ead12 - Implement wireguard iface tunnel monitoring - containerscrew

- efe7464b67a5ac38ead2c53b149fe8f6ec4fac40 - Wip: implement non ethernet packets - containerscrew

- b74e25477cd17274963e49b206b637f0d1242744 - Wip - containerscrew

- 1fff7b78586c621bdd7c97fe27dc92ff1ed6d05e - Disable udp logging by default - containerscrew

- fce21bfb0fe1f06da3c86eb8502923f4fb316665 - Add pid and command track to the log - containerscrew

- 9b638dc1d2652e05073a4f47c3f84003924424d9 - Creating new config bpf_map for tc - containerscrew

- f22f33712e1c792e63127af48d0f88336aa03d36 - Pipeline docker build: only when tags - containerscrew

- 8745ef84fac3112b4766348f190daa600e372654 - Change logger timestamp - containerscrew

- 25c6a3d90478e128d1f01effff96cf7103816974 - Update README.md - containerscrew

- d50980f9f906f9f5ce79f70b96be5169f1b937cf - Update README.md - containerscrew

- 208cc443c71216ff94c9f2b4094f9b3bb4f93ad3 - Update README.md - containerscrew

- 495298e9535b6ef40a880585b5eaa71f652e64d8 - Update README.md - containerscrew

- cafadeef3a8ed8068920c76ad5fb84ebdeec0ae4 - Update README.md - containerscrew

- fc3820f5e6705b7c754505acfab251f22f03546c - Refactoring ingress/egress with TC - containerscrew

- 1e2baf9dfeeb65d65d4a28433257284b5b143c1a - Creating the cli with clap - containerscrew

- 60f52ba02c1bec9149b7416902d4ddc18a40e3cd - Comment test pipeline - containerscrew

- ef7e7d23c47d572f16585bde94efe76334bc3907 - Update README.md - containerscrew

- 27291ec61559f65b1bd515164816ca79c6414aae - Update README.md - containerscrew

- c67826ba466c1a36662407b1ba7c19158bc06318 - Update nflux.toml - containerscrew

- e2cd0ebb8ad9c81438fdd817c5e9d92dafc8b649 - Merge pull request #19 from containerscrew/dependabot/cargo/toml-0.8.20 - github-actions[bot]

- 89fdc870dc881ebdf2fd7fdf818a44bfd04a486e - Bump toml from 0.8.19 to 0.8.20 - dependabot[bot]

- 03961ce14ea711336089780e6aa31f899c6a8bc3 - Merge pull request #18 from containerscrew/dependabot/cargo/bytes-1.10.0 - github-actions[bot]

- ed6d3919834ddb2c05c56506e68eb3140000fa44 - Bump bytes from 1.9.0 to 1.10.0 - dependabot[bot]

- 9879afe564fb197b7965ba74c16b35d33b981bb7 - Reorganize code - containerscrew

- 20bc37bc2a6c34ba94dec7e5c6ef9649a8f51113 - Update README.md - daniels

- ee4272beb91da49d98eb03871869b7d35eb878b3 - Update README - containerscrew

- 3e4b5cfdb5d543b642025b569a9d61c6a98c2226 - Merge pull request #17 from containerscrew/dependabot/cargo/tempfile-3.16.0 - github-actions[bot]

- d355b8dea6e135f3f997c28fd0c0558fff5b3a8f - Bump tempfile from 3.15.0 to 3.16.0 - dependabot[bot]

- dbdcaa9900b8585526bf2977e1c6d28ec4580ba2 - Wip: prometheus metrics - containerscrew

- fe10ab390c9f1a7f9469693e01c1a319c1a8c3c3 - Wip: modify prometheus metrics counters - containerscrew

- 7473534c6791d85bc1eebd7fb0a75b253382dee1 - Wip: implement prometheus metrics - containerscrew

- af539b4bba97202e915a5d4c430c800c24e58890 - Fix logger for src and dest ip - containerscrew

- 0fe5d54a6a40c5c5756f17a02d6e8410d2bd6e59 - Improve debian package - containerscrew

- 2c0fc47a8c2c13160ef0befa3d172f0a421d4e82 - Fix nflux/src/egress.rs - containerscrew

- 4c9c7f749f1d9f8694b914550f523205677d8746 - Improve egress/ingress traffic control - containerscrew

- f524d57edbe2653489bc96cebe638d681808fca3 - cargo fmt - containerscrew

- c9221fec89c33a4006a8b35ab8ceb84674bcca65 - Remove pid tracking for traffic control connections & organize code - containerscrew

- 45777b451457ba71d4b7abeedbe39c5f28b3aaf1 - Merge pull request #16 from containerscrew/dependabot/cargo/cargo_metadata-0.19.1 - github-actions[bot]

- 19d0b9eb3d4adcf78841f4cbd669b5d8a801713e - Bump cargo_metadata from 0.18.1 to 0.19.1 - dependabot[bot]

- 31d06bb15eca34738169900585927b34e5d95a26 - Wip: docker builds - containerscrew

- a1fcc76e7e8e7b9ca6478b8a377c5c601e468bc5 - Wip: egress connection - containerscrew

- 3b6243dcbee6b6b94cc213a9a1cf93e0eabdd1fd - Modify bpf_get_current_pid func - containerscrew

- b6ce832ef4c3c0913a5a0bd2dcd65ac7b689a83f - Remove .cargo/ root folder - containerscrew

- e9be3ddd1f5cbf265ecbc256a773bb69335f7a34 - Remove xtask folder - containerscrew

- 94787dc9513c80a4ca72370a52e62c6c27f4048f - Remove xtask for binary compilation - containerscrew

- 463a7392e1c27903a34dd2f987f554a81e86095b - Update README.md - containerscrew

- 525843e1b7722914a43754fed0bd59d8e491d29e - Wip: egress connection - containerscrew

- 95437e6f245a7105079a8bb4da743bcca33ceba2 - Wip: egress connection - containerscrew

- 1478f7d4bb83d14ad8f771ed9eeb2574d0486a28 - Delete unused files, log when ipv6 packets in egress filter - containerscrew

- eba1f1c6c867f61563033f38a52b2e6e1d8c7057 - Stable: egress sniff for physical and virtual interfaces - containerscrew

- e5aad555414966ec009d5e79efa216e8fae3bba2 - Merge pull request #15 from containerscrew/dependabot/cargo/log-0.4.25 - github-actions[bot]

- 6639abbb8e704f0c0fac60dbba9dff6b9cbcb53e - Bump log from 0.4.22 to 0.4.25 - dependabot[bot]

- c84ec6a9f74f1b438c1e31a058f7da7fdfb42ee7 - Fix get_process_name func - containerscrew

- 93600ab3433f909c4ff2b9734035e95c4bd48164 - Wip: egress connections - containerscrew

- 16435d5b3f91551f5e81c8e5fd89a88cbf789463 - Wip: egress connections - containerscrew

- 7f804ddbcf31c3a3292ea458e5c0ab139730361c - Wip - containerscrew

- 1a92398edba9b9275610e6043e495aca866d37c7 - Working with egress connections - containerscrew

- 4310533688092dd6f8a15c60635b7b353d306a99 - Wip - containerscrew

- a9a0b322b6ed3bf21ee571f00e7b28de6a4b6e3a - Logging pid for every egress packet - containerscrew

- e60a8f506de550f8cad0884f7284b177469f1a85 - Merge pull request #14 from containerscrew/dependabot/cargo/clap-4.5.26 - github-actions[bot]

- f0f8e4989fcf47cefd3c06f5f2727df7411708ec - Bump clap from 4.5.24 to 4.5.26 - dependabot[bot]

- 94b36d7ea1c74aaf5d76cc26d7d85bca97bcea9e - Working with egress log - containerscrew

- ff653b0df5298360fd16a831e6dde60d3bd82af0 - Merge pull request #13 from containerscrew/dependabot/cargo/tokio-1.43.0 - github-actions[bot]

- 0fff33cadade93406c828324fb321b3a36c50bd6 - Bump tokio from 1.42.0 to 1.43.0 - dependabot[bot]

- c01429290e9e936bb456d56a89a2f212d8600c16 - Merge pull request #12 from containerscrew/dependabot/cargo/clap-4.5.24 - github-actions[bot]

- 03ce305b56718aed5557e2f376971dd28710d1ce - Bump clap from 4.5.23 to 4.5.24 - dependabot[bot]

- b70f932bf9631818dbf749fabc2dbeb47ca6444a - Merge pull request #11 from containerscrew/dependabot/cargo/tempfile-3.15.0 - github-actions[bot]

- 555423299863d7e2502619415faa084365fe41da - Bump tempfile from 3.14.0 to 3.15.0 - dependabot[bot]

- a39b429c2be2972dfd794965c589b778603327ad - Make podman compatible - containerscrew

- c1679e7b5655a91f6a201e8502277548dfb7b211 - Update order for TCP IpProto in firewall ebpf code - containerscrew

- 158ce78a8ce1f4a258422d18665d3bce1dee28c6 - Edit log in xdp_firewall events - containerscrew

- 10706f1451ebd6b6a8e750419eeb8f592512e566 - First initial working version - containerscrew

- c9eecfee847f5966b38c488e9f7a48643d28333b - Merge pull request #10 from containerscrew/dependabot/cargo/serde-1.0.217 - github-actions[bot]

- 0084212e8032cf3a7c97fc2cf0cfde01345cfb6a - Bump serde from 1.0.216 to 1.0.217 - dependabot[bot]

- 5b7728a3d2ab8ef1aee4d58c87855f50a8d8c70a - Organice code - containerscrew

- d194b175220cfc024f00db076ca1faceee368664 - Support multiple interfaces - containerscrew

- e632b3457b94a6c424bbfcffecb05e65c6e7c9d5 - fmt - containerscrew

- 9c44581eb0115df610bacf79468324061db4972b - Refactor nflux.toml & implement egress monitoring for vpn virtual interfaces - containerscrew

- ada396863db657ecb3ce7d037a075de31cf412f2 - Merge pull request #9 from containerscrew/dependabot/cargo/anyhow-1.0.95 - github-actions[bot]

- 1ef538297c593aa674c1566a8140b6ceec175aee - Bump anyhow from 1.0.94 to 1.0.95 - dependabot[bot]

- 5d64c19ecbd9ba1a8c95d9bb456f8b729b1ff19f - Merge pull request #8 from containerscrew/dependabot/cargo/env_logger-0.11.6 - github-actions[bot]

- ca1a9feec30de682e72bab8da48fbb9275de01f5 - Bump env_logger from 0.11.5 to 0.11.6 - dependabot[bot]

- a2ebcdb0b62c7bdbbb30b9451bbd5a425eb51386 - Merge pull request #7 from containerscrew/dependabot/cargo/libc-0.2.169 - github-actions[bot]

- 894e6832878239df48de9c35107101eaaec07282 - Bump libc from 0.2.168 to 0.2.169 - dependabot[bot]

- b2debad491e6ee247655b2dfcbe892ffab25dd3d - Some performance in nflux running inside a container - containerscrew

- 1ae90481404573be03ea2fb5b778942117534d13 - Fix config tests - containerscrew

- 7258fb6e78a4e0865972b416a73e1a6aa2801d7d - Working with TCP state flags - containerscrew

- 7b47049d481108355ebfed1030e8570e8ba018b9 - Refactoring ipv4 tcp ingress proto - containerscrew

- 5b8b045041d977427f367d4a655fe4555bc30de4 - Wip - containerscrew

- b7fa980edcd25ffc90aa4d5878270ed57566338d - Wip - containerscrew

- d4729f4e88457c2d2de89da314fde43590f2b65e - Working with ingress traffic - containerscrew

- 6c9a15d752c6d0db8e41a2d7d9ca50e6136ffc0d - Improve tcp states - containerscrew

- a40c78424eef18486894807ce0987a2949bb5dfb - Refactor ipv4 traffic - containerscrew

- d0c5fe5c258c91a7f325af04e520d019d4666b73 - fmt - containerscrew

- 0ec24c1bcbe508cc647417c501ebcd4a017a3235 - Merge pull request #5 from containerscrew/dependabot/cargo/libc-0.2.168 - daniels

- 918da4bc4e0992465fe87db81b9495dfc7ebf65d - Merge pull request #6 from containerscrew/dependabot/cargo/serde-1.0.216 - daniels

- 0972bc198d4459c1b6cda32573c9b59b5ef08d0f - Fix version of github action - containerscrew

- a28d7c79639e8ab6d259019abd2428a543dc825e - Bump serde from 1.0.215 to 1.0.216 - dependabot[bot]

- d8cf108c0d95c5dbafaf21f717e923ac19aff2e5 - Bump libc from 0.2.167 to 0.2.168 - dependabot[bot]

- 76d38970f8a2a51c5ebdc632333b571d79942d9e - Update doc - containerscrew

- 0e012a23b0651a5da3737b7b2b1d0ac380dab305 - Update doc - containerscrew

- 5da607f203d6602850baaa28c674fc7ca07ff4e4 - Update doc - containerscrew

- 4b5fc09bea7e5f12f971bfeb310b04362eae2634 - Update doc - containerscrew

- 5055f2643377e4d966f5d086772ca81acac21d58 - Update doc - containerscrew

- 142a5ef6ca8b316126d5f0d08b332182f3b1581d - Working with TC egress implementation - containerscrew

- a5274b6992a85dcf7b9c5e34651c2ceea4cd34ab - Implement egress traffic monitoring - containerscrew

- 5996ae8e2d99b004254bb9b8431875b5783ec4ad - Adding TC egress control - containerscrew

- 02b2eb01f1a4267d97480606981d80e36fdf7561 - Change docker-compose file - containerscrew

- 41c1e1b7b412c53fcb7814ff577f132884ce50fc - Change nflux.toml config - containerscrew

- 02a504a182638686f3d2f73b1ba17e67356dc66b - Refactor ebpf code & logging - containerscrew

- a4e9d4b0abe8fa2a514c7fc18c255f2ec64235a9 - Merge pull request #4 from containerscrew/dependabot/cargo/clap-4.5.23 - daniels

- d2d7cd1f6cf945f7580331698b2ca20b6603c040 - Change auto-merge pipeline - containerscrew

- c220732c30465c22c05c5513537a1e3522973cf4 - Bump clap from 4.5.22 to 4.5.23 - dependabot[bot]

- 3588679978cb12e5c0ff7c5acea364718b08afa3 - Implementing ipv6 - containerscrew

- 2ac083a758c735079bf942096f9fc3b802b952fc - Nflux running in containers - containerscrew

- 84f9699c2a4a82c0732325bbafc78c9b9f6cfca6 - Implement icmp enable/disable - containerscrew

- 04a9632904b727cddc7017c6e4f1ce33607666a6 - Permit tcp syn-ack packets & config testing - containerscrew

- 2c7b483cb3729188653554e812d36ed8dc0b37b4 - Update nflux.toml - containerscrew

- bccd2369f1ff8098c85fb547622ce7570d22674c - Fix prefix_len dynamically - containerscrew

- ce4f18586bc681930bb64aef4f8011244dae9dea - Change nflux.toml - containerscrew

- eaffd95ae6f180589ee035ef35c93cb2e917cacc - Change pre-commit - containerscrew

- 402797fb545cd089781bfea6262ac9839cda2cc2 - fmt - containerscrew

- 29320f907b623152e7a0a06f4d6cf400a42e7bd3 - Refactor tests - containerscrew

- d62b95befca6ec8e2728c08219ba0df1a596fbb5 - Merge pull request #2 from containerscrew/dependabot/cargo/anyhow-1.0.94 - daniels

- ddf3b17268402961f81d4aea837e9700eb2db750 - Merge pull request #3 from containerscrew/dependabot/cargo/clap-4.5.22 - daniels

- e6eb610097da68b72b3f12f42ed09ae7bbf4efbe - Change config format & some tests - containerscrew

- 26d1eb384a3cef1ee6b3aebba02b27f4c22925f4 - Bump clap from 4.5.21 to 4.5.22 - dependabot[bot]

- 6b8d3a876791c8f3fcc6fc8909ea55bc27414684 - wip - containerscrew

- 10e46cc8a431687d58f25ee9a6a7f469435aa3b0 - Bump anyhow from 1.0.93 to 1.0.94 - dependabot[bot]

- f29651ff18e606e6faaec2c2caf5f629694c609f - Working with rule implementation - containerscrew

- 83f856b118c23d618849e3052a1647741ca590e4 - Populating ipv4 rules - containerscrew

- 706e4ebe6c3c0a50ad7316b781fe9618bb9b7a54 - Working with firewall.rules config structure - containerscrew

- 1e57abde34d2c7a5c8652cec7ef8f0309eb61bde - Modify pipeline tests.yml - containerscrew

- 2bc88cf878926039ad3622a23343e5ab46a492c8 - Add new pipelines & some basic tests - containerscrew

- 7f92d243fa2d236c3f3dc0c3a7d40648e27780c5 - Feat: some print messagess - containerscrew

- fc12598153412c26789b035ea71d92861dd4e743 - Fix denied syn packets - containerscrew

- f832eeabe6e6ffd01e1e6a960ccbaaa2efcf0691 - Update README.md - containerscrew

- 28c4c93850d20f3ed720deeb9363a3ae59be5a8b - Update README.md - containerscrew

- 49f009f6f72816a5ed4f85de4a7c46ffe243b7ff - Update README.md - containerscrew

- 04c8aa0b4371989190ac13db422acb3707a2560d - Wip - containerscrew

- 7539347b2a07d6f5a3e8196bb4cfec1e65f639c6 - Rollback nflux.conf to nflux.toml file - containerscrew

- bbff98648f1542202de6354b360ec6bdf2c579c5 - Implement new BPF_MAP_ARRAY for global app config & fix dpkg installation - containerscrew

- 5a5d58ad184d47487266da848ed4dd420f787181 - Wip - containerscrew

- 6bed3fa695414ac1287c38988035b02f2570cab6 - Implementing AsyncPerfEventArrayBuffer - containerscrew

- 8b23a8fb29c63025a1e9e7c3efb45379cccd0417 - Refactor logging - containerscrew

- 0cd0b095303c71200c56ad448f41d1b7fd06d909 - Update README - containerscrew

- 8b12d06dba1f5408870464dd46f55732a195c644 - First stable version - containerscrew

- 2bcf77d1eeec994811000fef2843e829496fe33d - Add logging type (text,json) - containerscrew

- 47a5abead91e8ed3f9c2279c38e5173f14b261da - Update README & wip ebpf program - containerscrew

- 4727105f6c1350551ce446b58cdf9254c961e713 - Create new testing tcp/udp services with compose - containerscrew

- 9df936626708f1d7e277fe4de23b60491c88a5a7 - Using perfbuff - containerscrew

- d4e8be191b1ee36c05c49e47a0b41271b9261c2c - pre-commit - containerscrew

- bc0e24283023a2f827c678317dc4e6917a5a626f - Add image examples - containerscrew

- 1f7c63b2b5f23e680c11c5bc2d22015980f8bb27 - Fix incoming connection - containerscrew

- 75ab4dd749885d61bc457ae0f300df23f1c9bf53 - Change cli description - containerscrew

- 0a1821ce9b8938ecc95b015e5553f986e2a80e92 - Rename the project to nflux - containerscrew

- 1d9c526f3b00e33da367756a2213ef13f1ce20af - Implement ipv4 access - containerscrew

- b4c8bdf8036dfce052f25fd5d06de613f764b54f - Reduce logging info for same ip - containerscrew

- 9eafbe0b92bd9ce4a738b44d09beb3a23a25c0da - Update doc - containerscrew

- 8ed77b2aa3d06e89a0c87ad51c8901e952c4f62e - Implement config.toml and add new doc - containerscrew

- 227b74f942ab0097bdda1442c5f35853e903ef0f - First approach: deny all incoming traffic and allow syn-ack - containerscrew

- 3aa007f324c2c17b5b33dd992eee5aaee5b723b6 - Initial commit - containerscrew

#### Refactoring
- 4a74f3b3580d740a2ed5042a89e42f1d96fdaf7a - **(TcEvent)** allow Ipv4 and Ipv6 in TcEvent struct & refactor handle_packet code) - containerscrew
- 288506a735c7d3a056cf44a288c5a762daec4e08 - **(cli)** about message - containerscrew
- 64c79fde410c667e08d99f5f1f1220080b559ee2 - **(cli)** help message & cargo fmt - containerscrew
- edaf643931fd95a6aee029daaad226eb4624cf67 - **(cli)** help message & cargo fmt - containerscrew
- a3a18071162824c46b6cee540d16e4800cce473b - **(cli)** help message & cargo fmt - containerscrew
- 4e64a0fbca26fe0e2b002fb0196afc7b313bb141 - **(ebpf-logger)** comment ebpf-logger - containerscrew
- 88c5f2f0724167bae061f30da67a33452bf3b058 - add changelog - containerscrew

- 139a09d0cbcd8224fd5c517f125bdeb4cfab25a2 - add changelog - containerscrew

- cafcbe89a5871e800f05b0a9aa0d9eb6928282bc - changelog generation - containerscrew

- 90265646e0f56649bd1fbdbef280b0b9449020f4 - rewrite git history - containerscrew

- c96e479fbbaaddb10c6d796810ebb3bee7d1c915 - rewrite git history - containerscrew

- f152512140612b0240d9132ffbac361636fad1be - update edition in Cargo.toml - containerscrew

- 0f140f963b11a99cd82c984389a822fbfe38f030 - delete changelog.md - containerscrew

- 0a3ed345799d025298b61538fced874bc782d9f9 - new nflux version with ebpf code refactor - containerscrew

- 4b8416de81b4bd9eedaaf3883d1beeb3982a4d81 - cog and pre-commit script - containerscrew

- e3e40aadfad0cbead3333a05d1f3a01e36c95250 - cog and pre-commit script - containerscrew

- 63c5ea1a527118a08fd060f3cdda46005a3b9d23 - cog and pre-commit script - containerscrew

- 689810ab2c4ae4bff863522cfb8ec91d7e749fb1 - cog and pre-commit script - containerscrew

- 9c9b9d908f7774fddfcfefba1ec5ad93157d8001 - edit cog.toml - containerscrew

- 3cdafefd63dea2db43fb3708264e73d53a0e1a60 - refactor - containerscrew

- d7574710e9d539860a01a2ca8adaeced23dc6a35 - refactoring - containerscrew

- bfc377203d1d8b4f30fd2c69b3d6a1926689b51f - refactoring tc ebpf code & implement ARP packet sniffing - containerscrew

- 533b02a6cb3c3f4a40ac96748e00ef7050b5d90a - refactoring ebpf dpkt code - containerscrew

- 5b4b0788714d8962f9b34c2d800ab426c6d4c119 - refactoring - containerscrew

- e872ba0e9168a3103593e2d6d82598ade541e04b - refactoring tc ebpf code & implement ARP packet sniffing - containerscrew

- de6eef2a38a814c8e960d3d08d595ff071659717 - refactoring ebpf code folder structure - containerscrew

- b53f41e692e38204b682f33c87d48cb52589e0b7 - unused imports and utils::is_ipv4_private_address - containerscrew

- caf2b3793b7a946b8243252e3b6fc534004b98ae - cgroup skb attach type - containerscrew

- a80d96e5fc9b21c2f0aaaa9e7210cbc6f2916202 - generic process_ring_buffer function - containerscrew

- 64ce6a1b5ec79fdfb6edbf9a8401a71b69d33f61 - renaming folder structure - containerscrew

- c20ca34b0954ef25cf1dfcde126e40f3fed82eed - pre-commit and cargo package release metadata - containerscrew

- 1b18dec3097bdb1a9c7357b9d2197aad80165a4f - rename some functions & delete comments in nflux/src/main.rs - containerscrew
