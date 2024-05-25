use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Error, PgPool};
use std::{env, time::Duration};

pub async fn init_pool(database_url: &str) -> Result<PgPool, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(1000)
        .idle_timeout(Duration::new(5, 0))
        .connect(&database_url)
        .await
        .unwrap();

    Ok(pool)
}

pub async fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    init_pool(&database_url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
