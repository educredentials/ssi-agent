#!/bin/sh
set -e

echo "Starting database initialization..."

# Check if connection string is provided
if [ -z "$UNICORE__EVENT_STORE__CONNECTION_STRING" ]; then
    echo "Error: UNICORE__EVENT_STORE__CONNECTION_STRING environment variable is not set"
    exit 1
fi

echo "Connecting to database..."

# Run schema initialization
# The connection string should already point to the correct database/schema
echo "Creating tables if they don't exist..."
psql "$UNICORE__EVENT_STORE__CONNECTION_STRING" -f /init.sql

echo "Database initialization complete!"