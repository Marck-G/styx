
use axum::{extract::State, response::IntoResponse};
use prometheus::{ Encoder, HistogramOpts, HistogramVec, IntCounterVec, Registry, TextEncoder};

use crate::app_state::AppState;


#[derive(Clone)]
pub struct Metrics {
    pub registry: Registry,
    pub request_counter: IntCounterVec,
    pub error_counter: IntCounterVec,
    pub success_counter: IntCounterVec,
    pub response_time_histogram: HistogramVec
}

impl Metrics{
    pub fn new () -> Self {
        let registry = Registry::new();
        let request_counter = IntCounterVec::new(
            prometheus::Opts::new("http_requests_total", "Total HTTP Request"),
            &["method", "path"],).unwrap();
        let error_counter = IntCounterVec::new(
            prometheus::Opts::new("http_errors_total", "Total HTTP Request"),
            &["method", "path"],).unwrap();
        let success_counter = IntCounterVec::new(
            prometheus::Opts::new("http_success_total", "Total HTTP Successful Responses"),
            &["method", "path"],
        ).unwrap();
        let response_time_histogram = HistogramVec::new(
            HistogramOpts::new("http_response_time_seconds", "HTTP Response Time in seconds"),
            &["method", "path"],
        ).unwrap();
        registry.register(Box::new(request_counter.clone())).unwrap();
        registry.register(Box::new(error_counter.clone())).unwrap();
        registry.register(Box::new(success_counter.clone())).unwrap();
        registry.register(Box::new(response_time_histogram.clone())).unwrap();
        Self{
            registry,
            request_counter,
            error_counter,
            success_counter,
            response_time_histogram
        }
    }
}

pub async fn metrics_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = state.metrics.registry.gather();

    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let body = String::from_utf8(buffer).unwrap();
    (axum::http::StatusCode::OK, body)
}