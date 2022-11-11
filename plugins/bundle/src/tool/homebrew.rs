use crate::{tool::InstallTools, ApplyBundle, BundleStage};
use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
};
use cmd_lib::spawn_with_output;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
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
        let brew = Homebrew::from(e.clone());

        match &brew.brewfile_path() {
            Ok(_brewfile_path) => {
                if brew.installed() {
                    println!("🟡 Skip: Install Homebrew");
                    println!("brew is already installed");
                } else {
                    println!("📌 Install Homebrew bundle");

                    if let Err(e) = brew.install() {
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
    });
}

fn apply(mut events: EventReader<ApplyBundle>) {
    events.iter().for_each(|e| {
        let brew = Homebrew::from(e.clone());

        match &brew.brewfile_path() {
            Ok(brewfile_path) => {
                if brew.installed() {
                    println!("📌 Apply Homebrew bundle");

                    if let Err(e) = brew.apply(&brewfile_path) {
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
    });
}

struct Homebrew {
    pub path: PathBuf,
}

impl Homebrew {
    fn homebrew_path() -> &'static str {
        "/opt/homebrew/bin/brew"
    }

    fn bundle_path(&self) -> PathBuf {
        self.path.join("bundle/homebrew")
    }

    fn brewfile_path(&self) -> io::Result<PathBuf> {
        self.bundle_path().join("Brewfile").canonicalize()
    }

    fn installed(&self) -> bool {
        Path::new(Self::homebrew_path()).exists()
    }

    fn install(&self) -> anyhow::Result<()> {
        let install_sh = reqwest::blocking::get(
            "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh",
        )?
        .text()?;

        let dir = tempdir()?;
        let file_path = dir.path().join("brew-install.sh");
        let file_path_str = file_path.display().to_string();
        let mut file = File::create(file_path)?;
        file.write_all(install_sh.as_bytes())
            .expect("Unable to write file");

        let mut install_brew = spawn_with_output!(NONINTERACTIVE=1 /bin/bash -C $file_path_str)?;

        let result = install_brew.wait_with_pipe(&mut |pipe| {
            BufReader::new(pipe)
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|f| println!("{f}"));
        })?;

        Ok(result)
    }

    fn apply(&self, brewfile_path: &PathBuf) -> anyhow::Result<()> {
        let brewfile_path_str = &brewfile_path.display();

        let mut brew_bundle = spawn_with_output!(brew bundle --cleanup --file $brewfile_path_str)?;

        let result = brew_bundle.wait_with_pipe(&mut |pipe| {
            BufReader::new(pipe)
                .lines()
                .filter_map(|line| line.ok())
                .for_each(|line| println!("{:?}", line));
        })?;

        Ok(result)
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
