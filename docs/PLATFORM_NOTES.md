# Platform Notes

These are implementation notes for Codex and future developers.

## Windows host

Recommended direction:
- screen capture via DXGI Desktop Duplication / newer Windows capture abstractions
- input injection through native Win32 APIs
- monitor enumeration via native display APIs
- clipboard bridge via Win32 clipboard APIs

Watch-outs:
- elevated / high-integrity windows can impose restrictions on injected input
- multi-monitor capture needs per-output handling
- session/desktop changes should not crash the host agent
- host update menu can run `scripts/update-host.ps1` and restart the configured Windows service

## Linux X11 host

Recommended direction:
- capture through X11-friendly APIs/libraries
- input through XTest or equivalent
- explicit monitor/window coordinate mapping
- clipboard via X11 selections

Watch-outs:
- compositors and color formats differ
- distro package names vary
- screen locking / login manager control differs by desktop/session
- host update menu can run `scripts/update-host.sh` and restart the configured systemd service

## Linux Wayland host

Recommended direction:
- use xdg-desktop-portal ScreenCast and RemoteDesktop interfaces
- treat permissions and session restore carefully
- expect desktop-environment differences

Watch-outs:
- behavior can differ by compositor/backend
- unattended workflows may need desktop-specific capability handling
- avoid pretending Wayland behaves like X11

## Android viewer

Scope:
- device list
- session connect
- remote frame rendering
- touch -> remote pointer / gesture translation
- soft keyboard -> remote key events
- clipboard send / receive later
- file pickers later

Not in scope:
- Android host mode
- background screen capture
- Android device remote control as a host target
- on-device `git pull` self-update for installed APKs

Update notes:
- show `BuildConfig.VERSION_NAME` / `VERSION_CODE` in the app menu
- explain source update flow honestly: pull repo, build APK, reinstall app

## Performance strategy

Phase order:
1. correctness
2. frame pacing
3. monitor selection
4. hardware encoding / decoding
5. bitrate adaptation
6. direct transport optimization
