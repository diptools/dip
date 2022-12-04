use crate::{
    config::BundleConfig, platform::Platform, schedule::BundleStage, tool::vm::VersionManager,
    ApplyBundle, Bundler,
};
use anyhow::{bail, Context};
use bevy::{
    app::{App, Plugin},
    ecs::{event::EventReader, system::Res},
};
use dip_macro::Installer;
use std::{fs, io::Write, os::unix::fs::PermissionsExt};

pub struct NodeJSPlugin;

impl Plugin for NodeJSPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BundleStage::Clean, clean)
            .add_system_to_stage(BundleStage::Apply, apply);
    }
}

fn clean(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let vm = NodeJS::new(config.clone());
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
        let vm = NodeJS::new(config.clone());
        let action = format!("Apply {}", &NodeJS::name());

        println!("📌 {}", &action);
        if let Err(e) = vm.install_all() {
            eprintln!("Failed to install Node.js: {e}");
        } else {
            println!("✅ {}", &action);
        }
    });
}

#[derive(Installer)]
struct NodeJS {
    bundle_config: BundleConfig,
    platform: Platform,
}

impl NodeJS {
    fn new(bundle_config: BundleConfig) -> Self {
        Self {
            bundle_config,
            platform: Platform::new(),
        }
    }

    fn version_url(&self, version: &String) -> String {
        format!("https://nodejs.org/dist/v{version}")
    }
}

impl Bundler for NodeJS {
    fn key() -> &'static str {
        "nodejs"
    }

    fn name() -> &'static str {
        "Node.js"
    }

    fn bundle_config(&self) -> &BundleConfig {
        &self.bundle_config
    }
}

impl VersionManager for NodeJS {
    fn file_name(&self, version: &String) -> String {
        format!(
            "{}{archive_ext}",
            self.file_name_without_ext(version),
            archive_ext = self.platform.archive_ext(),
        )
    }

    fn file_name_without_ext(&self, version: &String) -> String {
        format!(
            "node-v{version}-{name}-{arch}",
            name = self.platform.name(),
            arch = Platform::arch(),
        )
    }

    fn download_file_name(&self, version: &String) -> String {
        self.file_name_without_ext(version)
    }

    fn download_url(&self, version: &String) -> String {
        format!(
            "{version_url}/{file_name}",
            version_url = &self.version_url(version),
            file_name = &self.file_name(&version),
        )
    }

    fn versions(&self) -> &Vec<String> {
        &self.bundle_config().runtime().nodejs
    }

    fn checksum(&self, version: &String) -> anyhow::Result<Option<String>> {
        let url = format!(
            "{version_url}/SHASUMS256.txt",
            version_url = &self.version_url(version)
        );
        let res = reqwest::blocking::get(&url)
            .context("Failed to fetch checksum. Check internet connection.")?;

        match res
            .text()?
            .lines()
            .find(|ln| ln.contains(&self.file_name(version)))
        {
            Some(ln) => {
                let checksum = ln.split("  ").next().context("Cannot find checksum")?;
                Ok(Some(checksum.to_string()))
            }
            None => {
                bail!("Cannot find checksum");
            }
        }
    }

    fn shim(&self, version: &String) -> anyhow::Result<()> {
        for (bin_name, bin_path) in Self::list_shims()
            .iter()
            .map(|bin_name| {
                (
                    bin_name,
                    self.version_dir(version).join("bin").join(bin_name),
                )
            })
            .filter(|(_bin_name, bin_path)| bin_path.is_file())
        {
            let shim_path = &self.shims_dir().join(&bin_name);

            let mut shim_file = fs::File::create(shim_path)?;
            shim_file
                .set_permissions(fs::Permissions::from_mode(0o755))
                .context("Failed to give permission to shim")?;

            shim_file.write_all(&Self::format_shim(&bin_path)?.as_bytes())?;
        }

        Ok(())
    }

    fn list_shims() -> Vec<&'static str> {
        vec!["corepack", "node", "npm", "npx"]
    }

    fn remove_shim(&self) -> anyhow::Result<()> {
        for p in self.shim_paths().iter().filter(|p| p.is_file()) {
            fs::remove_file(&p)?;
        }

        Ok(())
    }
}
