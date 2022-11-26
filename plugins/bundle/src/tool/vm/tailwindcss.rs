use crate::{
    config::BundleConfig, platform::Platform, schedule::BundleStage, tool::vm::VersionManager,
    ApplyBundle, Bundler,
};
use anyhow::{bail, Context};
use bevy::{
    app::{App, Plugin},
    ecs::{event::EventReader, system::Res},
};
use reqwest::StatusCode;
use std::{fs, io::Write, os::unix::fs::PermissionsExt, path::PathBuf};

pub struct TailwindCSSPlugin;

impl Plugin for TailwindCSSPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BundleStage::Clean, clean)
            .add_system_to_stage(BundleStage::Apply, apply);
    }
}

fn clean(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let vm = TailwindCSS::from(config.as_ref());
        let action = format!("Clean {}", &TailwindCSS::name());

        println!("🫧  {}", &action);
        if let Err(e) = vm.clean_all() {
            eprintln!("Failed to clean {}: {e}", TailwindCSS::key());
        } else {
            println!("✅ {}", &action);
        }
    });
}

fn apply(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let vm = TailwindCSS::from(config.as_ref());
        let action = format!("Apply {}", &TailwindCSS::name());

        println!("📌 {}", &action);
        if let Err(e) = vm.install_all() {
            eprintln!("Failed to install Tailwind CSS: {e}");
        } else {
            println!("✅ {}", &action);
        }
    });
}

struct TailwindCSS {
    bundle_dir: PathBuf,
    installs_dir: PathBuf,
    shims_dir: PathBuf,
    versions: Vec<String>,
    platform: Platform,
}

impl Bundler for TailwindCSS {
    fn key() -> &'static str {
        "tailwindcss"
    }

    fn name() -> &'static str {
        "Tailwind CSS"
    }

    fn bundle_dir(&self) -> &PathBuf {
        &self.bundle_dir
    }
}

impl VersionManager for TailwindCSS {
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
            "https://github.com/tailwindlabs/tailwindcss/releases/download/v{version}/tailwindcss-{target}-{arch}{optional_ext}",
            target = self.platform.to_string(),
            arch = Platform::arch(),
            optional_ext = self.platform.ext(),
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
                let mut file = fs::File::create(self.version_dir(&version).join(Self::key()))
                    .context("Failed to create download target file")?;
                file.set_permissions(fs::Permissions::from_mode(0o755))
                    .context("Failed to give permission to download target file")?;

                file.write(&res.bytes()?)?;

                Ok(())
            }
            _ => {
                bail!("Fail to download binary")
            }
        }
    }

    fn shim(&self, version: &String) -> anyhow::Result<()> {
        let bin_name = &Self::key();
        let runtime_path = self.version_dir(version).join(&bin_name);
        let shim_path = &self.shims_dir().join(&bin_name);

        let mut shim_file = fs::File::create(shim_path)?;
        shim_file
            .set_permissions(fs::Permissions::from_mode(0o755))
            .context("Failed to give permission to shim")?;

        shim_file.write_all(&Self::format_shim(&runtime_path)?.as_bytes())?;

        Ok(())
    }
}

impl From<&BundleConfig> for TailwindCSS {
    fn from(config: &BundleConfig) -> Self {
        Self {
            bundle_dir: config.bundle_root().join(Self::key()),
            installs_dir: config.install_root().join(Self::key()),
            shims_dir: config.shim_root(),
            versions: config.vm.runtime.tailwindcss.clone(),
            platform: Platform::new(),
        }
    }
}
