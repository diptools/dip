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
use std::{fs, io::Write, os::unix::fs::PermissionsExt};

pub struct TailwindCSSPlugin;

impl Plugin for TailwindCSSPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BundleStage::Clean, clean)
            .add_system_to_stage(BundleStage::Apply, apply);
    }
}

fn clean(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let vm = TailwindCSS::new(config.clone());
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
        let vm = TailwindCSS::new(config.clone());
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
    bundle_config: BundleConfig,
    platform: Platform,
}

impl TailwindCSS {
    fn new(bundle_config: BundleConfig) -> Self {
        Self {
            bundle_config,
            platform: Platform::new(),
        }
    }
}

impl Bundler for TailwindCSS {
    fn key() -> &'static str {
        "tailwindcss"
    }

    fn name() -> &'static str {
        "Tailwind CSS"
    }

    fn bundle_config(&self) -> &BundleConfig {
        &self.bundle_config
    }
}

impl VersionManager for TailwindCSS {
    fn versions(&self) -> &Vec<String> {
        &self.bundle_config().runtime().tailwindcss
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
            .context("Failed to download. Check internet connection.")?;

        match res.status() {
            StatusCode::NOT_FOUND => {
                bail!("Download URL not found: {download_url}");
            }
            StatusCode::OK => {
                let version_dir = self.version_dir(version);
                fs::create_dir_all(&version_dir)?;

                let mut file = fs::File::create(version_dir.join(Self::key()))
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

    fn list_shims() -> Vec<&'static str> {
        vec!["tailwindcss"]
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

    fn remove_shim(&self) -> anyhow::Result<()> {
        let shim_path = &self.shims_dir().join(&Self::key());
        if shim_path.is_file() {
            fs::remove_file(shim_path)?;
        }

        Ok(())
    }
}
