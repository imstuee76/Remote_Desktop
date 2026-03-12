#!/usr/bin/env bash
set -euo pipefail

RESTART_ONLY="${1:-}"
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SERVICE_NAME="${PRIVATE_REMOTE_HOST_SERVICE:-private-remote-hostd}"

cd "$WORKSPACE_ROOT"

if [[ "$RESTART_ONLY" != "--restart-only" ]]; then
  echo "Pulling latest git changes for host..."
  git pull --ff-only
  echo "Building host binaries..."
  cargo build --release -p hostd -p desktop-control
fi

if command -v systemctl >/dev/null 2>&1 && systemctl list-unit-files | grep -q "^${SERVICE_NAME}\.service"; then
  echo "Restarting systemd service ${SERVICE_NAME}.service..."
  sudo systemctl restart "${SERVICE_NAME}.service"
else
  echo "systemd service ${SERVICE_NAME}.service not found. Build/update completed without service restart."
fi
