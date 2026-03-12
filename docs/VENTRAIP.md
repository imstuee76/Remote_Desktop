# VentraIP Notes

## What VentraIP should do in this project

Use VentraIP for:
- domain registration / DNS management
- public DNS records for broker/relay hostnames
- optional infrastructure visibility if you want to surface VentraIP's public status feed inside your dashboard

## What VentraIP should NOT do here

Do not use VentraIP to infer whether **your devices** are online.
Device online/offline state must come from the broker's heartbeat/presence system.

## Recommended DNS records

Example:
- `broker.example.com` -> VPS public IP
- `relay.example.com` -> VPS public IP
- `status.example.com` -> optional dashboard host
- `turn.example.com` -> optional future TURN/STUN host

## Suggested deployment model

1. Buy/use a domain in VentraIP
2. Create DNS records in VIPcontrol or your preferred DNS hosting
3. Point `broker.*` and `relay.*` to your VPS
4. Terminate TLS with Caddy or Nginx
5. Run the broker and relay behind that reverse proxy

## Optional public status integration

A future dashboard can poll VentraIP's public service status endpoint and display it next to your own broker status.
That is useful for distinguishing:
- "my broker is down"
- "VentraIP / DNS / dependency issue"
- "host device is offline"

## Important distinction

Online devices = your broker's heartbeats  
Domain health / provider incidents = VentraIP / infrastructure signals
