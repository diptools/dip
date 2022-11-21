use std::fmt;

pub enum Platform {
    Linux,
    Macos,
    Windows,
}

impl Platform {
    pub fn new() -> Platform {
        match std::env::consts::OS {
            "linux" => Platform::Linux,
            "macos" => Platform::Macos,
            "windows" => Platform::Windows,
            _ => panic!("Unsupported platform"),
        }
    }

    pub fn arch() -> &'static str {
        match std::env::consts::ARCH {
            "x86" | "x86_64" => "x64",
            "arm" | "aarch64" => "arm64",
            _ => panic!("Unsupported arch"),
        }
    }

    pub fn ext(&self) -> &'static str {
        match self {
            Platform::Windows => ".exe",
            _ => "",
        }
    }

    pub fn compression_ext(&self) -> &'static str {
        match self {
            Platform::Windows => ".zip",
            _ => ".tar.xz",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Platform::Linux => "linux",
            Platform::Macos => "darwin",
            Platform::Windows => "win",
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Platform::Linux => write!(f, "linux"),
            Platform::Macos => write!(f, "macos"),
            Platform::Windows => write!(f, "windows"),
        }
    }
}
