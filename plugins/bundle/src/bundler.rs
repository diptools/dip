use crate::config::BundleConfig;
use std::path::PathBuf;

pub trait Bundler {
    fn key() -> &'static str;

    fn name() -> &'static str;

    fn bundle_config(&self) -> &BundleConfig;

    fn bundle_dir(&self) -> PathBuf {
        self.bundle_config().root_dir().join(Self::key())
    }

    fn bundle_exists(&self) -> bool {
        self.bundle_dir().is_dir()
    }
}
