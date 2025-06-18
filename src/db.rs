use glob::Pattern;
use serde::Serialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{query, query_as, Result, SqlitePool};
#[derive(Debug, Clone, Serialize)]
pub struct Route {
    pub id: i64,
    pub path_pattern: String,
    pub http_method: String,
    pub target_url: String,
    pub required_roles: Option<String>, // JSON string o CSV con roles
    pub require_permission: Option<String>,
}
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(path: String) -> Result<Self> {
        let pool = SqlitePoolOptions::new().connect(&path).await?;
        tracing::info!("Database loaded: {}", path);

        // creamos la tabla
        query(
            r#"
            CREATE TABLE IF NOT EXISTS gateway_routes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path_pattern TEXT NOT NULL,
                http_method TEXT NOT NULL,
                target_url TEXT NOT NULL,
                required_roles TEXT,
                require_permission TEXT
            );
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
    pub async fn insert_route(
        &self,
        path_pattern: &str,
        http_method: &str,
        target_url: &str,
        required_roles: Option<&str>,
        require_permission: Option<&str>,
    ) -> Result<()> {
        query(
            r#"
            INSERT INTO gateway_routes 
            (path_pattern, http_method, target_url, required_roles, require_permission) 
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(path_pattern)
        .bind(http_method)
        .bind(target_url)
        .bind(required_roles)
        .bind(require_permission)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // Función para buscar rutas que coincidan con path y método
    pub async fn find_matching_route(&self, path: &str, method: &str) -> Result<Option<Route>> {
        let routes = query_as!(
            Route,
            r#"
            SELECT id, path_pattern, http_method, target_url, required_roles, require_permission
            FROM gateway_routes
            WHERE (http_method = ? OR http_method = 'ALL')
            "#,
            method
        )
        .fetch_all(&self.pool)
        .await?;

        // Filtrar solo rutas que hagan match
        let mut matches: Vec<Route> = routes
            .into_iter()
            .filter(|route| path_matches(&route.path_pattern, path))
            .collect();

        // Elegir la ruta con path_pattern más largo (más restrictiva)
        matches.sort_by(|a, b| b.path_pattern.len().cmp(&a.path_pattern.len()));

        Ok(matches.into_iter().next())
    }

    pub async fn get_all_routes(&self) -> Result<Vec<Route>> {
        let routes = query_as!(
            Route,
            r#"
        SELECT id, path_pattern, http_method, target_url, required_roles, require_permission
        FROM gateway_routes
        "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(routes)
    }
    pub async fn update_route(
        &self,
        route_id: i64,
        path_pattern: &str,
        http_method: &str,
        target_url: &str,
        required_roles: Option<&str>,
        require_permission: Option<&str>,
    ) -> Result<bool> {
        let result = query!(
            r#"
        UPDATE gateway_routes
        SET path_pattern = ?, http_method = ?, target_url = ?, 
            required_roles = ?, require_permission = ?
        WHERE id = ?
        "#,
            path_pattern,
            http_method,
            target_url,
            required_roles,
            require_permission,
            route_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_route(&self, route_id: i64) -> Result<bool> {
        let result = query!(
            r#"
        DELETE FROM gateway_routes
        WHERE id = ?
        "#,
            route_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

// Función de ejemplo para match de rutas (comodines)
// Aquí puedes usar crates como glob o regex para hacer matching avanzado
fn path_matches(pattern: &str, path: &str) -> bool {
    match Pattern::new(pattern) {
        Ok(p) => p.matches(path),
        Err(e) => {
            // Log an error if the pattern itself is invalid (e.g., malformed glob)
            tracing::error!("Invalid glob pattern '{}': {:?}", pattern, e);
            false
        }
    }
}
