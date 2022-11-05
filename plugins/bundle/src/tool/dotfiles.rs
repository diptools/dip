use crate::{ApplyBundle, BundleStage};
use bevy::{
    app::{App, Plugin},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::ParallelSystemDescriptorCoercion,
    },
    log,
};
use std::{env, fs, path::PathBuf};

// Plugin

pub struct DotfilesPlugin;

impl Plugin for DotfilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplyDotfiles>()
            .add_system_to_stage(BundleStage::Apply, apply)
            .add_system_to_stage(BundleStage::Apply, apply_dotfiles.after(apply));
    }
}

// Events

struct ApplyDotfiles {
    path: PathBuf,
}

fn apply(mut events: EventReader<ApplyBundle>, mut apply_dotfiles: EventWriter<ApplyDotfiles>) {
    events.iter().for_each(|e| {
        log::warn!("TODO: apply dotfiles");

        // Walk through ./bundle/dotfiles/*
        let current_path = env::current_dir().expect("Failed to get current directory.");
        let dotfiles_path = current_path.join(&e.path).join("bundle").join("dotfiles");

        if dotfiles_path.is_dir() {
            log::info!("📌");
            fs::read_dir(dotfiles_path)
                .unwrap()
                .filter_map(|path| {
                    if let Ok(path) = path {
                        if path.file_type().unwrap().is_dir() {
                            Some(path)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .for_each(|dir_entry| {
                    apply_dotfiles.send(ApplyDotfiles {
                        path: dir_entry.path(),
                    });
                });
        } else {
            log::info!(
                "dotfiles direcotry is empty: {}",
                &dotfiles_path.into_os_string().into_string().unwrap()
            );
            log::info!("🟡 Skip: Apply dotfiles");
        }
    });
}

fn apply_dotfiles(mut events: EventReader<ApplyDotfiles>) {
    events.iter().for_each(|e| {
        let path = &e.path.clone().into_os_string().into_string().unwrap();
        println!("{}", path);
    });
}
