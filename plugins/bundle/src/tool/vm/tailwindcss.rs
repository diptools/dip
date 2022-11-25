<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
use crate::tool::InstallTools;
=======
use crate::{
    config::{BundleConfig, Config},
    schedule::BundleStage,
    ApplyBundle, CleanBundle,
};
>>>>>>> 051d114 (Create installs directory when it does not exist)
=======
use crate::{config::BundleConfig, schedule::BundleStage, ApplyBundle};
>>>>>>> e04d1b0 (Merge bundle config with cli arguments)
=======
use crate::{config::BundleConfig, schedule::BundleStage, ApplyBundle, Bundler};
>>>>>>> 51d7a93 (Parse path and url from config file)
use bevy::{
    app::{App, Plugin},
    ecs::event::{EventReader, EventWriter},
    log,
};
<<<<<<< HEAD
<<<<<<< HEAD
=======
use std::{fs, path::PathBuf};
>>>>>>> 051d114 (Create installs directory when it does not exist)
=======
use std::{collections::HashSet, fs, path::PathBuf};

use super::VersionManager;
>>>>>>> 51d7a93 (Parse path and url from config file)

=======
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
<<<<<<< HEAD
use std::{collections::HashSet, fs, os::unix::fs::PermissionsExt, path::PathBuf};
use tokio::io::AsyncWriteExt;
<<<<<<< HEAD:plugins/bundle/src/tool/vm/tailwind.rs
>>>>>>> ced7a90 (Install standalone Tailwind CSS binary through version manager)
// Plugin
pub struct TailwindPlugin;
=======
>>>>>>> cdc95b3 (Fetch compressed files for Node.js runtime):plugins/bundle/src/tool/vm/tailwindcss.rs
=======
use std::{collections::HashSet, fs, io::Write, os::unix::fs::PermissionsExt, path::PathBuf};
>>>>>>> f6c8a2b (Unpack Node.js runtime in installs/ directory)

pub struct TailwindCSSPlugin;

impl Plugin for TailwindCSSPlugin {
    fn build(&self, app: &mut App) {
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
        app.add_event::<TailwindInstalled>().add_system(install);
    }
}

fn install(mut events: EventReader<InstallTools>, mut installed: EventWriter<TailwindInstalled>) {
    for _e in events.iter() {
        log::warn!("TODO: Install Tool");

        installed.send(TailwindInstalled);
=======
        app.add_system_to_stage(BundleStage::Clean, clean)
=======
        app
            // .add_system_to_stage(BundleStage::Clean, clean)
>>>>>>> a47ed81 (Add ConfigStartupStage)
=======
        app.add_system_to_stage(BundleStage::Clean, clean)
>>>>>>> 51d7a93 (Parse path and url from config file)
            .add_system_to_stage(BundleStage::Apply, apply);
    }
}

fn clean(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let vm = TailwindCSS::from(config.as_ref());
        let action = format!("Clean {}", &TailwindCSS::name());

        println!("📌 {}", &action);
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
    versions: HashSet<String>,
    platform: Platform,
}

impl TailwindCSS {
    fn bin_name(&self) -> String {
        format!(
            "tailwindcss-{target}-{arch}{optional_ext}",
            target = self.platform.to_string(),
            arch = Platform::arch(),
            optional_ext = self.platform.ext(),
        )
    }
}

impl Bundler for TailwindCSS {
    fn key() -> &'static str {
        "tailwindcss"
    }

<<<<<<< HEAD
<<<<<<< HEAD
    fn bundle_dir(&self) -> PathBuf {
        self.config.repo().join("bundle/vm")
    }

    fn installs_dir(&self) -> PathBuf {
        self.config.installs_dir().join("tailwindcss")
    }

    fn versions(&self) -> std::collections::hash_set::Iter<'_, std::string::String> {
        self.config.vm.runtime.tailwindcss.iter()
>>>>>>> 051d114 (Create installs directory when it does not exist)
=======
    fn bundle(&self) -> &PathBuf {
        &self.bundle
=======
    fn name() -> &'static str {
        "Tailwind CSS"
    }

    fn bundle_dir(&self) -> &PathBuf {
        &self.bundle_dir
>>>>>>> f6c8a2b (Unpack Node.js runtime in installs/ directory)
    }
}

impl VersionManager for TailwindCSS {
    fn installs_dir(&self) -> &PathBuf {
        &self.installs_dir
    }

    fn versions(&self) -> &HashSet<String> {
        &self.versions
    }

    fn download_url(&self, version: &String) -> String {
        format!(
            "https://github.com/tailwindlabs/tailwindcss/releases/download/v{version}/{bin_name}",
            bin_name = self.bin_name(),
        )
    }

    fn install(&self, version: &String) -> anyhow::Result<()> {
        let download_url = self.download_url(version);

        let res = reqwest::blocking::get(&download_url)
            .with_context(|| format!("Failed to download tool: {}", &Self::key()))?;

        if res.status() == StatusCode::NOT_FOUND {
            bail!("Download URL not found: {download_url}");
        }

        let mut file = fs::File::create(self.version_dir(&version).join(Self::key()))
            .context("Failed to create download target file")?;
        file.set_permissions(fs::Permissions::from_mode(0o755))
            .context("Failed to give permission to download target file")?;
        file.write(&res.bytes()?)?;

        Ok(())
    }
}

impl From<&BundleConfig> for TailwindCSS {
    fn from(config: &BundleConfig) -> Self {
        Self {
            bundle_dir: config.bundle_root().join(Self::key()),
            installs_dir: config.install_root().join(Self::key()),
            versions: config.vm.runtime.tailwindcss.clone(),
            platform: Platform::new(),
        }
>>>>>>> 51d7a93 (Parse path and url from config file)
    }
}

// Events

pub struct TailwindInstalled;
