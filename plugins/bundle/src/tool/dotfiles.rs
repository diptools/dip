use crate::{ApplyBundle, BundleStage};
use bevy::{
    app::{App, Plugin},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::ParallelSystemDescriptorCoercion,
    },
    log,
};
use std::{env, fs, os::unix, path::PathBuf};
use walkdir::WalkDir;

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
    path: PathBuf,
}

fn apply(mut events: EventReader<ApplyBundle>, mut apply_dotfiles: EventWriter<ApplySymlinks>) {
    events.iter().for_each(|e| {
        // Walk through ./bundle/dotfiles/*
        // let current_path = env::current_dir().expect("Failed to get current directory.");
        // let dotfiles_path = current_path.join(&e.path).join("bundle").join("dotfiles");
        let dotfiles_path = &e.path.join("bundle").join("dotfiles");

        if dotfiles_path.is_dir() {
            log::info!("📌 Apply dotfiles");
            fs::read_dir(dotfiles_path)
                .unwrap()
                .filter(|entry| entry.is_ok())
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().unwrap().is_dir())
                .for_each(|dir_entry| {
                    apply_dotfiles.send(ApplySymlinks {
                        path: dir_entry.path(),
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
        log::warn!("TODO: Get target path based on entry");

        let path = &e.path.clone().into_os_string().into_string().unwrap();

        WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .for_each(|entry| {
                println!("----------------------------------------------------------");
                println!("entry : {:?}\ntarget: {:?}", entry.path(), entry.path());
                // let target_path = dirs::home_dir().unwrap().join(entry.path);

                // #[cfg(target_os = "unix")]
                // unix::fs::symlink(entry.path(), link)
            });
    });
}
