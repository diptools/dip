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
use std::{collections::HashSet, fs, os::unix::fs::PermissionsExt, path::PathBuf};
use tokio::io::AsyncWriteExt;
>>>>>>> ced7a90 (Install standalone Tailwind CSS binary through version manager)
// Plugin
pub struct TailwindPlugin;

impl Plugin for TailwindPlugin {
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
        let tw = TailwindCSS::from(config.as_ref());

        println!("📌 Clean Tailwind CSS");
        if let Err(e) = tw.clean_all() {
            eprintln!("Failed to clean Tailwind CSS: {e}");
        } else {
            println!("✅ Clean Tailwind CSS");
        }
    });
}

fn apply(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        println!("📌　Install Tailwind CSS");
        let tw = TailwindCSS::from(config.as_ref());

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                if let Err(e) = tw.install_all().await {
                    eprintln!("Failed to install tailwind: {e}");
                } else {
                    println!("✅ Install Tailwind CSS");
                }
            });
    });
}

struct TailwindCSS {
    bundle_root: PathBuf,
    bundle: PathBuf,
    versions: HashSet<String>,
    platform: Platform,
}

impl TailwindCSS {
    async fn install(&self, version: &String) -> anyhow::Result<()> {
        let p = self.bin_path(version);

        if p.is_file() {
            return Ok(());
        }

        fs::create_dir_all(p.parent().context("Failed to get parent path")?)?;
        let download_url = self.download_url(version);

        let mut res = reqwest::get(&download_url)
            .await
            .with_context(|| format!("Failed to download tool: {}", Self::name()))?;

        if res.status() == StatusCode::NOT_FOUND {
            bail!("Download URL not found: {download_url}");
        }

        let mut file = tokio::fs::File::create(&p)
            .await
            .context("Failed to create download target file")?;
        file.set_permissions(fs::Permissions::from_mode(0o755))
            .await
            .context("Failed to give permission to download target file")?;
        while let Some(chunk) = res
            .chunk()
            .await
            .context("Failed to stream chunks of downloading content")?
        {
            file.write(chunk.as_ref())
                .await
                .context("Failed to write chunks of downloading content")?;
        }

        println!("Installed: {}", p.display());

        Ok(())
    }
}

impl Bundler for TailwindCSS {
    fn name() -> &'static str {
        "tailwindcss"
    }

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
    }
}

impl VersionManager for TailwindCSS {
    fn bundle_root(&self) -> &PathBuf {
        &self.bundle_root
    }

    fn versions(&self) -> &HashSet<String> {
        &self.versions
    }

    fn bin_name(&self) -> String {
        format!(
            "tailwindcss-{target}-{arch}{optional_ext}",
            target = self.platform.to_string(),
            arch = Platform::arch(),
            optional_ext = self.platform.ext(),
        )
    }

    fn download_url(&self, version: &String) -> String {
        format!(
            "https://github.com/tailwindlabs/tailwindcss/releases/download/v{version}/{bin_name}",
            bin_name = self.bin_name(),
        )
    }

    fn clean_all(&self) -> anyhow::Result<()> {
        if self.installs_dir().is_dir() {
            let installs =
                fs::read_dir(self.installs_dir()).context("Failed to read installs/ directory")?;

            installs
                .filter_map(Result::ok)
                .filter(|dir| dir.path().is_dir())
                .filter(|dir| {
                    let v = &dir
                        .path()
                        .file_name()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap();
                    !self.versions().contains(v)
                })
                .for_each(|dir| {
                    let path = dir.path();
                    if let Err(e) = fs::remove_dir_all(&path) {
                        eprintln!("Failed to cleanup directory: {e}");
                    } else {
                        println!("Cleaned: {}", path.display());
                    }
                });

            if fs::read_dir(self.installs_dir())?.next().is_none() {
                fs::remove_dir(self.installs_dir())
                    .context("Failed to clean empty installs directory")?;
            }
        }

        Ok(())
    }

    async fn install_all(&self) -> anyhow::Result<()> {
        let mut versions = self.versions().iter();
        while let Some(v) = versions.next() {
            if let Err(e) = self.install(v).await {
                eprintln!("Failed to install Tailwind CSS: {e}");
            };
        }

        Ok(())
    }
}

impl From<&BundleConfig> for TailwindCSS {
    fn from(config: &BundleConfig) -> Self {
        Self {
            bundle_root: config.bundle_root(),
            bundle: config.bundle_root().join("tailwindcss"),
            versions: config.vm.runtime.tailwindcss.clone(),
            platform: Platform::new(),
        }
>>>>>>> 51d7a93 (Parse path and url from config file)
    }
}

// Events

pub struct TailwindInstalled;
