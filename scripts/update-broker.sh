#!/usr/bin/env bash
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SERVICE_NAME="${PRIVATE_REMOTE_BROKER_SERVICE:-private-remote-broker}"
RESTART_MODE="${PRIVATE_REMOTE_BROKER_RESTART_MODE:-systemd}"

cd "$WORKSPACE_ROOT"

echo "Pulling latest git changes for broker..."
git pull --ff-only
echo "Building broker binary..."
cargo build --release -p device-broker

if [[ "$RESTART_MODE" == "docker-compose" ]]; then
  if command -v docker >/dev/null 2>&1; then
    echo "Restarting broker via docker compose..."
    docker compose -f ops/docker/docker-compose.yml up -d --build broker
  else
    echo "docker is not installed; broker update finished without restart."
  fi
elif command -v systemctl >/dev/null 2>&1 && systemctl list-unit-files | grep -q "^${SERVICE_NAME}\.service"; then
  echo "Restarting systemd service ${SERVICE_NAME}.service..."
  sudo systemctl restart "${SERVICE_NAME}.service"
else
  echo "systemd service ${SERVICE_NAME}.service not found. Broker update finished without restart."
fi
