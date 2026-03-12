# Android Viewer Stub

This module is intentionally a **starter Android viewer stub**, not a finished app.

## Scope

- browse devices from the broker
- request a session
- future remote frame rendering
- future input/touch translation

## Why this is a stub

The repo's real first priority is the broker + host control plane.
Once that API is stable, Codex should wire this Android app to the live broker.

## Suggested next steps for Codex

1. Create a small repository/data layer for `GET /api/v1/devices`
2. Add a ViewModel for loading device state
3. Replace placeholder sample data with live broker data
4. Add a session request flow
5. Add a frame renderer after the relay/session transport is defined
