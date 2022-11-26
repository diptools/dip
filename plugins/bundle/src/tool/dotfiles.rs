use crate::{ApplyBundle, BundleConfig, BundleStage, Bundler, CleanBundle};
use bevy::{
    app::{App, Plugin},
    ecs::{event::EventReader, system::Res},
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
            .add_system_to_stage(BundleStage::Apply, clean);
    }
}

// Systems

fn apply(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let dotfiles = Dotfiles::from(config.clone());
        let action = format!("Apply {}", &Dotfiles::name());

        if dotfiles.bundle_exists() {
            println!("📌 {}", &action);

            dotfiles.symlinks().for_each(|sym| sym.apply());
        } else {
            println!("🟡 Skip: {}", &action);
            println!("bundle/dotfiles directory is empty",);
        }

        println!("✅ {}", &action);
    });
}

fn clean(mut events: EventReader<CleanBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let dotfiles = Dotfiles::from(config.clone());
        let action = format!("Apply {}", &Dotfiles::name());

        if dotfiles.bundle_exists() {
            println!("📌 {}", &action);

            dotfiles.symlinks().for_each(|sym| sym.clean());
        } else {
            println!("🟡 Skip: {}", &action);
            println!("🟡 Skip {}: bundle/dotfiles directory is empty", &action);
        }

        println!("✅ {}", &action);
    });
}

struct Dotfiles {
    pub bundle_dir: PathBuf,
}

impl Bundler for Dotfiles {
    fn key() -> &'static str {
        "dotfiles"
    }

    fn name() -> &'static str {
        "dotfiles"
    }

    fn bundle_dir(&self) -> &PathBuf {
        &self.bundle_dir
    }
}

impl Dotfiles {
    fn symlinks(&self) -> Box<dyn Iterator<Item = Symlink> + '_> {
        Box::new(
            self.packages()
                .flat_map(|dir| WalkDir::new(&dir.path().into_iter()))
                .filter_map(Result::ok)
                .filter_map(|dir| {
                    let original = dir.path().to_path_buf().canonicalize().unwrap();
                    let diff = diff_paths(dir.path(), &self.bundle_dir()).unwrap();
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
        let dir = fs::read_dir(&self.bundle_dir())
            .unwrap()
            .filter_map(Result::ok);

        Box::new(dir)
    }
}

impl From<BundleConfig> for Dotfiles {
    fn from(config: BundleConfig) -> Self {
        Self {
            bundle_dir: config.bundle_root().join(Self::key()),
        }
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
