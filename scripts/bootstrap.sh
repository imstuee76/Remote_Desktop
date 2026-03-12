#!/usr/bin/env bash
set -euo pipefail

echo "Preparing Private Remote starter repo..."
mkdir -p ops/env

if [[ ! -f ops/env/.env ]]; then
  cp ops/env/example.env ops/env/.env
  echo "Created ops/env/.env from example."
else
  echo "ops/env/.env already exists."
fi

echo
echo "Next steps:"
echo "  1. Edit ops/env/.env"
echo "  2. Point your VentraIP DNS records to your VPS"
echo "  3. Open the repo in Codex / VS Code"
echo "  4. Start with docs/IMPLEMENTATION_PLAN.md"
