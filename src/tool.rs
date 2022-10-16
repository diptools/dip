use anyhow::{anyhow, Context};
use dip::{
    bevy::{
        app::{App, AppExit, Plugin},
        ecs::{
            event::{EventReader, EventWriter},
            system::Res,
        },
        log,
    },
    cli::SubcommandPlugin,
    core::task::{async_action, AsyncActionPool},
};
use std::{fs, path::PathBuf};
use tokio::fs::File;

pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ToolActionPlugin)
            .add_plugin(AsyncActionPlugin)
            .add_system(handle_list_tool)
            .add_system(handle_add_tool)
            .add_system(handle_install);
    }
}

#[derive(SubcommandPlugin, clap::Subcommand, Clone, Debug)]
pub enum ToolAction {
    List,
    Add { name: String },
}

fn handle_list_tool(mut events: EventReader<ListToolAction>) {
    for _ in events.iter() {
        for t in Tool::list().iter() {
            println!("- {t}");
        }
    }
}

fn handle_add_tool(
    mut events: EventReader<AddToolAction>,
    async_action: Res<AsyncActionPool<AsyncAction>>,
) {
    for e in events.iter() {
        let name = e.name.as_str();
        let tool = Tool::from_str(name).expect(&format!("Could not find tool: {name}"));

        match tool {
            Tool::Tailwind => async_action.send(AsyncAction::install(tool)),
        }
    }
}

fn handle_install(mut events: EventReader<Result<Install>>, mut app_exit: EventWriter<AppExit>) {
    for _ in events.iter() {
        app_exit.send(AppExit);
    }
}

#[derive(Debug, Clone, Default)]
pub struct ToolError {
    error: Option<std::sync::Arc<anyhow::Error>>,
}

impl From<anyhow::Error> for ToolError {
    fn from(error: anyhow::Error) -> Self {
        ToolError {
            error: Some(std::sync::Arc::new(error)),
        }
    }
}

type Result<T> = std::result::Result<T, ToolError>;

#[async_action]
impl AsyncActionCreator {
    async fn install(tool: Tool) -> Result<Install> {
        if let Err(e) = tool.download().await {
            log::warn!("{e:?}");
        }

        let _a = tool.install()?;

        Ok(Install)
    }
}

#[derive(Clone, Debug)]
/* #[derive(Clone, Debug, Deserialize, Default)] */
pub struct Install;

pub enum Tool {
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

    fn app_path() -> PathBuf {
        let p = dirs::data_local_dir().unwrap().join(".dip");
        Self::ensure_dir(&p);
        p
    }

    fn tool_path() -> PathBuf {
        let p = Self::app_path().join("tools");
        Self::ensure_dir(&p);
        p
    }

    // fn install_path() -> PathBuf {
    //     let p = Self::path().join("installs");
    //     Self::ensure_dir(&p);
    //     p
    // }

    fn install(&self) -> Result<()> {
        Ok(())
    }

    fn is_installed(&self) -> bool {
        Self::tool_path().read_dir().unwrap().next().is_some()
    }

    async fn download(&self) -> Result<()> {
        if self.is_downloaded() {
            Err(anyhow!("Tool: {}, is already downloaded", self.as_str()))?
        } else {
            /* let mut file = File::create(self.download_path()) */
            /*     .await */
            /*     .context("failed creating temporary output file")?; */

            let res = reqwest::get(self.download_url())
                .await
                .with_context(|| format!("Failed to download tool: {}", self.as_str()))?
                .bytes()
                .await
                .with_context(|| "Failed to parse file")?;

            println!("{res:#?}");

            Ok(())
        }
    }

    fn is_downloaded(&self) -> bool {
        println!("{:?}", self.download_path());
        false
    }

    fn download_path(&self) -> PathBuf {
        let p = Self::app_path().join("downloads").join(self.as_str());
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
