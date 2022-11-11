use crate::{ApplyBundle, BundleStage, CleanBundle};
use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
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
        app.add_system_to_stage(BundleStage::Apply, apply)
            .add_system_to_stage(BundleStage::Clean, clean);
    }
}

// Systems

fn apply(mut events: EventReader<ApplyBundle>) {
    events.iter().for_each(|e| {
        let dotfiles = Dotfiles::from(e.clone());

        if dotfiles.bundle_exists() {
            println!("📌 Apply dotfiles");

            dotfiles.symlinks().for_each(|sym| sym.apply());
        } else {
            println!("🟡 Skip: Apply dotfiles");
            println!("bundle/dotfiles directory is empty",);
        }

        println!("✅ Apply dotfiles");
    });
}

fn clean(mut events: EventReader<CleanBundle>) {
    events.iter().for_each(|e| {
        let dotfiles = Dotfiles::from(e.clone());

        if dotfiles.bundle_exists() {
            println!("📌 Clean dotfiles");

            dotfiles.symlinks().for_each(|sym| sym.clean());
        } else {
            println!("🟡 Skip: Clean dotfiles");
            println!("bundle/dotfiles directory is empty",);
        }

        println!("✅ Clean dotfiles");
    });
}

struct Dotfiles {
    pub path: PathBuf,
}

impl Dotfiles {
    fn bundle_path(&self) -> PathBuf {
        self.path.join("bundle/dotfiles")
    }

    fn bundle_exists(&self) -> bool {
        self.bundle_path().is_dir()
    }

    fn symlinks(&self) -> std::boxed::Box<dyn Iterator<Item = Symlink> + '_> {
        Box::new(
            self.packages()
                .flat_map(|dir| WalkDir::new(&dir.path().into_iter()))
                .filter_map(Result::ok)
                .filter_map(|dir| {
                    let original = dir.path().to_path_buf().canonicalize().unwrap();
                    let diff = diff_paths(dir.path(), &self.bundle_path()).unwrap();
                    let dotfile_bundle_name = diff.iter().next().unwrap();
                    let stripped = diff.strip_prefix(dotfile_bundle_name).unwrap();
                    let link = dirs::home_dir().unwrap().join(stripped);

                    if dir.file_type().is_dir() {
                        fs::create_dir_all(link).unwrap();
                        None
                    } else {
                        Some(Symlink { original, link })
                    }
                }),
        )
    }

    fn packages(&self) -> std::boxed::Box<dyn Iterator<Item = DirEntry> + '_> {
        let dir = fs::read_dir(&self.bundle_path())
            .unwrap()
            .filter_map(Result::ok);

        Box::new(dir)
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

struct Symlink {
    original: PathBuf,
    link: PathBuf,
}

impl Symlink {
    fn apply(&self) {
        if self.link.is_symlink() {
            // println!(
            //     "{}",
            //     &self.format("🟡 Skip: File is already symlinked")
            // );
        } else if self.link.is_file() {
            // println!("{}", &self.format("🟡 Skip: File already exists"));
        } else {
            #[cfg(target_family = "unix")]
            let res = os::unix::fs::symlink(&self.original, &self.link);

            #[cfg(target_family = "windows")]
            let res = os::windows::fs::symlink(&self.original, &self.link);

            match res {
                Ok(_) => {
                    println!("{}", &self.format("Symlink created"));
                }
                Err(e) => {
                    eprintln!("{}", &self.format(&e.to_string()));
                }
            }
        }
    }

    fn clean(&self) {
        if self.link.is_symlink() {
            match fs::remove_file(&self.link) {
                Ok(_) => {
                    println!("{}", &self.format("Symlink removed"));
                }
                Err(e) => {
                    eprintln!("{}", &self.format(&e.to_string()));
                }
            }
        }
    }

    fn format<'a>(&self, message: &'a str) -> String {
        format!(
            "----------------------------------------------------------\n\
            {message}\n\
            original : {:?}\n\
            link     : {:?}",
            &self.original, &self.link,
        )
    }
}
