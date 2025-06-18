use std::sync::Arc;

use crate::{config::Config, db::Database, http_client::HttpClient, metrics::Metrics, proxy_core::auth::AuthClient, telemetry::Telemetry};


#[derive(Clone)]
pub struct AppState {
     pub db: Arc<Database>,
    pub client: Arc<HttpClient>,
    pub telemetry: Arc<Telemetry>,
    pub metrics: Arc<Metrics>,
    pub auth_client: Arc<AuthClient>,
    pub config: Arc<Config>,
}