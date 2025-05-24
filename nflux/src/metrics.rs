use std::net::SocketAddr;
use std::sync::Arc;
use axum::{extract::State, routing::get, Router};
use prometheus::{Encoder, IntCounterVec, Opts, Registry, TextEncoder};
use tracing::info;

pub struct Metrics {
    pub connection_counter: IntCounterVec,
}

impl Metrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        let connection_opts = Opts::new("connections", "Track new network connections");

        let connection_counter = IntCounterVec::new(
            connection_opts,
            &[
                "dir",
                "type",
                "protocol",
                "total_len",
                "ttl",
                "src_ip",
                "dst_ip",
                "src_port",
                "dst_port",
                "src_mac",
                "dst_mac",
            ],
        )
        .expect("Failed to create connection counter");

        registry
            .register(Box::new(connection_counter.clone()))
            .expect("Failed to register metric");

        Arc::new(Self { connection_counter })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn track_connection(
        &self,
        direction: &str,
        connection_type: &str,
        protocol: &str,
        total_len: &str,
        ttl: &str,
        src_ip: &str,
        dst_ip: &str,
        src_port: &str,
        dst_port: &str,
        src_mac: &str,
        dst_mac: &str,
    ) {
        self.connection_counter
            .with_label_values(&[
                direction,
                connection_type,
                protocol,
                total_len,
                ttl,
                src_ip,
                dst_ip,
                src_port,
                dst_port,
                src_mac,
                dst_mac,
            ])
            .inc();
    }
}

pub async fn start_api(registry: Arc<Registry>) {
    let app = Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(registry.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Metrics server running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn metrics_handler(State(registry): State<Arc<Registry>>) -> String {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
