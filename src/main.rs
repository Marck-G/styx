use std::{error::Error, net::{SocketAddr}, sync::Arc};

use axum::{routing::get, serve, Router};
use tokio::net::TcpListener;
use axum::http::Method;
use tower_http::cors::CorsLayer;

use crate::{admin::admin_router, app_state::AppState, config::Config, db::Database, http_client::HttpClient, logger::Logger, metrics::Metrics, proxy_core::auth::AuthClient, telemetry::Telemetry};

mod config;
mod http_client;
mod app_state;
mod proxy_core;
mod db;
mod metrics;
mod proxy;
mod telemetry_cleaner;
mod logger;
mod telemetry;
mod dto;
mod admin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::from_lua_file("config.lua".to_string())?;
    let _logger = Logger::init(config.clone().into());
    tracing::info!("Loading database");
    let db_path = &config.database_path;
    let db = Arc::new(Database::new(db_path.clone()).await?) ;
    let client = Arc::new(HttpClient::new(config.http_proxy.timeout.clone()));
    let auth_client = Arc::new(AuthClient::new(config.clone().into()));
    let telemetry = Arc::new(Telemetry::new());
    telemetry_cleaner::start_telemetry_cleaner(telemetry.clone(), config.cache_ttl.clone()).await;

    let metrics = Arc::new(Metrics::new());

    let cors_layer = CorsLayer::new()
        // Permite cualquier origen. Para producción, cambia esto a .allow_origin(Any|YourDomainHere)
        .allow_origin(tower_http::cors::Any)
        // Permite los métodos HTTP comunes (GET, POST, PUT, DELETE, etc.)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS, // OPTIONS es importante para las peticiones preflight de CORS
        ])
        // Permite cualquier header. En producción, especifica solo los necesarios.
        .allow_headers(tower_http::cors::Any);
        // Si quieres permitir credenciales (cookies, headers de autorización)
        // .allow_credentials(true);

    let proxy_app_state = AppState {
        db: db.clone(),
        client,
        telemetry,
        metrics,
        auth_client,
        config: config.clone().into(),
    };

    let proxy = Router::new().route("/prometheus/metrics", get(metrics::metrics_handler))
    .fallback(proxy::gateway_handler)
    .layer(cors_layer)
    .with_state(proxy_app_state);
    let proxy_addr_str = format!("{}:{}", config.http_proxy.address.clone(), config.http_proxy.port.clone());
    let proxy_addr: SocketAddr = proxy_addr_str.parse()?;
    let proxy_listener = TcpListener::bind(&proxy_addr).await?;

    let admin_address = format!("{}:{}", config.admin.address.clone(), config.admin.port.clone());
    tracing::info!("Api GateWay url: http:// {}", admin_address);
    let admin_addr: SocketAddr = admin_address.parse()?;
    let admin_listener = TcpListener::bind(&admin_addr).await?;


    let gateway = tokio::spawn(async move {
        tracing::info!("Iniciado servidor proxy en http://{}", proxy_addr_str);
        serve(proxy_listener ,proxy.into_make_service()).await.unwrap();
    });

    let admin = tokio::spawn(async move {
        tracing::info!("Iniciado servidor admin en http://{}", admin_address);
        serve(admin_listener ,admin_router(db.clone()).into_make_service()).await.unwrap();
    });
    let(_,_) = tokio::join!(gateway, admin);
    
    Ok(())
}
