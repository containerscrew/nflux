use std::net::SocketAddr;
use prometheus::{Encoder, IntCounterVec, Opts, Registry, TextEncoder};
use std::sync::{Arc, Mutex};
use axum::extract::State;
use axum::Router;
use axum::routing::get;
use tracing::info;

pub struct Metrics {
    ingress_connection: IntCounterVec,
    egress_connection: IntCounterVec,
}

impl Metrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let ingress_connection = Opts::new("ingress_connections", "Register new ingress connections.");
        let egress_connection = Opts::new("egress_connections", "Register new egress connections.");

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

// API function
pub async fn start_api(state: Arc<Mutex<Registry>>) {
    let app = Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Metrics server running at http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        // Don't forget to add `ConnectInfo` if you aren't behind a proxy
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await
        .unwrap();
}

// `/metrics` handler
async fn metrics_handler(State(state): State<Arc<Mutex<Registry>>>) -> String {
    let encoder = TextEncoder::new();
    let registry = state.lock().unwrap();
    let mut buffer = Vec::new();
    encoder.encode(&registry.gather(), &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
