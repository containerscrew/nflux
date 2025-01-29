use prometheus::{IntCounterVec, Opts, Registry, labels};
use std::sync::Arc;

pub struct Metrics {
    tcp_connections: IntCounterVec,
    udp_connections: IntCounterVec,
}

impl Metrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let tcp_opts = Opts::new("tcp_connections_total", "Total number of TCP connections.");
        let udp_opts = Opts::new("udp_connections_total", "Total number of UDP connections.");

        let tcp_connections = IntCounterVec::new(tcp_opts, &["src_addr", "dst_addr", "src_port", "dst_port", "comm"])
            .expect("Failed to create TCP counter");
        let udp_connections = IntCounterVec::new(udp_opts, &["src_addr", "dst_addr", "src_port", "dst_port", "comm"])
            .expect("Failed to create UDP counter");

        registry.register(Box::new(tcp_connections.clone())).expect("Failed to register TCP metric");
        registry.register(Box::new(udp_connections.clone())).expect("Failed to register UDP metric");

        Arc::new(Self {
            tcp_connections,
            udp_connections,
        })
    }

    pub fn track_tcp_event(&self, src_addr: &str, dst_addr: &str, src_port: &str, dst_port: &str, comm: &str) {
        self.tcp_connections.with_label_values(&[src_addr, dst_addr, src_port, dst_port, comm]).inc();
    }

    pub fn track_udp_event(&self, src_addr: &str, dst_addr: &str, src_port: &str, dst_port: &str, comm: &str) {
        self.udp_connections.with_label_values(&[src_addr, dst_addr, src_port, dst_port, comm]).inc();
    }
}
