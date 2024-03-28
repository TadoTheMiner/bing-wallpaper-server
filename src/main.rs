use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use isahc::{get, ReadResponseExt};
use serde_json::{from_str, Value};
use std::{
    error::Error,
    fs::{metadata, read, write},
    path::PathBuf,
    time::{SystemTime, Duration},
};

const JSON_URL: &str = "http://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=en-US";
const BING_URL: &str = "https://bing.com";
const RESOURCE_NAME: &str = "bing-daily-wallpaper";
const DAY: u64 = 86_400;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route(&format!("/{RESOURCE_NAME}"), web::get().to(get_image)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn get_image() -> Result<impl Responder> {
    let image = if image_is_already_downloaded()? {
        read(get_image_path())?
    } else {
        download_image()?
    };

    Ok(HttpResponse::Ok().body(image))
}

fn image_is_already_downloaded() -> Result<bool> {
    let image_metadata = match metadata(get_image_path()) {
        Ok(metadata) => metadata,
        _ => return Ok(false),
    };
    Ok(SystemTime::now().duration_since(image_metadata.modified()?)? <= Duration::from_secs(DAY))
}

fn download_image() -> Result<Vec<u8>> {
    let image = get(&get_image_url()?)?.bytes()?;
    write(get_image_path(), &image)?;
    Ok(image)
}

fn get_image_url() -> Result<String> {
    let json = get(JSON_URL)?.text()?;
    let parsed_json: Value = from_str(&json)?;
    let url_from_json = parsed_json["images"][0]["url"].as_str().unwrap();
    Ok(format!("{BING_URL}{url_from_json}"))
}

fn get_image_path() -> PathBuf {
    let mut path = dirs::cache_dir().unwrap();
    path.push(format!("{RESOURCE_NAME}.jpeg"));
    path
}
