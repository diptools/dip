use bevy::app::{App, Plugin};
use std::{fs, path::PathBuf};

pub struct BundleConfigPlugin;

impl Plugin for BundleConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BundleConfig>();
    }
}

pub struct BundleConfig {
    app_path: PathBuf,
}

impl Default for BundleConfig {
    fn default() -> Self {
        Self {
            app_path: dirs::home_dir().unwrap().join(".dip"),
        }
    }
}

impl BundleConfig {
    pub fn app_path(&self) -> PathBuf {
        Self::ensure_dir(&self.app_path);

        self.app_path.clone()
    }

    pub fn install_path(&self) -> PathBuf {
        let p = self.app_path().join("installs");
        Self::ensure_dir(&p);

        p
    }

    fn ensure_dir(p: &PathBuf) {
        if !&p.is_dir() {
            fs::create_dir_all(&p).unwrap();
        }
    }
}
