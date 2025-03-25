<!-- START OF TOC !DO NOT EDIT THIS CONTENT MANUALLY-->
**Table of Contents**  *generated with [mtoc](https://github.com/containerscrew/mtoc)*
- [What is ebpf?](#what-is-ebpf)
<!-- END OF TOC -->
# What is ebpf?

_(Small intro)_

eBPF, which stands for [`Extended Berkeley Packet Filter`](https://ebpf.io/), is a revolutionary technology that allows for the dynamic insertion of small programs into various points in the kernel without requiring recompilation or modification of the kernel itself. These programs are executed in a restricted virtual machine (VM) environment directly within the kernel, providing the ability to intercept and modify data as it traverses the system. eBPF is utilized for tracing network packets, implementing firewall and network filtering programs, security software, and facilitating system monitoring.

![ebpf-overview](./img/ebpf-overview.png)

*Source: [ebpf.io](https://ebpf.io/what-is-ebpf/)*

If you'd like to learn more about eBPF, here are some online resources and favorite books to help you continue learning:

- [ebpf.io](https://ebpf.io/)
- [eBPF official documentary](https://www.youtube.com/watch?v=Wb_vD3XZYOA&t=294s)
- [Interesting tutorials by eunomia-bpf](https://eunomia.dev/)
- [Learning eBPF by Liz Rice (Book)](https://isovalent.com/books/learning-ebpf/)
- [BCC project with a lot of useful tools and examples](https://github.com/iovisor/bcc)
- [Linux Observability with BPF by David Calavera, Lorenzo Fontana (Book)](https://www.oreilly.com/library/view/linux-observability-with/9781492050193/)

> There are many more interesting links and articles on the internet as the community grows.
