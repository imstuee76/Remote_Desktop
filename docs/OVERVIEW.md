# Overview

This project is a clean-room remote desktop stack aimed at one owner managing many personal devices.

## Product shape

- A self-hosted broker tracks devices, pairings, and online presence.
- Hosts maintain outbound connections or recurring heartbeats to the broker.
- Viewers browse the broker's device list and request sessions.
- The session path starts relay-first for reliability behind NAT and home internet.
- Direct peer or TURN-assisted paths are optional future upgrades, not day-one requirements.

## User story

> I want to self-host a remote desktop system that behaves like the better parts of RustDesk / AnyDesk / TeamViewer, but only for my own devices and without subscriptions.

## MVP summary

- Pair a host to an owner account or owner secret
- See host online/offline state in a device list
- Connect from Android viewer to a Windows or Linux host
- View the remote screen
- Send keyboard/mouse/touch input
- Basic clipboard sync
- Basic file send/receive

## Architectural opinion

The fastest trustworthy route is:
1. broker + presence first
2. relay-only sessions second
3. platform capture/input implementations third
4. optimization and direct paths last

That avoids the usual trap of trying to solve NAT traversal, codec tuning, GPU acceleration, and every OS quirk before there is even a reliable control plane.
