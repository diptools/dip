use crate::{ApplyBundle, BundleStage};
use bevy::{
    app::{App, Plugin},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::ParallelSystemDescriptorCoercion,
    },
    log,
};
use pathdiff::diff_paths;
use std::{fs::{self, DirEntry}, os, path::PathBuf};
use walkdir::{WalkDir};

// Plugin

pub struct DotfilesPlugin;

impl Plugin for DotfilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplySymlinks>()
            .add_system_to_stage(BundleStage::Apply, apply)
            .add_system_to_stage(BundleStage::Apply, apply_symlinks.after(apply));
    }
}

// Events

struct ApplySymlinks {
    dotfiles_path: PathBuf,
    dir_entry: DirEntry,
}

fn apply(mut events: EventReader<ApplyBundle>, mut apply_dotfiles: EventWriter<ApplySymlinks>) {
    events.iter().for_each(|e| {
        let dotfiles_path = &e.path.join("bundle").join("dotfiles");

        if dotfiles_path.is_dir() {
            log::info!("📌 Apply dotfiles");
            fs::read_dir(dotfiles_path)
                .unwrap()
                .filter(|entry| entry.is_ok())
                .filter_map(Result::ok)
                .for_each(|dir_entry| {
                    apply_dotfiles.send(ApplySymlinks {
                        dotfiles_path: dotfiles_path.to_path_buf(),
                        dir_entry,
                    });
                });
        } else {
            log::info!(
                "dotfiles direcotry is empty: {}",
                &dotfiles_path
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            );
            log::info!("🟡 Skip: Apply dotfiles");
        }
    });
}

fn apply_symlinks(mut events: EventReader<ApplySymlinks>) {
    events.iter().for_each(|e| {
        WalkDir::new(&e.dir_entry.path())
            .into_iter()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                let original = entry.path().to_path_buf().canonicalize().unwrap();
                let diff = diff_paths(entry.path(), &e.dotfiles_path).unwrap();
                let dotfile_bundle_name = diff.iter().next().unwrap();
                let stripped = diff.strip_prefix(dotfile_bundle_name).unwrap();
                let link = dirs::home_dir().unwrap().join(stripped);

                if entry.file_type().is_dir() {
                    fs::create_dir_all(link).unwrap();
                    None
                } else {
                    Some(Symlink { original, link })
                }
            })
            .for_each(|symlink| symlink.apply());
    });
}

struct Symlink {
    original: PathBuf,
    link: PathBuf,
}

impl Symlink {
    fn apply(&self) {
        if self.link.is_symlink() {
            log::info!("----------------------------------------------------------");
            log::info!("🟡 Skip: File is already symlinked");
            log::info!("original : {:?}", self.original);
            log::info!("link     : {:?}", self.link);
        }
        else if self.link.is_file() {
            log::info!("----------------------------------------------------------");
            log::info!("🟡 Skip: File already exists");
            log::info!("original : {:?}", self.original);
            log::info!("link     : {:?}", self.link);
        } else {
            #[cfg(target_family = "unix")]
            match os::unix::fs::symlink(&self.original, &self.link) {
                Ok(_) => {
                    log::info!("----------------------------------------------------------");
                    log::info!("original : {:?}", self.original);
                    log::info!("link     : {:?}", self.link);
                }
                Err(e) => {
                    log::error!("----------------------------------------------------------");
                    log::error!("{e}");
                    log::error!("original : {:?}", self.original);
                    log::error!("link     : {:?}", self.link);
                }
            }

            #[cfg(target_family = "windows")]
            match os::windows::fs::symlink(&self.original, &self.link) {
                Ok(_) => {
                    log::info!("----------------------------------------------------------");
                    log::info!("original : {:?}", self.original);
                    log::info!("link     : {:?}", self.link);
                }
                Err(e) => {
                    log::error!("----------------------------------------------------------");
                    log::error!("{e}");
                    log::error!("original : {:?}", self.original);
                    log::error!("link     : {:?}", self.link);
                }
            }
        }
    }
}
