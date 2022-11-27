#[cfg(feature = "nodejs")]
mod nodejs;
#[cfg(feature = "tailwindcss")]
mod tailwindcss;

#[cfg(feature = "nodejs")]
use nodejs::NodeJSPlugin;

#[cfg(feature = "tailwindcss")]
use tailwindcss::TailwindCSSPlugin;

use crate::Bundler;
use anyhow::Context;
use bevy::app::{App, Plugin};
use std::{fs, path::PathBuf};

pub struct VersionManagerPlugin;

impl Plugin for VersionManagerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "tailwindcss")]
        app.add_plugin(TailwindCSSPlugin);

        #[cfg(feature = "nodejs")]
        app.add_plugin(NodeJSPlugin);
    }
}

pub trait VersionManager: Bundler {
    fn installs_dir(&self) -> PathBuf {
        self.bundle_config().install_root().join(Self::key())
    }

    fn shims_dir(&self) -> PathBuf {
        self.bundle_config().shim_root()
    }

    fn versions(&self) -> &Vec<String>;

    fn version_dir(&self, version: &String) -> PathBuf {
        self.installs_dir().join(version)
    }

    fn download_url(&self, version: &String) -> String;

    fn install(&self, version: &String) -> anyhow::Result<()>;

    fn list_shims() -> Vec<&'static str>;

    fn shim_paths(&self) -> Vec<PathBuf> {
        Self::list_shims()
            .iter()
            .map(|bin| self.shims_dir().join(bin))
            .collect()
    }

    /// Create shim file: e.g. $DATA_DIR/dip/bundle/shims/node
    fn shim(&self, version: &String) -> anyhow::Result<()>;

    fn remove_shim(&self) -> anyhow::Result<()>;

    fn format_shim(path: &PathBuf) -> anyhow::Result<String> {
        let sh = format!(
            "#!/bin/sh\n\
            \"{}\" \"$@\"\
        ",
            &path.canonicalize()?.display()
        );

        Ok(sh)
    }

    /// Iterate over version set defined in user config. Install only if bin doesn't exist.
    fn install_all(&self) -> anyhow::Result<()> {
        for v in self.versions().iter() {
            let p = self.version_dir(v);

            // Skip install if the version already exists
            if p.is_dir() {
                continue;
            }

            self.install(v)?;
            println!("Installed: {}", &p.display());
        }

        // Create shim with default version
        self.shim(
            self.versions()
                .first()
                .context("Cannot find find any runtime versions")?,
        )?;

        Ok(())
    }

    /// Iterate over each versions currently installed but removed from the user bundle config
    fn clean_all(&self) -> anyhow::Result<()> {
        if self.installs_dir().is_dir() {
            let installs = fs::read_dir(self.installs_dir())?;

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

        self.remove_shim()?;

        Ok(())
    }
}
