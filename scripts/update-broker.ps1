$ErrorActionPreference = "Stop"

$workspace = Resolve-Path (Join-Path $PSScriptRoot "..")
$serviceName = if ($env:PRIVATE_REMOTE_BROKER_SERVICE) { $env:PRIVATE_REMOTE_BROKER_SERVICE } else { "PrivateRemoteBroker" }

Push-Location $workspace
try {
    Write-Host "Pulling latest git changes for broker..."
    git pull --ff-only
    Write-Host "Building broker binary..."
    cargo build --release -p device-broker

    $service = Get-Service -Name $serviceName -ErrorAction SilentlyContinue
    if ($null -ne $service) {
        Write-Host "Restarting Windows service $serviceName..."
        Restart-Service -Name $serviceName -Force
    } else {
        Write-Host "Windows service $serviceName was not found. Broker build/update completed without service restart."
    }
} finally {
    Pop-Location
}
