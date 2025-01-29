use prometheus::{IntCounterVec, Opts, Registry};
use std::sync::Arc;

pub struct Metrics {
    ingress_connection: IntCounterVec,
    egress_connection: IntCounterVec,
}

impl Metrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let ingress_connection = Opts::new("ingress_connections", "Total number of ingress connections.");
        let egress_connection = Opts::new("egress_connections", "Total number of egress connections.");

        let ingress_connection = IntCounterVec::new(ingress_connection, &["protocol", "src_addr", "dst_addr", "src_port", "dst_port"])
            .expect("Failed to create ingress connection counter");

        let egress_connection = IntCounterVec::new(egress_connection, &["protocol", "src_addr", "dst_addr", "src_port", "dst_port"]).expect("Failed to create egress connection counter");

        registry.register(Box::new(ingress_connection.clone())).expect("Failed to register TCP metric");
        registry.register(Box::new(egress_connection.clone())).expect("Failed to register UDP metric");

        Arc::new(Self {
            ingress_connection,
            egress_connection,
        })
    }

    pub fn track_ingress_event(&self, protocol: &str, src_addr: &str, dst_addr: &str, src_port: &str, dst_port: &str) {
        self.ingress_connection.with_label_values(&[protocol, src_addr, dst_addr, src_port, dst_port]).inc();
    }

    pub fn track_egress_event(&self, protocol: &str, src_addr: &str, dst_addr: &str, src_port: &str, dst_port: &str) {
        self.egress_connection.with_label_values(&[protocol, src_addr, dst_addr, src_port, dst_port]).inc();
    }
}
