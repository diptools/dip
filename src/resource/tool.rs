use crate::resource::platform::Platform;
use anyhow::{anyhow, Context};
use std::{
    fs::{self, Permissions},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};
use tokio::{fs::File, io::AsyncWriteExt};

pub type ToolResult<T> = std::result::Result<T, ToolError>;

#[derive(Debug, Clone)]
pub struct ToolError {
    pub error: std::sync::Arc<anyhow::Error>,
}

impl From<anyhow::Error> for ToolError {
    fn from(error: anyhow::Error) -> Self {
        ToolError {
            error: std::sync::Arc::new(error),
        }
    }
}

pub enum Tool {
    Tailwind,
}

impl Tool {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "tailwindcss" => Some(Self::Tailwind),
            _ => None,
        }
    }

    pub fn list() -> Vec<&'static str> {
        vec!["tailwindcss"]
    }

    fn app_path() -> PathBuf {
        let p = dirs::home_dir().unwrap().join(".dip");
        Self::ensure_dir(&p);
        p
    }

    fn install_path() -> PathBuf {
        let p = Self::app_path().join("installs");
        Self::ensure_dir(&p);
        p
    }

    fn bin_path(&self) -> PathBuf {
        let p = Self::install_path().join(self.name());
        Self::ensure_dir(&p);
        p.join(self.bin_name())
    }

    fn is_installed(&self) -> bool {
        self.bin_path().is_file()
    }

    pub async fn install(&self) -> ToolResult<()> {
        if self.is_installed() {
            Err(anyhow!("{} is already installed", self.name()))?
        } else {
            let mut file = File::create(self.bin_path())
                .await
                .context("Failed to create download target file")?;

            file.set_permissions(Permissions::from_mode(0o777))
                .await
                .context("Failed to give permission to download target file")?;

            let mut res = reqwest::get(self.download_url())
                .await
                .with_context(|| format!("Failed to download tool: {}", self.name()))?;

            while let Some(chunk) = res
                .chunk()
                .await
                .context("Failed to stream chunks of downloading content")?
            {
                file.write(chunk.as_ref())
                    .await
                    .context("Failed to write chunks of downloading content")?;
            }

            Ok(())
        }
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

    fn name(&self) -> &'static str {
        match self {
            Tool::Tailwind => "tailwindcss",
        }
    }

    fn version(&self) -> &'static str {
        match self {
            Tool::Tailwind => "v3.1.8",
        }
    }

    fn bin_name(&self) -> String {
        let windows_extension = match self.target_platform() {
            Platform::Windows => ".exe",
            _ => "",
        };

        match self {
            Tool::Tailwind => format!(
                "tailwindcss-{target}-{arch}{optional_ext}",
                target = self.target_platform().as_str(),
                arch = self.target_arch(),
                optional_ext = windows_extension
            ),
        }
    }

    fn download_url(&self) -> String {
        match self {
            Tool::Tailwind => format!(
                "https://github.com/tailwindlabs/tailwindcss/releases/download/{version}/{bin_name}",
                version = self.version(),
                bin_name = self.bin_name(),
            ),
        }
    }

    fn ensure_dir(p: &PathBuf) {
        if !&p.is_dir() {
            fs::create_dir_all(&p).unwrap();
        }
    }
}
