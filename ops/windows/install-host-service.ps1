param(
    [string]$WorkspaceRoot = "D:\Code_Projects\Remote_Desktop",
    [string]$BrokerUrl = "http://127.0.0.1:8080",
    [string]$DeviceName = "Windows Host",
    [string]$Platform = "windows",
    [string]$ServiceName = "PrivateRemoteHostd"
)

$binary = Join-Path $WorkspaceRoot "target\release\hostd.exe"
$arguments = "--broker `"$BrokerUrl`" --device-name `"$DeviceName`" --platform $Platform demo-loop"

New-Service `
    -Name $ServiceName `
    -BinaryPathName "`"$binary`" $arguments" `
    -DisplayName "Private Remote Host Agent" `
    -StartupType Automatic
