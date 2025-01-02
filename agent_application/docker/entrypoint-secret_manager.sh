#!/bin/sh

# This script runs the secret manager with the -f flag to produce and/or
# overwrite the stronghold file
# It captures the output of the secret manager and writes it to a file

/usr/local/bin/agent_secret_manager -f > "${UNICORE__SECRET_MANAGER__STRONGHOLD_LOG_PATH}"

cat "${UNICORE__SECRET_MANAGER__STRONGHOLD_LOG_PATH}"
