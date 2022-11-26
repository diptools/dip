use crate::{
    config::BundleConfig, platform::Platform, schedule::BundleStage, tool::vm::VersionManager,
    ApplyBundle, Bundler,
};
use anyhow::{bail, Context};
use bevy::{
    app::{App, Plugin},
    ecs::{event::EventReader, system::Res},
};
use flate2::read::GzDecoder;
use reqwest::StatusCode;
use std::{fs, io::Write, os::unix::fs::PermissionsExt, path::PathBuf};
use tar::Archive;

pub struct NodeJSPlugin;

impl Plugin for NodeJSPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BundleStage::Clean, clean)
            .add_system_to_stage(BundleStage::Apply, apply);
    }
}

fn clean(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let vm = NodeJS::from(config.as_ref());
        let action = format!("Clean {}", &NodeJS::name());

        println!("🫧  {}", &action);
        if let Err(e) = vm.clean_all() {
            eprintln!("Failed to clean {}: {e}", NodeJS::key());
        } else {
            println!("✅ {}", &action);
        }
    });
}

fn apply(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let vm = NodeJS::from(config.as_ref());
        let action = format!("Apply {}", &NodeJS::name());

        println!("📌 {}", &action);
        if let Err(e) = vm.install_all() {
            eprintln!("Failed to install Node.js: {e}");
        } else {
            println!("✅ {}", &action);
        }
    });
}

struct NodeJS {
    bundle_dir: PathBuf,
    installs_dir: PathBuf,
    shims_dir: PathBuf,
    versions: Vec<String>,
    platform: Platform,
}

impl Bundler for NodeJS {
    fn key() -> &'static str {
        "nodejs"
    }

    fn name() -> &'static str {
        "Node.js"
    }

    fn bundle_dir(&self) -> &PathBuf {
        &self.bundle_dir
    }
}

impl NodeJS {
    fn file_name_without_ext(&self, version: &String) -> String {
        format!(
            "node-v{version}-{name}-{arch}",
            name = self.platform.name(),
            arch = Platform::arch(),
        )
    }

    fn file_name(&self, version: &String) -> String {
        format!(
            "{}{archive_ext}",
            self.file_name_without_ext(version),
            archive_ext = self.platform.archive_ext(),
        )
    }
}

impl VersionManager for NodeJS {
    fn installs_dir(&self) -> &PathBuf {
        &self.installs_dir
    }

    fn shims_dir(&self) -> &PathBuf {
        &self.shims_dir
    }

    fn versions(&self) -> &Vec<String> {
        &self.versions
    }

    fn download_url(&self, version: &String) -> String {
        format!(
            "https://nodejs.org/dist/v{version}/{file_name}",
            file_name = &self.file_name(&version),
        )
    }

    fn install(&self, version: &String) -> anyhow::Result<()> {
        let download_url = self.download_url(version);

        let res = reqwest::blocking::get(&download_url)
            .with_context(|| format!("Failed to download tool: {}", &Self::key()))?;

        match res.status() {
            StatusCode::NOT_FOUND => {
                bail!("Download URL not found: {download_url}");
            }
            StatusCode::OK => {
                if res.status() == StatusCode::NOT_FOUND {
                    bail!("Download URL not found: {download_url}");
                }
                let bytes = res.bytes()?;

                if cfg!(unix) {
                    let tar = GzDecoder::new(&bytes[..]);
                    let mut archive = Archive::new(tar);

                    archive.unpack(&self.installs_dir())?;
                    fs::rename(
                        &self
                            .installs_dir()
                            .join(&self.file_name_without_ext(&version)),
                        &self.installs_dir().join(&version),
                    )?;
                } else if cfg!(windows) {
                    // win: zip
                    todo!("Implement zip extraction logic for Windows");
                }

                Ok(())
            }
            _ => {
                bail!("Fail to download binary")
            }
        }
    }

    fn shim(&self, version: &String) -> anyhow::Result<()> {
        let runtime_path = self.version_dir(version).join("bin");
        for e in fs::read_dir(&runtime_path)?.flat_map(Result::ok) {
            if e.path().is_file() {
                let shim_path = &self.shims_dir().join(e.file_name());

                let mut shim_file = fs::File::create(shim_path)?;
                shim_file
                    .set_permissions(fs::Permissions::from_mode(0o755))
                    .context("Failed to give permission to shim")?;

                shim_file.write_all(&Self::format_shim(&e.path())?.as_bytes())?;
            }
        }
        Ok(())
    }
}

impl From<&BundleConfig> for NodeJS {
    fn from(config: &BundleConfig) -> Self {
        Self {
            bundle_dir: config.bundle_root().join(Self::key()),
            installs_dir: config.install_root().join(Self::key()),
            shims_dir: config.shim_root(),
            versions: config.vm.runtime.nodejs.clone(),
            platform: Platform::new(),
        }
    }
}
