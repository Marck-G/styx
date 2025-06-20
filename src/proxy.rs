use std::{collections::HashMap, io::Read, str::FromStr, time::Instant};

use axum::{
    body::{self, Body},
    extract::{Request, State},
    http::Response,
    response::IntoResponse,
};
use reqwest::{Method, StatusCode};
use url::Url;

use crate::{app_state::AppState, dto::auth::AuthResponse, proxy_core::{auth::AuthError, validator::{AuthChecker, ParseRouteAuth}}};

pub async fn gateway_handler(
    State(state): State<AppState>,
    req: Request<Body>,
) -> impl IntoResponse {
    
    let path = req.uri().path().to_owned();
    let start = Instant::now();
    let method = Method::from_str(req.method().as_str()).unwrap();
    let query = req.uri().query().unwrap_or_else(|| "");
    state.metrics
                    .request_counter.with_label_values(&[&method.clone().as_str(), &path.as_str()]);
    let route = match state.db.find_matching_route(&path, method.as_str()).await {
        Ok(Some(route)) => route,
        Ok(None) => {
            tracing::warn!("Route not found: {}", path);
            return (StatusCode::NOT_FOUND, "No route matched").into_response();
        }
        Err(err) => {
            tracing::error!("Database error: {:?}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "DB error").into_response();
        }
    };
    state.telemetry.increment(route.id.clone());
    // AUTH
    // Mapear los campos de autorización de la ruta
    let route_auth_requirements = ParseRouteAuth::from_route_auth_fields(
        &route.required_roles,
        &route.require_permission,
    );

    let api_token = req.headers()
        .get(state.config.token_header.clone())
        .and_then(|value| value.to_str().ok());

    let mut user_auth_response: Option<AuthResponse> = None;
    // Solo intentamos autenticar si se proporciona un token o si la ruta requiere autenticación
    let should_authenticate = api_token.is_some() || route_auth_requirements.requires_auth();

    if should_authenticate {
        if let Some(token) = api_token {
            match state.auth_client.validate_token(token).await {
                Ok(auth_resp) => {
                    user_auth_response = Some(auth_resp);
                },
                Err(AuthError::InvalidToken) => {
                    tracing::warn!("Invalid API token provided for path: {}", path);
                    return (StatusCode::UNAUTHORIZED, "Invalid API token").into_response();
                },
                Err(e) => {
                    tracing::error!("Auth service error for path {}: {:?}", path, e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, "Authentication service error").into_response();
                }
            }
        } else {
            // La ruta requiere autenticación pero no se proporcionó ningún token
            if route_auth_requirements.requires_auth() {
                tracing::warn!("Authorization required but no token provided for path: {}", path);
                return (StatusCode::UNAUTHORIZED, "Authorization required").into_response();
            }
            // Si la ruta no requiere auth y no hay token, no necesitamos una AuthResponse válida
        }
    }
    // --- Lógica de Autorización usando AuthChecker ---
    if route_auth_requirements.requires_auth() { // Solo aplicamos la lógica si la ruta tiene requisitos
        if let Some(user_auth_resp) = &user_auth_response {
            if !AuthChecker::can_access(&route_auth_requirements, user_auth_resp) {
                // can_access ya maneja si el token no es válido o si faltan roles/permisos
                tracing::warn!("Access denied by AuthChecker for path: {}", path);
                return (StatusCode::FORBIDDEN, "Forbidden: Insufficient permissions or roles").into_response();
            }
        } else {
            // Esto ocurre si route_auth_requirements.requires_auth() es true,
            // pero user_auth_response es None (ej. token no presente o error inicial)
            tracing::warn!("Authorization required but user not authenticated for path: {}", path);
            return (StatusCode::UNAUTHORIZED, "Authorization required").into_response();
        }
    }
    // --- Fin Lógica de Autorización ---

    let incoming_path = path.as_str();
    let route_path_prefix = route.path_pattern.strip_suffix("/*").unwrap_or(&route.path_pattern); // Suponiendo que `route.route_path` es algo como "/google/*"

    let remaining_path = if incoming_path.starts_with(route_path_prefix) {
        &incoming_path[route_path_prefix.len()..]
    } else {
        // Esto no debería ocurrir si `find_matching_route` es correcto,
        // pero es una salvaguarda. Si la ruta de BD es "/" y la petición es "/a/b", remaining_path será "/a/b"
        incoming_path
    };

    // Asegurarse de que `remaining_path` empiece con `/` si no está vacío
    let clean_remaining_path = if !remaining_path.is_empty() && !remaining_path.starts_with('/') {
        format!("./{}?{}", remaining_path, query)
    } else {
        format!(".{}?{}",remaining_path, query)
    };
    
    // Parsear la target_url como una Url base
    let mut target_base_url = match Url::parse(&route.target_url) {
        Ok(url) => url,
        Err(e) => {
            tracing::error!("Error parsing target_url '{}': {:?}", route.target_url, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid target URL configured").into_response();
        }
    };
    tracing::info!("Target Base: {}", target_base_url);
    tracing::info!("Current Path: {}", clean_remaining_path);
    // ensure it's `http://localhost:8080/v1/`.
    if !target_base_url.path().ends_with('/') {
        target_base_url.path_segments_mut()
            .map_err(|_| ()) // Handle case where path is opaque
            .unwrap()
            .push(""); // Add a trailing slash
    }

    // Unir el `remaining_path` usando `join` de la crate `url`
    // Esto maneja automáticamente las barras duplicadas y la final
    let target_url = match target_base_url.join(&clean_remaining_path) {
        Ok(url) => url.to_string(),
        Err(e) => {
            tracing::error!("Error joining URLs: base='{}', path='{}', error='{:?}'", route.target_url, clean_remaining_path, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Error building target URL").into_response();
        }
    };
    

    // Extraer headers a HashMap<String, String>
    let mut headers_map: HashMap<String, String> = HashMap::new();
    for (name, value) in req.headers().iter() {
        if let Ok(value_str) = value.to_str() {
            headers_map.insert(name.to_string(), value_str.to_string());
        }
    }
    // Extraer body completo como String
    let body_bytes = match body::to_bytes(req.into_body(), 100 * 1024 * 1024).await {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("Error leyendo body: {:?}", e);
            return (StatusCode::BAD_REQUEST, "Error leyendo body").into_response();
        }
    };
    let body_string = String::from_utf8_lossy(&body_bytes).to_string();

    // Llamar al cliente HTTP
    let resp = match state.client
        .request(
            method.clone(),
            &target_url,
            Some(headers_map),
            // query,
            Some(body_string),
        )
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            tracing::error!("Error en HTTP client: {:?}", e);
            return (StatusCode::BAD_GATEWAY, "Error contactando backend").into_response();
        }
    };

    // Construir respuesta axum copiando status, headers y body

    let status = resp.status();
    let mut builder = Response::builder().status(status);

    // Copy headers from the `reqwest::Response` to the `axum::Response` builder
    // You should filter out hop-by-hop headers that `axum` or `hyper` will manage
    // automatically (e.g., Connection, Keep-Alive, Proxy-Authenticate, Proxy-Authorization,
    // Te, Trailers, Transfer-Encoding, Upgrade).
    for (key, value) in resp.headers().iter() {
        if !key.as_str().eq_ignore_ascii_case("connection")
            && !key.as_str().eq_ignore_ascii_case("keep-alive")
            && !key.as_str().eq_ignore_ascii_case("proxy-authenticate")
            && !key.as_str().eq_ignore_ascii_case("proxy-authorization")
            && !key.as_str().eq_ignore_ascii_case("te")
            && !key.as_str().eq_ignore_ascii_case("trailers")
            && !key.as_str().eq_ignore_ascii_case("transfer-encoding")
            && !key.as_str().eq_ignore_ascii_case("upgrade")
            && !key.as_str().eq_ignore_ascii_case("content-length") // Axum will set this
        {
            builder = builder.header(key, value);
        }
    }


    // Obtener body como bytes para pasarlo a hyper::Body
    let bytes = match resp.bytes().await {
            Ok(b) => {
        tracing::info!("Response: {} bytes", b.clone().len());                
                b},
        Err(e) => {
            tracing::error!("Error leyendo body respuesta: {:?}", e);
            return (StatusCode::BAD_GATEWAY, "Error leyendo body respuesta").into_response();
        }
    };

    // let body = Body::from(bytes);
    let elapsed = start.elapsed().as_secs_f64();
    state.metrics
        .response_time_histogram
        .with_label_values(&[&method.clone().as_str(), &path.as_str()])
        .observe(elapsed);
    match builder.body(Body::from(bytes)) {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                state.metrics
                    .success_counter
                    .with_label_values(&[method.clone().as_str(), &path])
                    .inc();
            } else if status.is_client_error() || status.is_server_error() {
                state.metrics
                    .error_counter
                    .with_label_values(&[method.clone().as_str(), &path])
                    .inc();
            }
            tracing::info!("Response: {:?}", response.status());
           return response.into_response()
        }
        Err(e) => {
            tracing::error!("Error construyendo response: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error construyendo response",
            )
                .into_response();
        }
    };
}
