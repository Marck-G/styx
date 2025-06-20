config = {
    log_file_path = "./var/log/styx",
    database_path = "./lib/routes.db",
    some_option = "valor opcional",
    log_level = "info",
    log_to_console = true,
    log_to_file = true,
    log_format_json = true,
    cache_TTL = 36000,
    auth_url = "http://localhost:8082/v1/auth/check",
    token_header= "api-token"
}
http_proxy = {
    port = 3080,
    timeout = 30000,
    address = "0.0.0.0"
}

admin = {
    port = 3082,
    address = "0.0.0.0"
}

target = {
    token_header = "X-AuthToken"
}