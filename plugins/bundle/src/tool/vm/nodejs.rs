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
use std::{
    collections::HashSet,
    io::Write,
    // fs, os::unix::fs::PermissionsExt,
    path::PathBuf,
};
use tempfile::tempfile;
// use tokio::io::AsyncWriteExt;

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

        println!("📌 Clean Node.js");
        if let Err(e) = vm.clean_all() {
            eprintln!("Failed to clean {}: {e}", NodeJS::name());
        } else {
            println!("✅ Clean Node.js");
        }
    });
}

fn apply(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let vm = NodeJS::from(config.as_ref());

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                println!("📌 Install Node.js");
                if let Err(e) = vm.install_all().await {
                    eprintln!("Failed to install Node.js: {e}");
                } else {
                    println!("✅ Install Node.js");
                }
            });
    });
}

struct NodeJS {
    bundle_root: PathBuf,
    bundle: PathBuf,
    versions: HashSet<String>,
    platform: Platform,
}

impl Bundler for NodeJS {
    fn name() -> &'static str {
        "nodejs"
    }

    fn bundle(&self) -> &PathBuf {
        &self.bundle
    }
}

impl VersionManager for NodeJS {
    fn bundle_root(&self) -> &PathBuf {
        &self.bundle_root
    }

    fn versions(&self) -> &HashSet<String> {
        &self.versions
    }

    fn download_url(&self, version: &String) -> String {
        format!(
            "https://nodejs.org/dist/v{version}/node-v{version}-{name}-{arch}{compression_ext}",
            name = self.platform.name(),
            arch = Platform::arch(),
            compression_ext = self.platform.compression_ext(),
        )
    }

    async fn install(&self, version: &String) -> anyhow::Result<()> {
        let download_url = self.download_url(version);

        let mut res = reqwest::get(&download_url)
            .await
            .with_context(|| format!("Failed to download tool: {}", &Self::name()))?;

        if res.status() == StatusCode::NOT_FOUND {
            bail!("Download URL not found: {download_url}");
        }

        println!("{}", download_url);
        println!("{:#?}", res.status());
        println!("{res:#?}");

        let mut file = tempfile()?;

        // let mut file = tokio::fs::File::create(&p)
        //     .await
        //     .context("Failed to create download target file")?;
        // file.set_permissions(fs::Permissions::from_mode(0o755))
        //     .await
        //     .context("Failed to give permission to download target file")?;
        while let Some(chunk) = res
            .chunk()
            .await
            .context("Failed to stream chunks of downloading content")?
        {
            file.write(chunk.as_ref())
                .context("Failed to write chunks of downloading content")?;
        }

        println!("{:?}", &file);

        Ok(())
    }
}

impl From<&BundleConfig> for NodeJS {
    fn from(config: &BundleConfig) -> Self {
        Self {
            bundle_root: config.bundle_root(),
            bundle: config.bundle_root().join("nodejs"),
            versions: config.vm.runtime.nodejs.clone(),
            platform: Platform::new(),
        }
    }
}
