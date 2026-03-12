# Implementation Plan

This file is written to be directly actionable by Codex.

## Milestone 0 — repo hygiene
Deliverables:
- verify workspace structure
- keep docs synchronized
- add tests for protocol and broker state
- keep placeholder code compiling as features are added

Acceptance:
- project structure is consistent
- no fake "done" claims

## Milestone 1 — presence MVP
Deliverables:
- broker endpoints from `docs/API.md`
- in-memory device registry
- host CLI can register
- host CLI can send recurring heartbeats
- viewer CLI can list devices

Acceptance:
- one host appears online after registration/heartbeat
- stale hosts can be marked offline by timeout logic

## Milestone 2 — persistence + pairing basics
Deliverables:
- SQLite-backed broker state
- migrations
- owner secret or simple auth token
- persistent host config file
- stable device identity

Acceptance:
- restart broker without losing paired device metadata
- untrusted requests are rejected

## Milestone 3 — session control plane
Deliverables:
- session request API
- session accept/reject flow
- short-lived session ticket
- relay service stub
- viewer and host session negotiation

Acceptance:
- a requested session gets a real session ID
- host/viewer can both attach to that session in relay

## Milestone 4 — frame transport prototype
Deliverables:
- simple frame channel
- basic remote image rendering path
- control channel for mouse/keyboard events
- instrumentation/logging

Acceptance:
- Android viewer can see changing frames from a test source or real host source
- input events reach the host abstraction layer

## Milestone 5 — Windows host implementation
Deliverables:
- real screen capture
- real input injection
- monitor enumeration
- packaging notes

Acceptance:
- connect from Android to Windows host and control the desktop after login

## Milestone 6 — Linux X11 host implementation
Deliverables:
- capture + input on X11
- clipboard bridge
- deb packaging notes

Acceptance:
- connect from Android to Ubuntu/Linux Mint X11 session and control it

## Milestone 7 — Linux Wayland host implementation
Deliverables:
- portal-driven capture/control
- session restore handling where possible
- desktop-environment notes

Acceptance:
- connect from Android to a Wayland session with explicit portal approvals or documented unattended flow

## Milestone 8 — user features
Deliverables:
- clipboard sync
- file transfer
- monitor picker
- quality presets

Acceptance:
- useful everyday remote administration on personal devices

## Milestone 9 — packaging and ops
Deliverables:
- installers / service units
- Docker image for broker
- reverse proxy docs
- backup/restore docs
- update strategy

Acceptance:
- a fresh VPS + domain can host the stack cleanly
