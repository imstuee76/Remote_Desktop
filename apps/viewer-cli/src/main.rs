use anyhow::Context;
use clap::{Parser, Subcommand};
use reqwest::Client;

#[derive(Parser, Debug)]
#[command(name = "viewer-cli")]
#[command(about = "Starter viewer CLI for smoke-testing the broker")]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:8080")]
    broker: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Devices,
    RequestSession {
        #[arg(long)]
        target_device_id: String,

        #[arg(long, default_value = "CLI Viewer")]
        viewer_name: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = Client::new();

    match cli.command {
        Command::Devices => {
            let response = client
                .get(format!("{}/api/v1/devices", cli.broker))
                .send()
                .await
                .context("failed to list devices")?;

            println!("{}", response.text().await?);
        }
        Command::RequestSession {
            target_device_id,
            viewer_name,
        } => {
            let response = client
                .post(format!("{}/api/v1/sessions/request", cli.broker))
                .json(&serde_json::json!({
                    "target_device_id": target_device_id,
                    "viewer_name": viewer_name,
                    "preferred_transport": "relay-binary"
                }))
                .send()
                .await
                .context("failed to request session")?;

            println!("{}", response.text().await?);
        }
    }

    Ok(())
}
