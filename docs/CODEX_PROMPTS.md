# Handy Codex Prompts

## Backend
`Read AGENTS.md and implement Milestone 1 end-to-end. Keep the broker API aligned with docs/API.md and add tests.`

`Replace the broker's in-memory registry with SQLite, add migrations, and update docs/ARCHITECTURE.md with any changed assumptions.`

## Host
`Implement hostd local config persistence, stable device identity, and a daemon-style heartbeat loop.`

`Create the Windows host abstraction interfaces for capture, input, clipboard, and monitor enumeration without implementing every OS detail yet.`

## Android
`Wire the Android viewer stub to GET /api/v1/devices and render the device list using a repository + ViewModel pattern.`

`Add session request UI in the Android viewer and model the response from docs/API.md.`

## Ops
`Create a production-minded broker Docker image and refresh ops/docker/broker.Dockerfile and ops/caddy/Caddyfile.`

`Add a simple infrastructure status page that reports broker health and optional VentraIP status health.`
