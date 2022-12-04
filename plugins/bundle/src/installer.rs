use anyhow::{bail, Context};
use bytes::Bytes;
use flate2::read::GzDecoder;
use reqwest::StatusCode;
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf};
use tar::Archive;

pub trait Installer {
    /// 1. Download package: e.g. https://nodejs.org/dist/v16.18.1/node-v16.18.1-darwin-arm64.tar.gz
    /// 2. Verify checkum
    /// 3. Unpack to path: e.g. /User/Application Support/dip/bundle/installs/nodejs/16.18.1/
    fn install(
        &self,
        download_url: &String,
        checksum: &String,
        install_path: &PathBuf,
        file_name_without_ext: &String,
    ) -> anyhow::Result<()> {
        let res = reqwest::blocking::get(download_url)
            .context("Failed to download. Check internet connection.")?;

        match res.status() {
            StatusCode::OK => {
                if res.status() == StatusCode::NOT_FOUND {
                    bail!("Download URL not found: {download_url}");
                }
                let bytes = res.bytes()?;

                self.verify_checksum(&bytes, checksum)?;
                let mut cloned_path = install_path.clone();
                cloned_path.pop();

                if cfg!(unix) {
                    let tar = GzDecoder::new(&bytes[..]);
                    let mut archive = Archive::new(tar);

                    archive.unpack(&cloned_path)?;

                    fs::rename(
                        // e.g. /User/Application Support/dip/bundle/installs/nodejs/node-v16.18.1-darwin-arm64
                        cloned_path.join(&file_name_without_ext),
                        // e.g. /User/Application Support/dip/bundle/installs/nodejs/16.18.1/
                        &install_path,
                    )?;
                } else if cfg!(windows) {
                    // win: zip
                    todo!("Implement zip extraction logic for Windows");
                }

                Ok(())
            }
            StatusCode::NOT_FOUND => {
                bail!("Download URL not found: {download_url}");
            }
            _ => {
                bail!("Fail to download binary")
            }
        }
    }

    fn file_name(&self, version: &String) -> String;

    fn file_name_without_ext(&self, version: &String) -> String;

    fn verify_checksum(&self, bytes: &Bytes, checksum: &String) -> anyhow::Result<()> {
        let hash = Sha256::digest(&bytes);
        if hash[..] == hex::decode(checksum)? {
            Ok(())
        } else {
            bail!("Checksum doesn't match")
        }
    }
}
