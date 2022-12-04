use anyhow::{bail, Context};
use bytes::Bytes;
use flate2::read::GzDecoder;
use reqwest::{header, StatusCode};
use sha2::{Digest, Sha256};
use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};
use tar::Archive;

pub trait Installer {
    /// 1. Download package: e.g. https://nodejs.org/dist/v16.18.1/node-v16.18.1-darwin-arm64.tar.gz
    /// 2. Verify checkum
    /// 3. Unpack to path: e.g. /User/Application Support/dip/bundle/installs/nodejs/16.18.1/
    fn install(
        &self,
        download_url: &String,
        install_path: &PathBuf,
        file_name: &String,
        checksum: Option<&String>,
    ) -> anyhow::Result<()> {
        let res = reqwest::blocking::get(download_url)
            .context("Failed to download. Check internet connection.")?;

        match res.status() {
            StatusCode::OK => {
                match res.headers()[header::CONTENT_TYPE].to_str()? {
                    "application/gzip" => {
                        let bytes = res.bytes()?;

                        if let Some(checksum) = checksum {
                            self.verify_checksum(&bytes, checksum)?;
                        }

                        let mut cloned_path = install_path.clone();
                        cloned_path.pop();

                        let tar = GzDecoder::new(&bytes[..]);
                        let mut archive = Archive::new(tar);

                        archive.unpack(&cloned_path)?;

                        fs::rename(
                            // e.g. /User/Application Support/dip/bundle/installs/nodejs/node-v16.18.1-darwin-arm64
                            cloned_path.join(&file_name),
                            // e.g. /User/Application Support/dip/bundle/installs/nodejs/16.18.1/
                            &install_path,
                        )?;
                        Ok(())
                    }
                    "application/octet-stream" => {
                        let file_path = &install_path.join(file_name);

                        fs::create_dir_all(&install_path)?;
                        fs::write(&file_path, &res.bytes()?)?;
                        fs::set_permissions(&file_path, fs::Permissions::from_mode(0o755))?;

                        Ok(())
                    }
                    unsupported_content_type => {
                        bail!(
                            "Content-Type is not supported: {}",
                            unsupported_content_type
                        );
                    }
                }
            }
            StatusCode::NOT_FOUND => {
                bail!("Download URL not found: {download_url}");
            }
            _ => {
                bail!("Fail to download binary")
            }
        }
    }

    fn verify_checksum(&self, bytes: &Bytes, checksum: &String) -> anyhow::Result<()> {
        let hash = Sha256::digest(&bytes);
        if hash[..] == hex::decode(checksum)? {
            Ok(())
        } else {
            bail!("Checksum doesn't match")
        }
    }
}
