Write-Host "Running starter checks..."

if (Get-Command cargo -ErrorAction SilentlyContinue) {
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace
} else {
    Write-Host "cargo not found; skipping Rust checks"
}

if (Test-Path "clients/android-viewer/gradlew.bat") {
    Push-Location "clients/android-viewer"
    .\gradlew.bat assembleDebug
    Pop-Location
} else {
    Write-Host "gradle wrapper not found; skipping Android build"
}
