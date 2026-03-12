use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
    sync::mpsc::{self, Receiver},
    thread,
};

use eframe::egui;
use protocol::{ReleaseTargetId, ReleaseVersionInfo};
use release_meta::{host_target_for_current_platform, release_catalog, version_for};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root should be resolvable")
}

fn host_service_name() -> String {
    env::var("PRIVATE_REMOTE_HOST_SERVICE").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "PrivateRemoteHostd".to_string()
        } else {
            "private-remote-hostd".to_string()
        }
    })
}

fn run_script(script_name: &str, restart_only: bool) -> anyhow::Result<String> {
    let root = repo_root();

    let output = if cfg!(target_os = "windows") {
        let mut command = Command::new("powershell.exe");
        command
            .arg("-NoProfile")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-File")
            .arg(root.join("scripts").join(format!("{script_name}.ps1")));
        if restart_only {
            command.arg("-RestartOnly");
        }
        command.current_dir(&root).output()?
    } else {
        let mut command = Command::new("bash");
        command.arg(root.join("scripts").join(format!("{script_name}.sh")));
        if restart_only {
            command.arg("--restart-only");
        }
        command.current_dir(&root).output()?
    };

    let mut log = String::new();
    log.push_str(&String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        if !log.is_empty() {
            log.push('\n');
        }
        log.push_str(&String::from_utf8_lossy(&output.stderr));
    }

    if output.status.success() {
        Ok(log)
    } else {
        Err(anyhow::anyhow!(log))
    }
}

enum WorkerMessage {
    Completed { ok: bool, log: String },
}

struct DesktopControlApp {
    current_target: ReleaseTargetId,
    current_version: ReleaseVersionInfo,
    worker: Option<Receiver<WorkerMessage>>,
    busy: bool,
    status: String,
    log_output: String,
}

impl DesktopControlApp {
    fn new() -> Self {
        let current_target = host_target_for_current_platform();
        let current_version = version_for(current_target.clone()).expect("current target version");
        Self {
            current_target,
            current_version,
            worker: None,
            busy: false,
            status: "Ready".to_string(),
            log_output: String::new(),
        }
    }

    fn start_script(&mut self, restart_only: bool) {
        if self.busy {
            return;
        }

        self.busy = true;
        self.status = if restart_only {
            "Restarting host service...".to_string()
        } else {
            "Updating from git and restarting host service...".to_string()
        };

        let (tx, rx) = mpsc::channel();
        self.worker = Some(rx);

        thread::spawn(move || {
            let result = run_script("update-host", restart_only);
            let message = match result {
                Ok(log) => WorkerMessage::Completed { ok: true, log },
                Err(error) => WorkerMessage::Completed {
                    ok: false,
                    log: error.to_string(),
                },
            };
            let _ = tx.send(message);
        });
    }

    fn poll_worker(&mut self) {
        if let Some(receiver) = &self.worker {
            if let Ok(message) = receiver.try_recv() {
                match message {
                    WorkerMessage::Completed { ok, log } => {
                        self.busy = false;
                        self.status = if ok {
                            "Host update finished.".to_string()
                        } else {
                            "Host update failed.".to_string()
                        };
                        self.log_output = log;
                        self.worker = None;
                    }
                }
            }
        }
    }

    fn current_platform_label(&self) -> &'static str {
        match self.current_target {
            ReleaseTargetId::WindowsHost => "Windows host",
            ReleaseTargetId::LinuxHost => "Linux host",
            ReleaseTargetId::AndroidViewer => "Android viewer",
            ReleaseTargetId::BrokerServer => "Broker server",
        }
    }
}

impl eframe::App for DesktopControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll_worker();

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Menu", |ui| {
                    if ui
                        .add_enabled(!self.busy, egui::Button::new("Update from Git"))
                        .clicked()
                    {
                        self.start_script(false);
                        ui.close();
                    }
                    if ui
                        .add_enabled(!self.busy, egui::Button::new("Restart Host Service"))
                        .clicked()
                    {
                        self.start_script(true);
                        ui.close();
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Private Remote Desktop Control");
            ui.label(format!(
                "Target: {}",
                self.current_platform_label()
            ));
            ui.label(format!(
                "Version: {} (build {})",
                self.current_version.version, self.current_version.build
            ));
            ui.label(format!("Host service: {}", host_service_name()));
            ui.label(format!("Status: {}", self.status));
            ui.separator();
            ui.heading("Platform Versions");

            egui::Grid::new("release_grid").striped(true).show(ui, |ui| {
                ui.strong("Target");
                ui.strong("Version");
                ui.strong("Build");
                ui.end_row();

                for entry in &release_catalog().targets {
                    ui.label(match entry.target {
                        ReleaseTargetId::WindowsHost => "Windows host",
                        ReleaseTargetId::LinuxHost => "Linux host",
                        ReleaseTargetId::AndroidViewer => "Android viewer",
                        ReleaseTargetId::BrokerServer => "Broker server",
                    });
                    ui.label(&entry.version);
                    ui.label(entry.build.to_string());
                    ui.end_row();
                }
            });

            ui.separator();
            ui.heading("Update Log");
            ui.add(
                egui::TextEdit::multiline(&mut self.log_output)
                    .desired_rows(16)
                    .desired_width(f32::INFINITY),
            );
        });

        if self.busy {
            ctx.request_repaint_after(std::time::Duration::from_millis(200));
        }
    }
}

fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Private Remote Desktop Control",
        native_options,
        Box::new(|_cc| Ok(Box::new(DesktopControlApp::new()))),
    )
    .map_err(|error| anyhow::anyhow!(error.to_string()))
}
