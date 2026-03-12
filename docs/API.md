# Broker API (starter contract)

This is the starter HTTP/JSON contract for the broker.
It is intentionally simple and exists so Codex has a stable first target.

## `GET /healthz`

Response:
```json
{ "status": "ok" }
```

## `POST /api/v1/devices/register`

Request:
```json
{
  "device_id": "desktop-123",
  "device_name": "Office Desktop",
  "platform": "windows",
  "capabilities": ["screen", "input", "clipboard"],
  "owner_hint": "personal",
  "public_key": "base64-or-placeholder"
}
```

Response:
```json
{
  "ok": true,
  "message": "registered",
  "device_id": "desktop-123"
}
```

## `POST /api/v1/devices/heartbeat`

Request:
```json
{
  "device_id": "desktop-123",
  "active_session_count": 0,
  "hostname": "office-box",
  "local_ips": ["192.168.1.50"]
}
```

Response:
```json
{
  "ok": true,
  "message": "heartbeat accepted"
}
```

## `GET /api/v1/devices`

Response:
```json
{
  "devices": [
    {
      "device_id": "desktop-123",
      "device_name": "Office Desktop",
      "platform": "windows",
      "status": "online",
      "last_seen_epoch_ms": 1736400000000,
      "capabilities": ["screen", "input", "clipboard"]
    }
  ]
}
```

## `POST /api/v1/sessions/request`

Request:
```json
{
  "target_device_id": "desktop-123",
  "viewer_name": "Pixel Phone",
  "preferred_transport": "relay-binary"
}
```

Response:
```json
{
  "ok": true,
  "session_id": "sess_abc123",
  "transport": "relay-binary",
  "message": "session requested"
}
```

## `GET /api/v1/admin/version`

Response:
```json
{
  "targets": [
    {
      "target": "windows-host",
      "version": "0.1.0-windows",
      "build": 1100
    },
    {
      "target": "linux-host",
      "version": "0.1.0-linux",
      "build": 2100
    },
    {
      "target": "android-viewer",
      "version": "0.1.0-android",
      "build": 3100
    },
    {
      "target": "broker-server",
      "version": "0.1.0-server",
      "build": 4100
    }
  ]
}
```

## `POST /api/v1/admin/update`

Request:
```json
{
  "admin_token": "replace-me"
}
```

Response:
```json
{
  "ok": true,
  "message": "broker update triggered; service restart is in progress"
}
```

Notes:
- `BROKER_ADMIN_TOKEN` must be configured on the server.
- This endpoint is intended for self-hosted admin use behind your own auth boundary.
- The HTML admin page at `/admin` calls these endpoints.

## Future additions

- auth / owner tokens
- pairing approvals
- session cancellation
- session acceptance / rejection
- multi-monitor enumeration
- clipboard endpoints or multiplexed relay channels
- file transfer negotiation
- richer host capabilities and desktop environment details
