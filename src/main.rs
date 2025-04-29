mod image;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use clap::{Arg, Command, crate_description, crate_name, value_parser};
use std::path::PathBuf;

const LOCALHOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";
const RESOURCE_NAME: &str = "bing-daily-wallpaper";

#[actix_web::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    HttpServer::new(|| App::new().route(&format!("/{RESOURCE_NAME}"), web::get().to(get_image)))
        .bind((LOCALHOST, cli().get_one::<u16>("port").unwrap().to_owned()))?
        .run()
        .await?;
    Ok(())
}

fn cli() -> clap::ArgMatches {
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

async fn get_image() -> actix_web::Result<impl Responder> {
    Ok(HttpResponse::Ok().body(image::get_image()?))
}
