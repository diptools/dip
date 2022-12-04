use crate::{
    config::BundleConfig, platform::Platform, schedule::BundleStage, tool::vm::VersionManager,
    ApplyBundle, Bundler,
};
use bevy::{
    app::{App, Plugin},
    ecs::{event::EventReader, system::Res},
};
use dip_macro::Installer;
use std::{fs, os::unix::fs::PermissionsExt};

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

#[derive(Installer)]
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
    fn file_name(&self, version: &String) -> String {
        format!(
            "{}{optional_ext}",
            self.file_name_without_ext(version),
            optional_ext = self.platform.ext(),
        )
    }

    fn file_name_without_ext(&self, _version: &String) -> String {
        format!(
            "tailwindcss-{target}-{arch}",
            target = self.platform.to_string(),
            arch = Platform::arch(),
        )
    }

    fn download_file_name(&self, _file_name: &String) -> String {
        Self::key().to_string()
    }

    fn versions(&self) -> &Vec<String> {
        &self.bundle_config().runtime().tailwindcss
    }

    fn download_url(&self, version: &String) -> String {
        format!(
            "https://github.com/tailwindlabs/tailwindcss/releases/download/v{version}/{file_name}",
            file_name = &self.file_name(&version),
        )
    }

    fn checksum(&self, _version: &String) -> anyhow::Result<Option<String>> {
        Ok(None)
    }

    fn list_shims() -> Vec<&'static str> {
        vec!["tailwindcss"]
    }

    fn shim(&self, version: &String) -> anyhow::Result<()> {
        let bin_name = &Self::key();
        let runtime_path = self.version_dir(version).join(&bin_name);
        let shim_path = &self.shims_dir().join(&bin_name);

        fs::write(&shim_path, &Self::format_shim(&runtime_path)?.as_bytes())?;
        fs::set_permissions(&shim_path, fs::Permissions::from_mode(0o755))?;

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
