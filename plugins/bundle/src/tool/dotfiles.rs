use crate::{ApplyBundle, Bundle, BundleStage, CleanBundle};
use bevy::{
    app::{App, Plugin},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::ParallelSystemDescriptorCoercion,
    },
};
use pathdiff::diff_paths;
use std::{
    fs::{self, DirEntry},
    os,
    path::PathBuf,
};
use walkdir::WalkDir;

// Plugin

pub struct DotfilesPlugin;

impl Plugin for DotfilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ApplySymlinks>()
            .add_system_to_stage(BundleStage::Apply, apply)
            .add_system_to_stage(BundleStage::Apply, apply_symlinks.after(apply))
            .add_system_to_stage(BundleStage::Clean, clean);
    }
}

// Events

struct ApplySymlinks {
    dotfiles_path: PathBuf,
    dir_entry: DirEntry,
}

fn apply(mut events: EventReader<ApplyBundle>, mut apply_dotfiles: EventWriter<ApplySymlinks>) {
    events.iter().for_each(|e| {
        let dotfiles = Dotfiles::from(e.clone());
        let dotfiles_path = &dotfiles.bundle_path();

        if dotfiles_path.is_dir() {
            println!("📌 Apply dotfiles");
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
            println!(
                "dotfiles direcotry is empty: {}",
                &dotfiles_path
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            );

            println!("🟡 Skip: Apply dotfiles");
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

struct Dotfiles {
    pub path: PathBuf,
}

impl Bundle for Dotfiles {
    fn bundle_path(&self) -> PathBuf {
        self.path.join("bundle/dotfiles")
    }
}

impl From<ApplyBundle> for Dotfiles {
    fn from(ApplyBundle { path }: ApplyBundle) -> Self {
        Self { path }
    }
}

impl From<CleanBundle> for Dotfiles {
    fn from(CleanBundle { path }: CleanBundle) -> Self {
        Self { path }
    }
}

fn clean(mut events: EventReader<CleanBundle>) {
    events.iter().for_each(|_e| {
        println!("hey");
    });
}

struct Symlink {
    original: PathBuf,
    link: PathBuf,
}

impl Symlink {
    fn apply(&self) {
        if self.link.is_symlink() {
            println!(
                "{}",
                &self.symlink_log_message(Some("🟡 Skip: File is already symlinked"))
            );
        } else if self.link.is_file() {
            println!(
                "{}",
                &self.symlink_log_message(Some("🟡 Skip: File already exists"))
            );
        } else {
            #[cfg(target_family = "unix")]
            match os::unix::fs::symlink(&self.original, &self.link) {
                Ok(_) => {
                    println!("{}", &self.symlink_log_message(None));
                }
                Err(e) => {
                    eprintln!("{}", &self.symlink_log_message(Some(&e.to_string())));
                }
            }

            #[cfg(target_family = "windows")]
            match os::windows::fs::symlink(&self.original, &self.link) {
                Ok(_) => {
                    println!("{}", &self.symlink_log_message(None));
                }
                Err(e) => {
                    printlne!("{}", &self.symlink_log_message(Some(&e.to_string())));
                }
            }
        }
    }

    fn symlink_log_message<'a>(&self, message: Option<&'a str>) -> String {
        let message = message.unwrap_or("".into());
        format!(
            "----------------------------------------------------------\n\
            {message}\n\
            original : {:?}\n\
            link     : {:?}",
            &self.original, &self.link,
        )
    }
}
