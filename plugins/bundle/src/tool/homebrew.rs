use crate::{ApplyBundle, BundleStage};
use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
    log,
};
use cmd_lib::{run_fun, spawn_with_output};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
};
use tempfile::tempdir;

use super::InstallTools;

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
    events.iter().for_each(
        |e| match &e.path.join("bundle/homebrew/Brewfile").canonicalize() {
            Ok(_brewfile_path) => {
                if run_fun!(which brew).is_ok() {
                    log::info!("🟡 Skip: Install Homebrew");
                    log::info!("brew path already exists");
                } else {
                    log::info!("📌 Install Homebrew");

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

                    let mut install_brew = spawn_with_output!(/bin/bash -C $file_path_str).unwrap();

                    let result = if e.verbose {
                        install_brew.wait_with_pipe(&mut |pipe| {
                            BufReader::new(pipe)
                                .lines()
                                .filter_map(|line| line.ok())
                                .for_each(|f| log::info!("{f}"));
                        })
                    } else {
                        if let Err(e) = install_brew.wait_with_output() {
                            Err(e)
                        } else {
                            Ok(())
                        }
                    };

                    if let Err(e) = result {
                        log::error!("Failed to run brew install.");
                        log::error!("{e}");
                    } else {
                        log::info!("✅ Install Homebrew");
                    }
                }
            }
            Err(_e) => {
                log::info!("🟡 Skip: Install Homebrew");
                log::info!("bundle/homebrew/Brewfile does not exists.",);
            }
        },
    );
}
fn apply(mut events: EventReader<ApplyBundle>) {
    events.iter().for_each(|e| {
        let current_path = env::current_dir().expect("Failed to get current directory.");
        let brewfile_path = current_path.join(&e.path).join("bundle/homebrew/Brewfile");

        match brewfile_path.canonicalize() {
            Ok(_brewfile_path) => match run_fun!(which brew) {
                Ok(_brew_path) => {
                    log::info!("📌 Apply Homebrew bundle");
                    let brewfile_path_str = &brewfile_path
                        .clone()
                        .into_os_string()
                        .into_string()
                        .unwrap();

                    let mut brew_bundle =
                        spawn_with_output!(brew bundle --file $brewfile_path_str).unwrap();

                    let result = if e.verbose {
                        brew_bundle.wait_with_pipe(&mut |pipe| {
                            BufReader::new(pipe)
                                .lines()
                                .filter_map(|line| line.ok())
                                .for_each(|line| log::info!("{:?}", line));
                        })
                    } else {
                        if let Err(e) = brew_bundle.wait_with_output() {
                            Err(e)
                        } else {
                            Ok(())
                        }
                    };

                    if let Err(e) = result {
                        log::error!("{e}");
                        log::error!("Failed to run brew bundle.");
                    } else {
                        log::info!("✅ Apply Homebrew bundle");
                    }
                }
                Err(e) => {
                    log::error!("{e}");
                    log::error!("Could not find homebrew binary.");
                }
            },
            Err(_e) => {
                log::info!("🟡 Skip: Apply Homebrew bundle");
                log::info!("bundle/homebrew/Brewfile does not exists.");
            }
        }
    });
}
