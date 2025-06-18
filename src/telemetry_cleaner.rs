use std::sync::Arc;
use std::time::Duration;
use crate::telemetry::Telemetry;
use tracing::info;

pub async fn start_telemetry_cleaner(telemetry: Arc<Telemetry>, interval_secs: u64) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

        loop {
            interval.tick().await;

            telemetry.clear();
            info!("Telemetría limpiada automáticamente.");
        }
    });
}
