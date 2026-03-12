mod platform;

use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use protocol::{
    Capability, HeartbeatRequest, PlatformKind, RegisterHostRequest, SessionTransport,
};
use reqwest::Client;
use release_meta::{host_target_for_current_platform, version_for};
use std::time::Duration;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(name = "hostd")]
#[command(about = "Starter host agent CLI for the clean-room remote desktop project")]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:8080")]
    broker: String,

    #[arg(long, default_value = "Personal Device")]
    device_name: String,

    #[arg(long, default_value_t = HostPlatformArg::Windows)]
    platform: HostPlatformArg,

    #[arg(long)]
    device_id: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Register,
    Heartbeat,
    DemoLoop {
        #[arg(long, default_value_t = 15)]
        interval_seconds: u64,
    },
    SessionProbe {
        #[arg(long, default_value = "Android Viewer")]
        viewer_name: String,
    },
    Version,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum HostPlatformArg {
    Windows,
    LinuxX11,
    LinuxWayland,
}

impl From<HostPlatformArg> for PlatformKind {
    fn from(value: HostPlatformArg) -> Self {
        match value {
            HostPlatformArg::Windows => PlatformKind::Windows,
            HostPlatformArg::LinuxX11 => PlatformKind::LinuxX11,
            HostPlatformArg::LinuxWayland => PlatformKind::LinuxWayland,
        }
    }
}

fn stable_or_generated_device_id(input: Option<String>) -> String {
    input.unwrap_or_else(|| format!("dev_{}", Uuid::new_v4().simple()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = Client::new();
    let device_id = stable_or_generated_device_id(cli.device_id);
    let platform: PlatformKind = cli.platform.into();

    match cli.command {
        Command::Register => {
            register(&client, &cli.broker, &device_id, &cli.device_name, platform).await?;
        }
        Command::Heartbeat => {
            heartbeat(&client, &cli.broker, &device_id, 0).await?;
        }
        Command::DemoLoop { interval_seconds } => {
            register(&client, &cli.broker, &device_id, &cli.device_name, platform).await?;
            loop {
                heartbeat(&client, &cli.broker, &device_id, 0).await?;
                tokio::time::sleep(Duration::from_secs(interval_seconds)).await;
            }
        }
        Command::SessionProbe { viewer_name } => {
            let response = client
                .post(format!("{}/api/v1/sessions/request", cli.broker))
                .json(&protocol::SessionRequest {
                    target_device_id: device_id,
                    viewer_name,
                    preferred_transport: SessionTransport::RelayBinary,
                })
                .send()
                .await
                .context("failed to request test session")?;

            println!("{}", response.text().await?);
        }
        Command::Version => {
            let target = host_target_for_current_platform();
            let version = version_for(target).expect("host release version should exist");
            println!("{} build {}", version.version, version.build);
        }
    }

    Ok(())
}

async fn register(
    client: &Client,
    broker: &str,
    device_id: &str,
    device_name: &str,
    platform: PlatformKind,
) -> anyhow::Result<()> {
    let request = RegisterHostRequest {
        device_id: device_id.to_string(),
        device_name: device_name.to_string(),
        platform,
        capabilities: vec![Capability::Screen, Capability::Input, Capability::Clipboard],
        owner_hint: Some("personal".to_string()),
        public_key: Some("placeholder-public-key".to_string()),
    };

    let response = client
        .post(format!("{}/api/v1/devices/register", broker))
        .json(&request)
        .send()
        .await
        .context("failed to register device")?;

    println!("{}", response.text().await?);
    Ok(())
}

async fn heartbeat(
    client: &Client,
    broker: &str,
    device_id: &str,
    active_session_count: u32,
) -> anyhow::Result<()> {
    let request = HeartbeatRequest {
        device_id: device_id.to_string(),
        active_session_count,
        hostname: Some("placeholder-hostname".to_string()),
        local_ips: vec!["127.0.0.1".to_string()],
    };

    let response = client
        .post(format!("{}/api/v1/devices/heartbeat", broker))
        .json(&request)
        .send()
        .await
        .context("failed to send heartbeat")?;

    println!("{}", response.text().await?);
    Ok(())
}
