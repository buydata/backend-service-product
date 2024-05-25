mod api;
mod db;
mod model;
mod s3;
mod service;

use crate::api::data_product_controller::{create, show};
use actix_web::{web, App, HttpServer};
use db::establish_connection;
use minio::s3::client::Client;
use s3::establish_connection_s3;
use sqlx::{Pool, Postgres};
use std::env;

pub struct AppState {
    db: Pool<Postgres>,
    s3: Client,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    init()?;

    log::info!("creating temporary upload directory");
    std::fs::create_dir_all("./tmp")?;

    let url_host = env::var("SERVICE_HOST").expect("SERVICE_HOST must be set");
    let url_port = env::var("SERVICE_PORT")
        .expect("SERVICE_PORT must be set")
        .parse::<u16>()
        .unwrap();
    let url: String = format!("{}:{}", &url_host, &url_port);
    let pool = establish_connection().await;
    let s3_client = establish_connection_s3().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                s3: s3_client.clone(),
            }))
            .service(create)
            .service(show)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(url)?
    .run()
    .await?;

    Ok(())
}

fn init() -> Result<(), fern::InitError> {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".into());
    let log_level = log_level.parse().unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr());

    if let Ok(log_file) = env::var("LOG_FILE") {
        let log_file = std::fs::File::create(log_file)?;
        builder = builder.chain(log_file);
    }

    builder.apply()?;

    log::trace!("TRACE output enabled");
    log::debug!("DEBUG output enabled");
    log::info!("INFO output enabled");
    log::warn!("WARN output enabled");
    log::error!("ERROR output enabled");

    Ok(())
}
