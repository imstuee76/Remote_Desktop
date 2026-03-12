Write-Host "Preparing Private Remote starter repo..."

New-Item -ItemType Directory -Force -Path "ops/env" | Out-Null

if (-not (Test-Path "ops/env/.env")) {
    Copy-Item "ops/env/example.env" "ops/env/.env"
    Write-Host "Created ops/env/.env from example."
} else {
    Write-Host "ops/env/.env already exists."
}

Write-Host ""
Write-Host "Next steps:"
Write-Host "  1. Edit ops/env/.env"
Write-Host "  2. Point your VentraIP DNS records to your VPS"
Write-Host "  3. Open the repo in Codex / VS Code"
Write-Host "  4. Start with docs/IMPLEMENTATION_PLAN.md"
