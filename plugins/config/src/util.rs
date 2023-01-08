use std::path::PathBuf;

pub struct ConfigUtil;

impl ConfigUtil {
    pub fn ensure_dir(p: &PathBuf) {
        if !&p.is_dir() {
            std::fs::create_dir_all(&p).unwrap();
        }
    }
}
