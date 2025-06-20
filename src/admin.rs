use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use axum::body::Body;
use axum::extract::Path;
use axum::http::{Response, StatusCode, Method};
use axum::routing::get_service;
use axum::{
    response::Json,
    routing::{delete, get, post, put},
    Extension, Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::Span;

use crate::dto::routes_create::UpdateRouteRequest;
use crate::{
    db::{Database, Route},
    dto::routes_create::CreateRouteRequest,
};
use tower_http::services::ServeDir;

fn on_response_fn<'r, 's>(response: &'r Response<Body>, latency: Duration, _span: &'s Span) {
    tracing::info!("Response: {} ({:?})", response.status(), latency);
} 
pub fn admin_router(db: Arc<Database>) -> Router {
    let webui_dist_path = PathBuf::from("./http"); 
    Router::new()
        .route("/admin/routes", get(list_routes))
        .route("/admin/routes", post(create_route))
        .route("/admin/routes/{id}", put(update_route))
        .route("/admin/routes/{id}", delete(delete_route))
        .layer(Extension(db))
        .layer(CorsLayer::very_permissive())
       
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<Body>| {
                    tracing::info_span!(
                        "HTTP Request",
                        method = %request.method(),
                        uri = %request.uri(),
                    )
                })
                .on_response(on_response_fn),
        )
         .fallback_service(get_service(ServeDir::new(webui_dist_path.clone()).append_index_html_on_directories(true).not_found_service(
            ServeDir::new(PathBuf::from("./http/assets")).append_index_html_on_directories(true)
        )))
}

pub async fn list_routes(
    Extension(db): Extension<Arc<Database>>,
) -> Result<Json<Vec<Route>>, (StatusCode, &'static str)> {
    match db.get_all_routes().await {
        Ok(rs) => Ok(Json(rs)),
        Err(e) => {
            tracing::error!("Error en BD {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "No se ha podido conectar con la Base de datos",
            ))
        }
    }
}

pub async fn create_route(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<CreateRouteRequest>,
) -> (StatusCode, &'static str) {
    let result = db
        .insert_route(
            &payload.path_pattern,
            &payload.http_method,
            &payload.target_url,
            payload.required_roles.as_deref(),
            payload.require_permission.as_deref(),
        )
        .await;

    match result {
        Ok(_) => (StatusCode::CREATED, "Ruta creada correctamente"),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error creando la ruta"),
    }
}

pub async fn update_route(
    Path(route_id): Path<i64>,
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<UpdateRouteRequest>,
) -> (StatusCode, &'static str) {
    let result = db
        .update_route(
            route_id,
            &payload.path_pattern,
            &payload.http_method,
            &payload.target_url,
            payload.required_roles.as_deref(),
            payload.require_permission.as_deref(),
        )
        .await;

    match result {
        Ok(updated) => {
            if updated {
                (StatusCode::OK, "Ruta actualizada correctamente")
            } else {
                (StatusCode::NOT_FOUND, "Ruta no encontrada")
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error actualizando la ruta",
        ),
    }
}

pub async fn delete_route(
    Path(route_id): Path<i64>,
    Extension(db): Extension<Arc<Database>>,
) -> (StatusCode, &'static str) {
    let result = db.delete_route(route_id).await;

    match result {
        Ok(deleted) => {
            if deleted {
                (StatusCode::OK, "Ruta eliminada correctamente")
            } else {
                (StatusCode::NOT_FOUND, "Ruta no encontrada")
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error eliminando la ruta",
        ),
    }
}
