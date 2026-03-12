# Private Remote — clean-room starter for Codex

This repository is a **Codex-ready starter scaffold** for a self-hosted remote desktop stack for personal use across many devices.

It is **not** a finished RustDesk replacement yet. Instead, it gives Codex:
- clear project instructions in `AGENTS.md`
- a clean-room architecture and feature plan
- a Rust workspace scaffold for the broker, host agent, and test viewer CLI
- a desktop control app for Windows/Linux host updates
- an Android viewer module stub
- self-hosting and VentraIP notes

## Goal

Build a private remote desktop system with these targets:

- **Hosts**: Windows, Ubuntu, Linux Mint Cinnamon
- **Viewer**: Android
- **Backend**: self-hosted broker + relay under your own domain
- **Business model**: no subscriptions, no seat limits, personal multi-device use

## Design choices locked in

- Clean-room implementation only. Do not copy code from RustDesk, AnyDesk, TeamViewer, or other remote desktop products.
- Android is **viewer/client only**, not a host.
- Start with a **relay-first** design so every device can connect out to a broker/relay over standard outbound internet paths.
- Device online/offline state comes from **our broker presence service**, not from VentraIP.
- VentraIP is used primarily for **DNS / domain ownership / optional status integration**.

## What is in this zip

- `AGENTS.md` — primary Codex instructions
- `.codex/config.toml` — project-scoped Codex config
- `docs/` — architecture, implementation order, security, VentraIP notes, API contract, feature matrix
- `Cargo.toml` — Rust workspace
- `crates/protocol` — shared protocol types
- `services/device-broker` — starter HTTP broker with in-memory device presence
- `apps/hostd` — starter host agent CLI scaffold
- `apps/viewer-cli` — starter viewer CLI for smoke testing
- `clients/desktop-control` — Windows/Linux control UI with git update menu
- `clients/android-viewer` — Android app stub for the future viewer
- `ops/` — example Docker/Caddy/env files
- `scripts/` — bootstrap and check scripts for Linux/macOS and PowerShell
- `ops/releases/manifest.json` — per-target version/build catalog

## Suggested first Codex prompts

1. `Read AGENTS.md and docs/IMPLEMENTATION_PLAN.md, then implement Milestone 1 completely.`
2. `Upgrade the device broker from in-memory state to SQLite, add migrations, and update docs/API.md.`
3. `Implement hostd device registration and recurring heartbeat with local config persistence.`
4. `Implement the Android viewer device list screen against docs/API.md.`
5. `Design the relay-only session transport for MVP and add protocol messages in crates/protocol.`

## Quick start

1. Unzip this folder.
2. Open it in VS Code / Codex.
3. Trust the project so Codex can read `.codex/config.toml`.
4. Read `AGENTS.md`.
5. Set up your domain names in `ops/env/example.env`.
6. Have Codex execute the milestones in `docs/IMPLEMENTATION_PLAN.md`.

## Recommended deployment names

- `broker.yourdomain.tld` — API, pairing, presence, signaling
- `relay.yourdomain.tld` — relay transport
- `turn.yourdomain.tld` — optional future TURN/STUN if direct media is added later
- `status.yourdomain.tld` — optional dashboard

## Update surfaces

- Windows/Linux host control app: `cargo run -p desktop-control`
- Broker admin page: `GET /admin`
- Android viewer: top-right menu shows the running Android version and the source-update instruction

Desktop and server update actions run the scripts under `scripts/` and expect the repo to be a real git checkout.

## Important reality check

This scaffold deliberately stops short of pretending the product is already finished.
The broker crate is only a **starter service** and the host / Android pieces are **skeletons** meant for Codex to complete in phases.
