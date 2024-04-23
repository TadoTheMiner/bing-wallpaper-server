mod image;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use clap::{crate_description, crate_name, value_parser, Arg, Command};
use image::Image;
use std::{error::Error, path::PathBuf};

const LOCALHOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";
pub const RESOURCE_NAME: &str = "bing-daily-wallpaper";

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route(&format!("/{RESOURCE_NAME}"), web::get().to(get_image)))
        .bind((LOCALHOST, cli().get_one::<u16>("port").unwrap().to_owned()))?
        .run()
        .await
}

pub fn cli() -> clap::ArgMatches {
    Command::new(crate_name!())
        .about(crate_description!())
        .arg(
            Arg::new("port")
                .default_value(DEFAULT_PORT)
                .value_parser(value_parser!(u16)),
        )
        .arg(Arg::new("cache_path").value_parser(value_parser!(PathBuf)))
        .get_matches()
}

async fn get_image() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().body(Image::get()?.into_vec()))
}
