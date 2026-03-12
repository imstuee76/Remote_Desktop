param(
    [switch]$RestartOnly
)

$ErrorActionPreference = "Stop"

$workspace = Resolve-Path (Join-Path $PSScriptRoot "..")
$serviceName = if ($env:PRIVATE_REMOTE_HOST_SERVICE) { $env:PRIVATE_REMOTE_HOST_SERVICE } else { "PrivateRemoteHostd" }

Push-Location $workspace
try {
    if (-not $RestartOnly) {
        Write-Host "Pulling latest git changes for host..."
        git pull --ff-only
        Write-Host "Building host binaries..."
        cargo build --release -p hostd -p desktop-control
    }

    $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
    if ($null -ne $service) {
        Write-Host "Restarting Windows service $serviceName..."
        Restart-Service -Name $serviceName -Force
    } else {
        Write-Host "Windows service $serviceName was not found. Build/update completed without service restart."
    }
} finally {
    Pop-Location
}
