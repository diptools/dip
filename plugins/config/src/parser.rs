use anyhow::{Context, Result};
use bevy::log;
use reqwest::Url;
use serde::{de, Deserialize, Deserializer};
use std::path::PathBuf;

pub struct ConfigParser;

impl ConfigParser {
    pub fn url_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Url>, D::Error> {
        let s = Deserialize::deserialize(d);
        match s {
            Ok(s) => match Url::parse(s) {
                Ok(url) => Ok(Some(url)),
                Err(e) => {
                    log::warn!("{e}");
                    Ok(None)
                }
            },
            Err(e) => {
                log::warn!("{e}");
                Ok(None)
            }
        }
    }

    pub fn path_from_str<'de, D: Deserializer<'de>>(d: D) -> Result<PathBuf, D::Error> {
        let s: String = Deserialize::deserialize(d)?;

        match Self::to_path(&s) {
            Ok(path) => {
                if path.is_dir() {
                    Ok(path)
                } else {
                    Err(de::Error::custom(&format!(
                        "Make sure to create bundle directory: {}",
                        path.display()
                    )))
                }
            }
            Err(_e) => Err(de::Error::custom("Failed to parse bundle directory path")),
        }
    }

    pub fn to_path(value: &String) -> Result<PathBuf> {
        let p = value
            .replace(
                "$HOME",
                dirs::home_dir()
                    .context("Cannot find home directory.")?
                    .to_str()
                    .context("Failed to convert path to string.")?,
            )
            .replace(
                "$CONFIG_DIR",
                dirs::config_dir()
                    .context("Cannot find config directory.")?
                    .to_str()
                    .context("Failed to convert path to string.")?,
            )
            .replace(
                "$DATA_DIR",
                dirs::data_dir()
                    .context("Cannot find data directory.")?
                    .to_str()
                    .context("Failed to convert path to string.")?,
            )
            .into();

        Ok(p)
    }
}
