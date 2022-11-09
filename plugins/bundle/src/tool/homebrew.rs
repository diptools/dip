use crate::{tool::InstallTools, ApplyBundle, Bundle, BundleStage};
use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
};
use cmd_lib::spawn_with_output;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};
use tempfile::tempdir;

// Plugin

pub struct HomebrewPlugin;

impl Plugin for HomebrewPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(BundleStage::Install, install)
            .add_system_to_stage(BundleStage::Apply, apply);
    }
}

// Systems

fn install(mut events: EventReader<InstallTools>) {
    events.iter().for_each(|e| {
        Homebrew::from(e.clone()).install();
    });
}
fn apply(mut events: EventReader<ApplyBundle>) {
    events.iter().for_each(|e| {
        Homebrew::from(e.clone()).apply();
    });
}

struct Homebrew {
    pub path: PathBuf,
}

impl Bundle for Homebrew {
    fn bundle_path(&self) -> PathBuf {
        self.path.join("bundle/homebrew")
    }
}

impl Homebrew {
    fn homebrew_path() -> &'static str {
        "/opt/homebrew/bin/brew"
    }

    fn brewfile_path(&self) -> PathBuf {
        self.bundle_path().join("bundle/homebrew/Brewfile")
    }

    fn install(&self) {
        match &self.brewfile_path().canonicalize() {
            Ok(_brewfile_path) => {
                if Path::new(Self::homebrew_path()).exists() {
                    println!("🟡 Skip: Install Homebrew");
                    println!("brew path already exists");
                } else {
                    println!("📌 Install Homebrew");

                    let install_sh = reqwest::blocking::get(
                        "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh",
                    )
                    .expect("Failed to fetch Homebrew installation script")
                    .text()
                    .expect("Failed to parse Homebrew installation script into text");

                    let dir = tempdir().unwrap();
                    let file_path = dir.path().join("brew-install.sh");
                    let file_path_str = file_path.clone().into_os_string();
                    let mut file = File::create(file_path).unwrap();
                    file.write_all(install_sh.as_bytes())
                        .expect("Unable to write file");

                    let mut install_brew =
                        spawn_with_output!(NONINTERACTIVE=1 /bin/bash -C $file_path_str).unwrap();

                    let result = install_brew.wait_with_pipe(&mut |pipe| {
                        BufReader::new(pipe)
                            .lines()
                            .filter_map(|line| line.ok())
                            .for_each(|f| println!("{f}"));
                    });

                    if let Err(e) = result {
                        println!("Failed to run brew install.");
                        eprintln!("{e}");
                    } else {
                        println!("✅ Install Homebrew");
                    }
                }
            }
            Err(_e) => {
                println!("🟡 Skip: Install Homebrew");
                println!("bundle/homebrew/Brewfile does not exists.",);
            }
        }
    }

    fn apply(&self) {
        match &self.brewfile_path().canonicalize() {
            Ok(brewfile_path) => {
                if Path::new(Self::homebrew_path()).exists() {
                    println!("📌 Apply Homebrew bundle");
                    let brewfile_path_str = &brewfile_path
                        .clone()
                        .into_os_string()
                        .into_string()
                        .unwrap();

                    let mut brew_bundle =
                        spawn_with_output!(brew bundle --cleanup --file $brewfile_path_str)
                            .unwrap();

                    let result = brew_bundle.wait_with_pipe(&mut |pipe| {
                        BufReader::new(pipe)
                            .lines()
                            .filter_map(|line| line.ok())
                            .for_each(|line| println!("{:?}", line));
                    });

                    if let Err(e) = result {
                        println!("Failed to run brew bundle.");
                        eprintln!("{e}");
                    } else {
                        println!("✅ Apply Homebrew bundle");
                    }
                } else {
                    eprintln!("Could not find homebrew binary.");
                }
            }
            Err(_e) => {
                println!("🟡 Skip: Apply Homebrew bundle");
                println!("bundle/homebrew/Brewfile does not exists.");
            }
        }
    }
}

impl From<InstallTools> for Homebrew {
    fn from(InstallTools { path }: InstallTools) -> Self {
        Self { path }
    }
}

impl From<ApplyBundle> for Homebrew {
    fn from(ApplyBundle { path }: ApplyBundle) -> Self {
        Self { path }
    }
}
