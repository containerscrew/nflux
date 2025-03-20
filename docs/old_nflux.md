<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [OLD nflux](#old-nflux)
<!-- END OF TOC -->
# OLD nflux

> [!NOTE]
> [In this branch](https://github.com/containerscrew/nflux/tree/old-20250206) I tried to implement firewall functions using XDP, which I have now removed due to complexity given my initial knowledge of eBPF and control of TCP states (syn/ack/rst...etc) in ipv4, ipv6, among other problems. I will change the tool to be a simple cli to monitor eggress/ingress traffic using TC. As a first feature to learn how to use eBPF with Aya-rs, it is enough for someone who is learning.
