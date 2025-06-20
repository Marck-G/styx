#!/bin/bash
# Check for debug mode
if [ "$DEBUG_MODE" = "true" ]; then
    echo "DEBUG_MODE is enabled. Keeping container alive for inspection."
    # Keep the container running indefinitely
    tail -f /dev/null
else
    set -e

    # Generate config.lua from template using environment variables
    # For string values, wrap with quotes. For boolean/numeric, do not.
    sed -e "s|\${LOG_FILE_PATH}|\"$LOG_FILE_PATH\"|g" \
        -e "s|\${DATABASE_PATH}|\"$DATABASE_PATH\"|g" \
        -e "s|\${SOME_OPTION}|\"$SOME_OPTION\"|g" \
        -e "s|\${LOG_LEVEL}|\"$LOG_LEVEL\"|g" \
        -e "s|\${LOG_TO_CONSOLE}|$LOG_TO_CONSOLE|g" \
        -e "s|\${LOG_TO_FILE}|$LOG_TO_FILE|g" \
        -e "s|\${LOG_FORMAT_JSON}|$LOG_FORMAT_JSON|g" \
        -e "s|\${CACHE_TTL}|$CACHE_TTL|g" \
        -e "s|\${AUTH_URL}|\"$AUTH_URL\"|g" \
        -e "s|\${TOKEN_HEADER}|\"$TOKEN_HEADER\"|g" \
        -e "s|\${HTTP_PROXY_PORT}|$HTTP_PROXY_PORT|g" \
        -e "s|\${HTTP_PROXY_TIMEOUT}|$HTTP_PROXY_TIMEOUT|g" \
        -e "s|\${HTTP_PROXY_ADDRESS}|\"$HTTP_PROXY_ADDRESS\"|g" \
        -e "s|\${ADMIN_PORT}|$ADMIN_PORT|g" \
        -e "s|\${ADMIN_ADDRESS}|\"$ADMIN_ADDRESS\"|g" \
        -e "s|\${TARGET_TOKEN_HEADER}|\"$TARGET_TOKEN_HEADER\"|g" \
        /app/config.lua.template > /app/config.lua


    echo "Starting application in normal mode."
    # Execute the main application
    exec /app/styx
fi
