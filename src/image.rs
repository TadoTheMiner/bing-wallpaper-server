use std::{
    fs::{metadata, read, write},
    path::PathBuf,
    time::{Duration, SystemTime},
};

use isahc::{get, ReadResponseExt};
use serde_json::{from_str, Value};

use crate::{cli, Result, RESOURCE_NAME};

const JSON_URL: &str = "http://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US";
const BING_URL: &str = "https://bing.com";
const DAY: u64 = 86_400;

pub struct Image(Vec<u8>);
impl Image {
    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }

    pub fn get() -> Result<Self> {
        let cli = cli();
        let default_image_path = Self::get_default_image_path();
        let image_path: &PathBuf = cli.get_one("cache_path").unwrap_or(&default_image_path);
        Ok(if Self::is_already_downloaded(image_path)? {
            read(image_path)?.into()
        } else {
            Self::download(image_path)?
        })
    }

    fn is_already_downloaded(image_path: &PathBuf) -> Result<bool> {
        let image_metadata = match metadata(image_path) {
            Ok(metadata) => metadata,
            _ => return Ok(false),
        };
        Ok(
            SystemTime::now().duration_since(image_metadata.modified()?)?
                <= Duration::from_secs(DAY),
        )
    }

    fn download(image_path: &PathBuf) -> Result<Self> {
        let image = get(&Self::get_url()?)?.bytes()?;
        write(image_path, &image)?;
        Ok(image.into())
    }

    fn get_url() -> Result<String> {
        let json = get(JSON_URL)?.text()?;
        let parsed_json: Value = from_str(&json)?;
        let url_from_json = parsed_json["images"][0]["url"].as_str().unwrap();
        Ok(format!("{BING_URL}{url_from_json}"))
    }

    fn get_default_image_path() -> PathBuf {
        let mut path = dirs::cache_dir().unwrap();
        path.push(format!("{RESOURCE_NAME}.jpeg"));
        path
    }
}

impl From<Vec<u8>> for Image {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}
