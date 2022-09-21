use dip::{
    bevy::{
        app::{App, Plugin},
        ecs::event::EventReader,
        log,
    },
    cli::SubcommandPlugin,
};
use std::{fs, path::PathBuf};

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ToolActionPlugin)
            .add_system(handle_list_tool)
            .add_system(handle_add_tool);
    }
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum ToolAction {
    List,
    Add(AddAction),
}

#[derive(clap::Args, Clone, Debug)]
pub struct AddAction {
    name: String,
}

enum Tool {
    Tailwind,
}

impl Tool {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "tailwindcss" => Some(Self::Tailwind),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Tool::Tailwind => "tailwindcss",
        }
    }

    fn list() -> Vec<&'static str> {
        vec!["tailwindcss"]
    }

    fn path() -> PathBuf {
        let p = dirs::home_dir().unwrap().join(".dip");
        Self::ensure_dir(&p);
        p
    }

    // fn install_path() -> PathBuf {
    //     let p = Self::path().join("installs");
    //     Self::ensure_dir(&p);
    //     p
    // }

    fn download_path(&self) -> PathBuf {
        let p = Self::path().join("downloads").join(self.as_str());
        Self::ensure_dir(&p);
        p
    }

    fn target_platform(&self) -> Platform {
        match self {
            _ => match std::env::consts::OS {
                "linux" => Platform::Linux,
                "macos" => Platform::Macos,
                "windows" => Platform::Windows,
                _ => panic!("unsupported platformm"),
            },
        }
    }

    fn target_arch(&self) -> &'static str {
        match std::env::consts::ARCH {
            "x86" | "x86_64" => "x64",
            "arm" | "aarch64" => "arm64",
            _ => panic!("unsupported arch"),
        }
    }

    fn download_url(&self) -> String {
        let windows_extension = match self.target_platform() {
            Platform::Windows => ".exe",
            _ => "",
        };

        format!(
            "https://github.com/tailwindlabs/tailwindcss/releases/download/{version}/tailwindcss-{target}-{arch}{optional_ext}",
            version = "v3.1.8", // self.tool_version(),
            target = self.target_platform().as_str(),
            arch = self.target_arch(),
            optional_ext = windows_extension
        )
    }

    fn ensure_dir(p: &PathBuf) {
        if !&p.is_dir() {
            fs::create_dir_all(&p).unwrap();
        }
    }
}

enum Platform {
    Linux,
    Macos,
    Windows,
}

impl Platform {
    fn as_str(&self) -> &'static str {
        match self {
            Platform::Linux => "linux",
            Platform::Macos => "macos",
            Platform::Windows => "windows",
        }
    }
}

fn handle_list_tool(mut events: EventReader<ListToolAction>) {
    for _ in events.iter() {
        for t in Tool::list().iter() {
            println!("- {t}");
        }
    }
}

fn handle_add_tool(mut events: EventReader<AddToolAction>) {
    for e in events.iter() {
        let name = e.name.as_str();

        match Tool::from_str(name) {
            Some(tool) => match tool {
                Tool::Tailwind => {
                    let download_path = tool.download_path();
                    let is_empty = download_path.read_dir().unwrap().next().is_none();

                    if is_empty {
                        println!("{}", tool.download_url());
                    } else {
                    }
                }
            },
            None => {
                log::error!("Could not find tool: {name}");
            }
        }
    }
}
