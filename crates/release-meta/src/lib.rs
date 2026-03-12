use std::sync::OnceLock;

use protocol::{ReleaseCatalogResponse, ReleaseTargetId, ReleaseVersionInfo};
use serde::Deserialize;

const RELEASE_MANIFEST_JSON: &str = include_str!("../../../ops/releases/manifest.json");

#[derive(Debug, Deserialize)]
struct ReleaseManifestFile {
    targets: Vec<ReleaseVersionInfo>,
}

pub fn release_catalog() -> &'static ReleaseCatalogResponse {
    static RELEASE_CATALOG: OnceLock<ReleaseCatalogResponse> = OnceLock::new();
    RELEASE_CATALOG.get_or_init(|| {
        let parsed: ReleaseManifestFile =
            serde_json::from_str(RELEASE_MANIFEST_JSON).expect("invalid release manifest");
        ReleaseCatalogResponse {
            targets: parsed.targets,
        }
    })
}

pub fn version_for(target: ReleaseTargetId) -> Option<ReleaseVersionInfo> {
    release_catalog()
        .targets
        .iter()
        .find(|entry| entry.target == target)
        .cloned()
}

pub fn host_target_for_current_platform() -> ReleaseTargetId {
    if cfg!(target_os = "windows") {
        ReleaseTargetId::WindowsHost
    } else {
        ReleaseTargetId::LinuxHost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_catalog_contains_expected_targets() {
        let catalog = release_catalog();
        assert!(
            catalog
                .targets
                .iter()
                .any(|entry| entry.target == ReleaseTargetId::WindowsHost)
        );
        assert!(
            catalog
                .targets
                .iter()
                .any(|entry| entry.target == ReleaseTargetId::LinuxHost)
        );
        assert!(
            catalog
                .targets
                .iter()
                .any(|entry| entry.target == ReleaseTargetId::AndroidViewer)
        );
        assert!(
            catalog
                .targets
                .iter()
                .any(|entry| entry.target == ReleaseTargetId::BrokerServer)
        );
    }
}
