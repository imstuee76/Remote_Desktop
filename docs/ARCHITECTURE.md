# Architecture

## Principles

1. Clean-room
2. Self-hosted first
3. Relay-first for reliability
4. Small trust boundary
5. Platform-specific code isolated behind traits/interfaces
6. Honest fallback behavior when a platform limits control

## High-level components

### 1) Device Broker
Responsibilities:
- owner auth / pairing
- device registration
- online presence and heartbeats
- session request routing
- device metadata and tags
- admin version catalog and broker update trigger
- future policy enforcement and audit logs

### 2) Relay
Responsibilities:
- binary transport for session traffic
- frame and input message relay
- future adaptive transport negotiation
- future direct-path coordination

### 3) Host Agent (`hostd`)
Responsibilities:
- install/run on Windows and Linux
- register with broker
- maintain heartbeat
- capture screen
- inject input
- transfer clipboard/files
- persist local device identity and config

Companion control surface:
- `clients/desktop-control` provides a desktop menu for viewing per-target versions, pulling latest git changes, and restarting the host service

### 4) Viewer
Responsibilities:
- show device list
- request session
- render frames
- send input
- choose monitor
- initiate clipboard/file flows

## Recommended phase architecture

### Phase A — control plane only
- HTTP/JSON broker API
- host registration
- heartbeats
- device list
- manual session requests

### Phase B — relay-only session MVP
- broker issues short-lived session ticket
- host connects to relay
- viewer connects to relay
- relay joins both sides by session ID
- start with simplistic binary frame transport so the control plane gets proven early

### Phase C — real transport
- frame diffing / tiling
- hardware or software H.264 encode path
- adaptive bitrate and frame pacing
- clipboard and file channels
- multi-monitor and quality presets

## Connectivity model

### MVP
Every participant makes **outbound** connections to infrastructure you control:
- host -> broker
- viewer -> broker
- host -> relay
- viewer -> relay

This avoids requiring port-forwarding on most home networks.

### Later
Optional:
- STUN/TURN
- ICE-style direct path
- QUIC direct data path
- fallback back to relay if direct connect fails

## Identity model

Recommended:
- one owner identity
- per-device key pair
- per-device stable ID
- pairing token or QR-like bootstrap flow
- short-lived session grants

## Storage model

### Early
- SQLite is fine for personal/self-hosted use
- in-memory session registry
- file-backed local host config

### Later
- Postgres if multi-user or high-availability is needed
- Redis or broker-local memory for volatile relay/session state

## Screen transport strategy

### MVP proof path
- still-image or tile-based encoded frames over relay
- enough to validate broker/relay/session lifecycle

### Usable path
- H.264 baseline/high profile depending hardware
- dynamic scaling and bitrate
- damage-region aware capture when possible
- monitor-specific streams

## Platform boundaries

### Windows host
- capture module
- input injection module
- clipboard bridge
- file transfer module

### Linux host
Split into:
- X11 implementation
- Wayland implementation
- shared Linux host control and packaging

## Trust boundaries

- Broker is trusted to know device metadata and online presence.
- Relay is trusted to relay encrypted traffic, but the long-term goal should be end-to-end session protection above transport where practical.
- Host and viewer are trusted endpoints owned by the same operator.

## Packaging goals

### Windows
- service + tray helper later
- installer later
- portable debug build for early milestones
- control app can call `scripts/update-host.ps1`

### Linux
- deb package first
- AppImage later
- systemd user/service units
- desktop entry for configuration UI
- control app can call `scripts/update-host.sh`

### Android
- APK sideload first
- Play Store packaging only after permissions, privacy wording, and stability are solid
- version display is in-app; source updates remain off-device build/install
