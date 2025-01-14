use metrics_exporter_prometheus::PrometheusBuilder;
use tracing::info;

pub fn metrics_init() {
    info!("metrics init");
    PrometheusBuilder::new().install().unwrap();
}
