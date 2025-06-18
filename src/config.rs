use mlua::{Lua, Result, Table};
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub struct HttpConf{
    pub timeout: u64,
    pub port: i16,
    pub address: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config{
    pub log_file_path: String,
    pub database_path: String,
    pub log_level: String,
    pub log_to_console: bool,
    pub log_to_file: bool,
    pub log_format_json: bool,
    pub cache_ttl: u64,
    pub auth_url: String,
    pub http_proxy: HttpConf,
    pub admin: HttpConf,
    pub token_header: String,
}

impl Config {
    pub fn from_lua_file(path: String) -> Result<Config>{
        let lua = Lua::new();

        // Ejecutar el script de configuración
        lua.load(&std::fs::read_to_string(path)?).exec()?;

        // Suponemos que el script devuelve una tabla global llamada "config"
        let globals: Table = lua.globals();
        let config_table: Table = globals.get("config")?;

        // Extraer valores
        let log_file_path: String = config_table.get("log_file_path")?;
        let database_path: String = config_table.get("database_path")?;
        let log_level: String = config_table.get("log_level")?;
        let log_to_console: bool = config_table.get("log_to_console")?;
        let log_to_file: bool = config_table.get("log_to_file")?;
        let log_format_json: bool = config_table.get("log_format_json")?;
        let cache_ttl: u64 = config_table.get("cache_TTL")?;
        let auth_url: String = config_table.get("auth_url")?;
        let token_header: String = config_table.get("token_header")?;

        let proxy_table: Table = globals.get("http_proxy")?;
        let p_port: i16 = proxy_table.get("port")?;
        let p_timeout: u64 = proxy_table.get("timeout")?;
        let p_address: String = proxy_table.get("address")?;
        let proxy: HttpConf = HttpConf { timeout: p_timeout, port: p_port, address: p_address };

        let admin_table: Table = globals.get("admin")?;
        let a_port: i16 = admin_table.get("port")?;
        let a_address: String = admin_table.get("address")?;
        let admin: HttpConf = HttpConf { timeout: 0u64, port: a_port, address: a_address };

        Ok(Config {
            log_file_path,
            database_path,
            log_level,
            log_to_console,
            log_to_file,
            log_format_json,
            cache_ttl,
            http_proxy: proxy,
            admin,
            auth_url,
            token_header
        })
    }
}