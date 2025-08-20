use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use once_cell::sync::Lazy;
use std::time::Instant;

/// Global handle to render Prometheus metrics.
pub static METRICS_HANDLE: Lazy<PrometheusHandle> = Lazy::new(|| {
    PrometheusBuilder::new()
        .install_recorder()
        .expect("failed to install Prometheus recorder")
});

/// Instant marking the start time of the daemon.
pub static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

/// Force initialization of metric components at startup.
pub fn init() {
    Lazy::force(&METRICS_HANDLE);
    Lazy::force(&START_TIME);
}
