use std::sync::Arc;
use tracing_subscriber::prelude::*;

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_appender::non_blocking::WorkerGuard;

use crate::config::Config;

pub struct Logger {
    _file_guard: Option<WorkerGuard>, // Necesario para no perder logs en archivos
}

impl Logger {
    pub fn init(config: Arc<Config>) -> Self {
        let base_log_level = config.log_level.clone();
        let mut layers = Vec::new();

        let filter = EnvFilter::try_new(config.log_level.clone()).unwrap_or_else(|_| EnvFilter::new("info"));

        // Consola
        if config.log_to_console {
            let console_layer = fmt::layer()
                .with_target(false)
                .with_filter(filter);

            layers.push(console_layer.boxed());
        }

        // Archivo
        let mut file_guard = None;
        if config.log_to_file {
            let file_appender = tracing_appender::rolling::daily(config.log_file_path.clone(), "styx.log");

            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

            file_guard = Some(guard);

            let file_layer = if config.log_format_json {
        fmt::layer()
            .json()
            .with_writer(non_blocking)
            .with_filter(EnvFilter::try_new(base_log_level.clone()).unwrap_or_else(|_| EnvFilter::new("info")))
            .boxed()
    } else {
        fmt::layer()
            .with_writer(non_blocking)
            .with_filter(EnvFilter::try_new(base_log_level.clone()).unwrap_or_else(|_| EnvFilter::new("info")))
            .boxed()
    };

            layers.push(file_layer.boxed());
        }

        // Componer el subscriber con todos los layers activos
        tracing_subscriber::registry()
            .with(layers)
            .init();

        Self {
            _file_guard: file_guard,
        }
    }
}
