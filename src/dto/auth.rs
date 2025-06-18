use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthResponse {
    pub valid: bool,
    pub roles: Option<Vec<String>>,
    pub permissions: Option<Vec<String>>,
}