use std::{
    collections::HashMap,
    net::SocketAddr,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use protocol::{
    new_session_id, AdminUpdateRequest, BasicResponse, DeviceListResponse, DeviceStatus,
    HeartbeatRequest, ListedDevice, RegisterHostRequest, RegisterHostResponse, ReleaseTargetId,
    ReleaseVersionInfo, SessionRequest, SessionResponse,
};
use release_meta::{release_catalog, version_for};
use tokio::sync::RwLock;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    devices: Arc<RwLock<HashMap<String, ListedDevice>>>,
    admin_token: Arc<String>,
    workspace_root: Arc<PathBuf>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            devices: Arc::new(RwLock::new(HashMap::new())),
            admin_token: Arc::new(std::env::var("BROKER_ADMIN_TOKEN").unwrap_or_default()),
            workspace_root: Arc::new(
                std::env::var("PRIVATE_REMOTE_WORKSPACE_ROOT")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))),
            ),
        }
    }
}

fn now_epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_millis() as u64
}

#[derive(serde::Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let bind = std::env::var("BROKER_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let addr: SocketAddr = bind.parse().expect("invalid BROKER_BIND socket address");
    let state = AppState::default();

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/admin", get(admin_page))
        .route("/api/v1/admin/version", get(admin_versions))
        .route("/api/v1/admin/update", post(admin_update))
        .route("/api/v1/devices/register", post(register_device))
        .route("/api/v1/devices/heartbeat", post(heartbeat))
        .route("/api/v1/devices", get(list_devices))
        .route("/api/v1/sessions/request", post(request_session))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind device broker listener");

    tracing::info!(%bind, "device broker listening");
    axum::serve(listener, app)
        .await
        .expect("device broker server failure");
}

async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn admin_page(State(state): State<AppState>) -> Html<String> {
    let broker_version = broker_release();
    let update_enabled = !state.admin_token.is_empty();
    let page = format!(
        r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Private Remote Broker Admin</title>
    <style>
      :root {{
        color-scheme: light;
        font-family: "Segoe UI", sans-serif;
        --bg: #f4f1ea;
        --panel: #fffdf9;
        --ink: #1a1f16;
        --accent: #1f5f4a;
        --border: #d5ccbb;
      }}
      body {{
        margin: 0;
        background: linear-gradient(135deg, #efe6d3 0%, #f8f6f1 55%, #e5efe9 100%);
        color: var(--ink);
      }}
      nav {{
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 16px 24px;
        background: rgba(255, 253, 249, 0.86);
        backdrop-filter: blur(8px);
        border-bottom: 1px solid var(--border);
      }}
      main {{
        max-width: 980px;
        margin: 0 auto;
        padding: 24px;
      }}
      .panel {{
        background: var(--panel);
        border: 1px solid var(--border);
        border-radius: 18px;
        padding: 20px;
        margin-bottom: 18px;
        box-shadow: 0 12px 32px rgba(26, 31, 22, 0.08);
      }}
      .row {{
        display: flex;
        gap: 12px;
        flex-wrap: wrap;
        align-items: center;
      }}
      input, button {{
        font: inherit;
        padding: 10px 14px;
        border-radius: 12px;
        border: 1px solid var(--border);
      }}
      button {{
        background: var(--accent);
        color: white;
        border: 0;
        cursor: pointer;
      }}
      button[disabled] {{
        opacity: 0.55;
        cursor: not-allowed;
      }}
      table {{
        width: 100%;
        border-collapse: collapse;
      }}
      th, td {{
        padding: 10px 8px;
        border-bottom: 1px solid var(--border);
        text-align: left;
      }}
      code {{
        background: #f0ebdf;
        padding: 2px 6px;
        border-radius: 8px;
      }}
      #status {{
        min-height: 1.5em;
      }}
    </style>
  </head>
  <body>
    <nav>
      <div>
        <strong>Private Remote Broker</strong>
        <div>Version {version} (build {build})</div>
      </div>
      <button id="update-button" {disabled}>Update from Git</button>
    </nav>
    <main>
      <section class="panel">
        <h2>Server Update</h2>
        <p>This action pulls the latest git changes, rebuilds the broker, and restarts the broker service path configured on this server.</p>
        <div class="row">
          <input id="token" type="password" placeholder="BROKER_ADMIN_TOKEN" style="min-width: 280px;">
          <span>Workspace root: <code>{workspace_root}</code></span>
        </div>
        <p id="status">{status_text}</p>
      </section>
      <section class="panel">
        <h2>Platform Versions</h2>
        <table>
          <thead>
            <tr><th>Target</th><th>Version</th><th>Build</th></tr>
          </thead>
          <tbody id="version-body"></tbody>
        </table>
      </section>
    </main>
    <script>
      async function loadVersions() {{
        const response = await fetch('/api/v1/admin/version');
        const payload = await response.json();
        const body = document.getElementById('version-body');
        body.innerHTML = '';
        payload.targets.forEach((entry) => {{
          const row = document.createElement('tr');
          row.innerHTML = `<td>${{entry.target}}</td><td>${{entry.version}}</td><td>${{entry.build}}</td>`;
          body.appendChild(row);
        }});
      }}

      document.getElementById('update-button').addEventListener('click', async () => {{
        const token = document.getElementById('token').value;
        const response = await fetch('/api/v1/admin/update', {{
          method: 'POST',
          headers: {{ 'Content-Type': 'application/json' }},
          body: JSON.stringify({{ admin_token: token }})
        }});
        const payload = await response.json();
        document.getElementById('status').innerText = payload.message;
      }});

      loadVersions();
    </script>
  </body>
</html>"#,
        version = broker_version.version,
        build = broker_version.build,
        workspace_root = state.workspace_root.display(),
        disabled = if update_enabled { "" } else { "disabled" },
        status_text = if update_enabled {
            "Ready."
        } else {
            "Set BROKER_ADMIN_TOKEN to enable server-side git updates."
        }
    );

    Html(page)
}

async fn admin_versions() -> Json<protocol::ReleaseCatalogResponse> {
    Json(release_catalog().clone())
}

async fn admin_update(
    State(state): State<AppState>,
    Json(request): Json<AdminUpdateRequest>,
) -> impl IntoResponse {
    if state.admin_token.is_empty() {
        return (
            StatusCode::FORBIDDEN,
            Json(BasicResponse {
                ok: false,
                message: "BROKER_ADMIN_TOKEN is not configured on the server".to_string(),
            }),
        );
    }

    if request.admin_token != *state.admin_token {
        return (
            StatusCode::UNAUTHORIZED,
            Json(BasicResponse {
                ok: false,
                message: "invalid admin token".to_string(),
            }),
        );
    }

    match launch_broker_update(&state.workspace_root) {
        Ok(()) => (
            StatusCode::ACCEPTED,
            Json(BasicResponse {
                ok: true,
                message: "broker update triggered; service restart is in progress".to_string(),
            }),
        ),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(BasicResponse {
                ok: false,
                message: format!("failed to trigger broker update: {error}"),
            }),
        ),
    }
}

fn broker_release() -> ReleaseVersionInfo {
    version_for(ReleaseTargetId::BrokerServer).expect("broker release should be present")
}

fn launch_broker_update(workspace_root: &Path) -> anyhow::Result<()> {
    if cfg!(target_os = "windows") {
        Command::new("powershell.exe")
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-File")
            .arg(workspace_root.join("scripts").join("update-broker.ps1"))
            .current_dir(workspace_root)
            .spawn()?;
    } else {
        let command = format!(
            "nohup bash '{}' >/tmp/private-remote-broker-update.log 2>&1 &",
            workspace_root.join("scripts").join("update-broker.sh").display()
        );
        Command::new("sh")
            .arg("-lc")
            .arg(command)
            .current_dir(workspace_root)
            .spawn()?;
    }

    Ok(())
}

async fn register_device(
    State(state): State<AppState>,
    Json(request): Json<RegisterHostRequest>,
) -> Json<RegisterHostResponse> {
    let listing = ListedDevice {
        device_id: request.device_id.clone(),
        device_name: request.device_name,
        platform: request.platform,
        status: DeviceStatus::Online,
        last_seen_epoch_ms: now_epoch_ms(),
        capabilities: request.capabilities,
    };

    state
        .devices
        .write()
        .await
        .insert(request.device_id.clone(), listing);

    Json(RegisterHostResponse {
        ok: true,
        message: "registered".to_string(),
        device_id: request.device_id,
    })
}

async fn heartbeat(
    State(state): State<AppState>,
    Json(request): Json<HeartbeatRequest>,
) -> Json<BasicResponse> {
    let mut devices = state.devices.write().await;
    let response = match devices.get_mut(&request.device_id) {
        Some(device) => {
            device.status = if request.active_session_count > 0 {
                DeviceStatus::Busy
            } else {
                DeviceStatus::Online
            };
            device.last_seen_epoch_ms = now_epoch_ms();
            BasicResponse {
                ok: true,
                message: "heartbeat accepted".to_string(),
            }
        }
        None => BasicResponse {
            ok: false,
            message: "unknown device".to_string(),
        },
    };

    Json(response)
}

async fn list_devices(State(state): State<AppState>) -> Json<DeviceListResponse> {
    let devices = state
        .devices
        .read()
        .await
        .values()
        .cloned()
        .collect::<Vec<_>>();

    Json(DeviceListResponse { devices })
}

async fn request_session(
    State(_state): State<AppState>,
    Json(request): Json<SessionRequest>,
) -> Json<SessionResponse> {
    Json(SessionResponse {
        ok: true,
        session_id: new_session_id(),
        transport: request.preferred_transport,
        message: format!("session requested for {}", request.target_device_id),
    })
}
