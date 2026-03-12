# Feature Matrix

Legend:
- **MVP** = must exist in first usable release
- **Next** = after MVP
- **Later** = valuable, but not blocking

| Feature | Windows Host | Linux X11 Host | Linux Wayland Host | Android Viewer | Priority |
|---|---|---|---|---|---|
| Device registration | Yes | Yes | Yes | N/A | MVP |
| Online/offline presence | Yes | Yes | Yes | View only | MVP |
| In-app version display | Yes | Yes | Yes | Yes | MVP |
| Menu-triggered git update path | Yes | Yes | Admin page only | Limited to source-build notice | MVP |
| Device aliases / tags | Yes | Yes | Yes | View/edit later | MVP |
| Relay-only connection path | Yes | Yes | Yes | Yes | MVP |
| Screen viewing | Yes | Yes | Yes | Yes | MVP |
| Input control | Yes | Yes | Portal-based | Yes | MVP |
| Unattended access after login | Yes | Yes | Desktop-policy dependent | Yes | MVP |
| Multi-monitor selection | Yes | Yes | Partial | Yes | Next |
| Clipboard sync | Yes | Yes | Partial | Yes | Next |
| File transfer | Yes | Yes | Yes | Yes | Next |
| Host chat / notes | Optional | Optional | Optional | Optional | Later |
| Remote audio | Yes | X11 easier than Wayland | Desktop-dependent | Yes | Later |
| Direct peer mode | Optional | Optional | Optional | Optional | Later |
| Lock-screen access | Harder | DE/session dependent | Usually harder | N/A | Later |
| Remote reboot / reconnect | Yes | Yes | Yes | Yes | Later |
| LAN discovery | Yes | Yes | Yes | Yes | Later |
| Wake-on-LAN helpers | Possible | Possible | Possible | Trigger only | Later |

## Notes

- Wayland should be treated as a first-class target, but its control/capture path is fundamentally different from X11.
- "Android Viewer" means browse devices, connect, view, and send input. It does **not** mean Android hosting.
- Android exposes its running version in-app, but installed APKs cannot safely self-`git pull`; source updates remain a build-and-reinstall flow.
- Device count is intentionally unlicensed in this project.
