use thiserror::Error; // Добавьте thiserror в Cargo.toml

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Storage error: {0}")]
    StorageError(#[from] minio::s3::error::Error),

    #[error("Product not found")]
    NotFound,

    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] reqwest::Error),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
