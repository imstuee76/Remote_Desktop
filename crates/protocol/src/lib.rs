use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PlatformKind {
    Windows,
    LinuxX11,
    LinuxWayland,
    AndroidViewer,
    DesktopViewer,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceStatus {
    Online,
    Offline,
    Busy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SessionTransport {
    RelayBinary,
    RelayH264,
    RelayQuic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Capability {
    Screen,
    Input,
    Clipboard,
    FileTransfer,
    MultiMonitor,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseTargetId {
    WindowsHost,
    LinuxHost,
    AndroidViewer,
    BrokerServer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterHostRequest {
    pub device_id: String,
    pub device_name: String,
    pub platform: PlatformKind,
    pub capabilities: Vec<Capability>,
    pub owner_hint: Option<String>,
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicResponse {
    pub ok: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterHostResponse {
    pub ok: bool,
    pub message: String,
    pub device_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatRequest {
    pub device_id: String,
    pub active_session_count: u32,
    pub hostname: Option<String>,
    pub local_ips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListedDevice {
    pub device_id: String,
    pub device_name: String,
    pub platform: PlatformKind,
    pub status: DeviceStatus,
    pub last_seen_epoch_ms: u64,
    pub capabilities: Vec<Capability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceListResponse {
    pub devices: Vec<ListedDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRequest {
    pub target_device_id: String,
    pub viewer_name: String,
    pub preferred_transport: SessionTransport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResponse {
    pub ok: bool,
    pub session_id: String,
    pub transport: SessionTransport,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseVersionInfo {
    pub target: ReleaseTargetId,
    pub version: String,
    pub build: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseCatalogResponse {
    pub targets: Vec<ReleaseVersionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUpdateRequest {
    pub admin_token: String,
}

pub fn new_session_id() -> String {
    format!("sess_{}", Uuid::new_v4().simple())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_id_has_expected_prefix() {
        let value = new_session_id();
        assert!(value.starts_with("sess_"));
        assert!(value.len() > 10);
    }

    #[test]
    fn release_target_ids_use_kebab_case() {
        let value = serde_json::to_string(&ReleaseTargetId::BrokerServer).unwrap();
        assert_eq!(value, "\"broker-server\"");
    }
}
