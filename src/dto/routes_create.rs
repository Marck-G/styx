use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRouteRequest {
    pub path_pattern: String,
    pub http_method: String,
    pub target_url: String,
    pub required_roles: Option<String>,
    pub require_permission: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateRouteRequest {
    pub path_pattern: String,
    pub http_method: String,
    pub target_url: String,
    pub required_roles: Option<String>,
    pub require_permission: Option<String>,
}