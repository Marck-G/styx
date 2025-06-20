# Stage 1: Build the Rust application
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /app

# Install system dependencies required for building Rust applications
# For reqwest with native-tls, libssl-dev is often needed.
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libssl3 \
    openssl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo.toml and Cargo.lock first to leverage Docker's caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy src/main.rs to build dependencies, then remove it
# This helps with caching layers if source code changes frequently but dependencies don't
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && cargo build --release
RUN rm -rf target/release/deps/styx* target/release/styx # Clean up dummy build

# Copy the rest of the source code
COPY . .

# Build the Rust application in release mode
RUN cargo build --release

# --- Stage 2: Build the React WebUI ---
FROM node:20-alpine AS webui_builder 
# Usamos una imagen de Node.js ligera para el frontend

WORKDIR /app/webui 
# El directorio de trabajo para tu frontend

# Copia package.json y package-lock.json (o yarn.lock) para instalar dependencias primero
COPY webui/package.json webui/package-lock.json ./

# Instala las dependencias del frontend
RUN npm install

# Copia el resto del código fuente del frontend
COPY webui/ .

# Construye la aplicación React para producción
# Esto generará los archivos estáticos en la carpeta 'dist' por defecto
RUN npm run build


# Stage 3: Create the final lightweight image for deployment
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libssl3 \
    openssl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*


# Set the working directory
WORKDIR /app

# Create necessary directories for logs and database (if used by the app)
# These paths should match the defaults in your config.lua.template
RUN mkdir -p /app/var/log /app/lib

# Copy the compiled binary from the builder stage
# Replace 'your_app_name' with the actual name of your executable
# (usually the 'name' field in your Cargo.toml)
COPY --from=builder /app/target/release/styx /app/styx
# Copy the compiled WebUI static files from the webui_builder stage
# Los archivos se copiarán a /app/webui_dist dentro del contenedor final
COPY --from=webui_builder /app/webui/dist /app/http
# Copy the config template and the entrypoint script
COPY ./assets/entrypoint.sh /app/entrypoint.sh
COPY ./assets/config.lua.template /app/config.lua.template

# Make the entrypoint script executable
RUN chmod +x /app/entrypoint.sh

# Set default environment variables for your application.
# These values will be used if no specific environment variables are passed when running the container.
ENV LOG_FILE_PATH="./var/log/styx" \
    DATABASE_PATH="./lib/routes.db" \
    SOME_OPTION="valor opcional" \
    LOG_LEVEL="info" \
    LOG_TO_CONSOLE="true" \
    LOG_TO_FILE="true" \
    LOG_FORMAT_JSON="true" \
    CACHE_TTL="36000" \
    AUTH_URL="http://10.0.0.118:8082/v1/auth/check" \
    TOKEN_HEADER="api-token" \
    HTTP_PROXY_PORT="3080" \
    HTTP_PROXY_TIMEOUT="30000" \
    HTTP_PROXY_ADDRESS="0.0.0.0" \
    ADMIN_PORT="3082" \
    ADMIN_ADDRESS="0.0.0.0" \
    TARGET_TOKEN_HEADER="X-AuthToken"

# Expose the ports your application listens on
EXPOSE 3080
EXPOSE 3082

# Set the entrypoint script to run when the container starts
ENTRYPOINT ["/app/entrypoint.sh"]

# The CMD is executed by the ENTRYPOINT. If ENTRYPOINT is set, CMD becomes arguments to ENTRYPOINT.
# Since our ENTRYPOINT is an executable script, no CMD is strictly needed here unless you want to pass args.
# For example, if you wanted to pass args: CMD ["--some-arg", "value"]
# In our case, the script itself executes the app, so this line is fine to be omitted or left as is.
