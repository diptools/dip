use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Config {
    bundle: Bundle,
}

#[derive(Deserialize)]
pub struct Bundle {
    vm: VersionManager,
}

#[derive(Deserialize)]
pub struct VersionManager {
    runtime: Runtime,
}

#[derive(Deserialize)]
pub struct Runtime {
    tailwindcss: VersionList,
    nodejs: VersionList,
}

pub type VersionList = Vec<String>;
