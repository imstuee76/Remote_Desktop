# Security

## Security posture

This project is intended for self-hosted personal use, but it still needs sane security boundaries.

## Minimum acceptable posture before internet exposure

- TLS termination on public endpoints
- authenticated owner access
- per-device identity
- short-lived session grants
- replay-resistant pairing flow
- audit logs for session creation
- explicit unattended-access policy on each host

## Recommended model

### Pairing
- install host
- generate device key pair locally
- show pairing code or read bootstrap token
- owner approves the device
- broker stores device metadata + public key

### Session start
- viewer authenticates as owner
- viewer requests target device
- broker issues short-lived signed session ticket
- host validates ticket
- relay associates traffic using the ticket/session ID

## Host safety rules

- host agent should never default to world-open control
- unattended mode should be explicit
- file transfer paths must be sandboxed/validated
- clipboard sync must be opt-out capable
- sensitive key material belongs in OS-appropriate secure storage when possible

## Logging

Log:
- device registrations
- last-seen heartbeats
- session requests
- session approvals / denials
- relay open / close events
- file transfer metadata

Do not log:
- clipboard contents
- file contents
- raw screen frames
- plaintext credentials

## Clean-room legal hygiene

- no copied protocol formats
- no copied UI strings
- no copied source code
- no imported AGPL components unless the project owner explicitly decides to accept that license later
