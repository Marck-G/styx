use crate::dto::auth::AuthResponse;


#[derive(Debug)]
pub struct ParseRouteAuth {
    pub permissions: Option<Vec<String>>,
    pub roles: Option<Vec<String>>,
}
impl ParseRouteAuth {
    /// Convierte los campos `required_roles` y `require_permission` de `Route` a `ParseRouteAuth`.
    /// Asume que los campos son cadenas JSON que representan arrays de strings (ej. `["admin", "user"]`).
    pub fn from_route_auth_fields(
        required_roles_json: &Option<String>,
        require_permission_json: &Option<String>,
    ) -> Self {
        let mut permissions: Option<Vec<String>> = None;
        let mut roles: Option<Vec<String>> = None;

        if let Some(roles_str) = required_roles_json {
            if !roles_str.trim().is_empty() {
                match serde_json::from_str::<Vec<String>>(roles_str) {
                    Ok(parsed_roles) => {
                        if !parsed_roles.is_empty() {
                            roles = Some(parsed_roles);
                        }
                    },
                    Err(e) => {
                        tracing::error!("Error parsing required_roles JSON '{}': {:?}", roles_str, e);
                        // Decide how to handle parsing errors:
                        // - Treat as no roles required for this specific field.
                        // - Or, panic/return an error if invalid JSON should halt processing.
                        //   For a gateway, letting it continue but without the roles seems safer.
                    }
                }
            }
        }

        if let Some(permission_str) = require_permission_json {
            if !permission_str.trim().is_empty() {
                // Aquí, asumiendo que `require_permission` es una *única* cadena JSON con un array
                // O si es solo una cadena, podrías convertirla a `vec![permission_str.clone()]`
                // La especificación es "String" y no "String[]", si es solo un permiso, se trata diferente:
                // Si es un único permiso, lo envolvemos en un vector.
                if let Ok(parsed_perms) = serde_json::from_str::<Vec<String>>(permission_str) {
                     if !parsed_perms.is_empty() {
                        permissions = Some(parsed_perms);
                     }
                } else {
                    // Si no es un array JSON, quizás sea una sola cadena de permiso
                    permissions = Some(vec![permission_str.clone()]);
                }
            }
        }

        ParseRouteAuth { permissions, roles }
    }

    /// Comprueba si la ruta requiere alguna autorización (roles o permisos).
    pub fn requires_auth(&self) -> bool {
        let mut require = true;
        require = require && self.permissions.as_ref().map_or(false, |p| !p.is_empty());
        tracing::info!("Is permissions empty: {}", require);
        require = require || self.roles.as_ref().map_or(false, |r| !r.is_empty());
        tracing::info!("Is roles empty: {}", require);
        tracing::info!("Self: {:?}", &self);
        require
    }
}


pub struct AuthChecker{}

impl AuthChecker {
    pub fn can_access(route: &ParseRouteAuth, user: &AuthResponse) -> bool{
        // Si el token del usuario no es válido, el acceso es denegado inmediatamente
        if !user.valid {
            tracing::warn!("Access denied: User token is not valid.");
            return false;
        }
         // Si la ruta no requiere ninguna autorización, el acceso es permitido por defecto
        if !route.requires_auth() {
            tracing::debug!("Access granted: Route does not require specific authorization.");
            return true;
        }
         // --- Verificación de Roles ---
        // Si la ruta especifica roles requeridos...
        if let Some(required_roles) = &route.roles {
            // Y el usuario tiene roles...
            if let Some(user_roles) = &user.roles {
                // Comprueba si el usuario tiene AL MENOS UNO de los roles requeridos
                let has_required_role = required_roles.iter().any(|req_role| {
                    user_roles.contains(req_role)
                });

                if !has_required_role {
                    tracing::warn!(
                        "Access denied: User does not have any of the required roles. Required: {:?}, User: {:?}",
                        required_roles, user_roles
                    );
                    return false; // No tiene ningún rol requerido
                }
            } else {
                // La ruta requiere roles, pero el usuario no tiene ninguno
                tracing::warn!(
                    "Access denied: Route requires roles ({:?}), but user has no roles.",
                    required_roles
                );
                return false;
            }
        }

        // --- Verificación de Permisos ---
        // Si la ruta especifica permisos requeridos...
        if let Some(required_permissions) = &route.permissions {
            // Y el usuario tiene permisos...
            if let Some(user_permissions) = &user.permissions {
                // Comprueba si el usuario tiene AL MENOS UNO de los permisos requeridos
                let has_required_permission = required_permissions.iter().any(|req_perm| {
                    user_permissions.contains(req_perm)
                });

                if !has_required_permission {
                    tracing::warn!(
                        "Access denied: User does not have any of the required permissions. Required: {:?}, User: {:?}",
                        required_permissions, user_permissions
                    );
                    return false; // No tiene ningún permiso requerido
                }
            } else {
                // La ruta requiere permisos, pero el usuario no tiene ninguno
                tracing::warn!(
                    "Access denied: Route requires permissions ({:?}), but user has no permissions.",
                    required_permissions
                );
                return false;
            }
        }

        // Si se llegó hasta aquí, significa que todas las verificaciones pasaron
        tracing::info!(
            "Access granted: User authorized for route. Roles met: {:?}, Permissions met: {:?}",
            route.roles, route.permissions
        );
        true

    }
}