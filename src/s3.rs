use std::env;

use dotenvy::dotenv;

use minio::s3::client::{Client, ClientBuilder};
use minio::s3::creds::StaticProvider;
use minio::s3::error::Error;
use minio::s3::http::BaseUrl;

pub async fn create_client_connection(
    s3_url: &BaseUrl,
    static_provider: StaticProvider,
) -> Result<Client, Error> {
    let client = ClientBuilder::new(s3_url.clone())
        .provider(Some(Box::new(static_provider)))
        .build()?;

    Ok(client)
}

pub async fn establish_connection_s3() -> Client {
    dotenv().ok();

    let s3_url = env::var("S3_URL")
        .expect("S3_ACCESS_KEY must be set")
        .parse::<BaseUrl>()
        .unwrap();

    let s3_access_key = env::var("S3_ACCESS_KEY").expect("S3_ACCESS_KEY must be set");
    let s3_secret_key = env::var("S3_SECRET_KEY").expect("S3_SECRET_KEY must be set");

    let static_provider = StaticProvider::new(&s3_access_key, &s3_secret_key, None);

    create_client_connection(&s3_url, static_provider)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to "))
}
