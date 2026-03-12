# AGENTS.md

## Mission

Build a **clean-room**, self-hosted remote desktop platform for personal multi-device use.

Baseline inspiration:
- unattended access
- device list / online presence
- relay-based connectivity
- clipboard and file transfer
- Android viewer
- Windows and Linux hosts

**Do not copy code** from RustDesk, AnyDesk, TeamViewer, or any AGPL/proprietary remote desktop implementation.
You may study public API docs, OS documentation, and clean-room behavioral descriptions, but every line in this repository must be original.

## Locked product constraints

- Android is **viewer only**
- Hosts must support:
  - Windows
  - Ubuntu
  - Linux Mint Cinnamon
- Start with **relay-first** connectivity
- No subscription logic
- No seat or device-count licensing
- Personal/self-hosted usage first
- Device presence comes from our backend, not from a registrar or DNS provider

## Engineering priorities

1. Safe, boring, understandable architecture
2. Self-hosted deployment simplicity
3. Reliable online/offline device visibility
4. Unattended access after user login
5. Input + screen transport
6. Clipboard + file transfer
7. Multi-monitor
8. Performance upgrades (hardware encode, direct paths, adaptive bitrate)

## Non-goals for the first milestones

- Full parity with commercial remote desktop products
- Lock-screen / pre-login control
- Mesh networking
- Fancy account systems
- Cloud SaaS features
- iOS viewer
- macOS host
- Browser viewer

## Workflow rules

- Read `docs/IMPLEMENTATION_PLAN.md` before major coding.
- Keep the repo clean-room.
- Prefer small vertical slices that can be tested end to end.
- When adding a new endpoint or message type, update:
  - `docs/API.md`
  - `docs/FEATURE_MATRIX.md`
  - relevant README comments
- Do not silently widen scope.
- When platform support differs, document it instead of hand-waving it.

## Milestone order

### Milestone 1
Broker presence API + host registration + heartbeat + viewer device listing.

### Milestone 2
Pairing/auth basics, persistent storage, host config, Android viewer list UI.

### Milestone 3
Relay-only session setup with control channel and dummy frame transport.

### Milestone 4
Windows capture/input path.

### Milestone 5
Linux X11 capture/input path.

### Milestone 6
Linux Wayland path via portals.

### Milestone 7
Clipboard, file transfer, multi-monitor selection.

### Milestone 8
Packaging, installers, system services, auto-start, update strategy.

## Security rules

- Never expose a device-control feature publicly without authentication.
- Prefer per-device key pairs and short-lived session approvals.
- Avoid plaintext secrets in committed files.
- Use env examples, not real credentials.
- Mark placeholders honestly as placeholders.

## Testing rules

- Add or update tests when changing protocol/state logic.
- Keep platform-specific code behind clear interfaces.
- Do not merge platform hacks into shared crates.
- Prefer deterministic unit tests for broker state transitions.

## Build/check expectations

Linux/macOS:
- `bash scripts/check.sh`

Windows PowerShell:
- `pwsh -File scripts/check.ps1`

## Repo conventions

- Rust shared logic goes in `crates/`
- Services go in `services/`
- End-user apps go in `apps/` or `clients/`
- Ops/deployment files go in `ops/`
- Product/architecture decisions go in `docs/`

## VentraIP rule

Treat VentraIP as:
- a domain/DNS home
- an optional infrastructure status input

Do **not** model device presence by scraping registrar/DNS state.
Device online/offline visibility must come from the broker heartbeat system.

## Clean-room reminder

The phrase "like RustDesk" in user goals means:
- similar user value
- similar feature families
- similar self-hosted ownership model

It does **not** authorize code reuse.
