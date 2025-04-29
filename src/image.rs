use std::{
    error::Error,
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use isahc::{ReadResponseExt, get};
use serde_json::{Value, from_str};
use tap::Tap;

use crate::{RESOURCE_NAME, cli};

const JSON_URL: &str = "http://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US";
const BING_URL: &str = "https://bing.com";
const DAY: u64 = 86_400;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub fn get_image() -> Result<Vec<u8>> {
    let cli = cli();
    let default_image_path = get_default_image_path();
    let image_path: &PathBuf = cli.get_one("cache_path").unwrap_or(&default_image_path);
    Ok(if is_already_downloaded(image_path)? {
        fs::read(image_path)?
    } else {
        let image = download()?;
        fs::write(image_path, &image)?;
        image
    })
}

fn is_already_downloaded(image_path: &PathBuf) -> Result<bool> {
    let image_metadata = match fs::metadata(image_path) {
        Ok(metadata) => metadata,
        _ => return Ok(false),
    };
    Ok(SystemTime::now().duration_since(image_metadata.modified()?)? <= Duration::from_secs(DAY))
}

fn download() -> Result<Vec<u8>> {
    Ok(get(get_url()?)?.bytes()?)
}

fn get_url() -> Result<String> {
    let json = get(JSON_URL)?.text()?;
    let parsed_json: Value = from_str(&json)?;
    let url_from_json = parsed_json["images"][0]["url"].as_str().unwrap();
    Ok(format!("{BING_URL}{url_from_json}"))
}

fn get_default_image_path() -> PathBuf {
    dirs::cache_dir()
        .unwrap()
        .tap_mut(|p| p.push(format!("{RESOURCE_NAME}.jpeg")))
}
