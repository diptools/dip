use crate::{tool::InstallTools, ApplyBundle, BundleConfig, BundleStage, Bundler};
use bevy::{
    app::{App, Plugin},
    ecs::{event::EventReader, system::Res},
};
use cmd_lib::spawn_with_output;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
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

fn install(mut events: EventReader<InstallTools>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let brew = Homebrew::from(config.clone());

        if brew.brewfile_exists() {
            let homebrew_path = Homebrew::homebrew_path();
            if homebrew_path.is_file() {
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
        } else {
            println!("🟡 Skip: Install Homebrew");
            println!("bundle/homebrew/Brewfile does not exists.",);
        }
    });
}

fn apply(mut events: EventReader<ApplyBundle>, config: Res<BundleConfig>) {
    events.iter().for_each(|_e| {
        let brew = Homebrew::from(config.clone());

        if brew.brewfile_exists() {
            let homebrew_path = Homebrew::homebrew_path();
            if homebrew_path.is_file() {
                println!("📌 Apply Homebrew bundle");

                if let Err(e) = brew.apply(&brew.brewfile_path()) {
                    println!("Failed to run brew bundle.");
                    eprintln!("{e}");
                } else {
                    println!("✅ Apply Homebrew bundle");
                }
            } else {
                eprintln!("Could not find homebrew binary.");
            }
        } else {
            println!("🟡 Skip: Apply Homebrew bundle");
            println!("bundle/homebrew/Brewfile does not exists.");
        }
    });
}

struct Homebrew {
<<<<<<< HEAD
<<<<<<< HEAD
    pub path: PathBuf,
=======
    pub repo: PathBuf,
>>>>>>> e04d1b0 (Merge bundle config with cli arguments)
=======
    pub bundle: PathBuf,
}

impl Bundler for Homebrew {
    fn name() -> &'static str {
        "homebrew"
    }

    fn bundle(&self) -> &PathBuf {
        &self.bundle
    }
>>>>>>> 51d7a93 (Parse path and url from config file)
}

impl Homebrew {
    fn homebrew_path() -> PathBuf {
        PathBuf::from("/opt/homebrew/bin/brew")
    }

<<<<<<< HEAD
<<<<<<< HEAD
    fn bundle_path(&self) -> PathBuf {
        self.path.join("bundle/homebrew")
=======
    fn bundle_dir(&self) -> PathBuf {
        self.repo.join("bundle/homebrew")
>>>>>>> e04d1b0 (Merge bundle config with cli arguments)
=======
    fn brewfile_path(&self) -> PathBuf {
        self.bundle().join("Brewfile")
>>>>>>> 51d7a93 (Parse path and url from config file)
    }

    fn brewfile_exists(&self) -> bool {
        self.brewfile_path().is_file()
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

<<<<<<< HEAD
impl From<InstallTools> for Homebrew {
    fn from(InstallTools { path }: InstallTools) -> Self {
        Self { path }
    }
}

impl From<ApplyBundle> for Homebrew {
    fn from(ApplyBundle { path }: ApplyBundle) -> Self {
        Self { path }
=======
impl From<BundleConfig> for Homebrew {
    fn from(config: BundleConfig) -> Self {
        Self {
            bundle: config.bundle_root().join("homebrew"),
        }
>>>>>>> e04d1b0 (Merge bundle config with cli arguments)
    }
}
