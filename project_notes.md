Everything I’ve learned about eBPF, aya-rs, and Rust, shared in my own tool for network traffic monitoring. It includes:

* Network traffic monitoring at L2/L3 using traffic control

* Linux dropped packets monitoring using the kernel tracepoint → tracepoint/skb/kfree_skb

* JSON log mode to capture data and build dashboards (Grafana with Promtail, Fluent Bit, etc.)

* In progress: capturing TCP states, RTT, and other improvements as they come to mind.

I still have many concepts left to understand. Truly grasping how the kernel and everything under the hood works is not easy, but it gives you superpowers. :)))))
