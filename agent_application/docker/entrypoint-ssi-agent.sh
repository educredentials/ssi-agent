#!/bin/sh
echo "${UNICORE__SECRET_MANAGER__STRONGHOLD_LOG_PATH}"
ls -l "$(dirname ${UNICORE__SECRET_MANAGER__STRONGHOLD_LOG_PATH})"
ls -l "${UNICORE__SECRET_MANAGER__STRONGHOLD_LOG_PATH}"

output=$(cat ${UNICORE__SECRET_MANAGER__STRONGHOLD_LOG_PATH})

# Extract the ed25519_key value
ed25519_key=$(echo "$output" | grep 'ed25519_key' | awk '{print $3}')

export UNICORE__SECRET_MANAGER__EDDSA_KEY_ID=$ed25519_key

# Run the SSI agent
/usr/local/bin/agent_application
