use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub owner_id: Text<Uuid>,
    pub type_x: Text<String>,
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, TypedBuilder)]
pub struct DataProduct {
    pub id: Uuid,
    pub owner_id: Option<Uuid>,
    pub status: String,
    pub type_format: String,
    pub category: String,
    #[builder(default, setter(strip_option))]
    pub partitions: Option<i16>,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

impl DataProduct {
    pub(crate) async fn create(product: DataProduct, ppg: &Pool<Postgres>, cnt: i16 ) -> Result<DataProduct, Error> {
        let query = sqlx::query!(
                r#"
                    INSERT INTO data_products (id, owner_id, status, type, category, partitions, created_at, update_at)
                    VALUES ( $1, $2, $3, $4, $5, $6, $7, $8)
                "#,
                product.id: UUID,
                product.owner_id: UUID,
                product.status,
                product.type_format,
                product.category,
                cnt,
                product.created_at: TIMESTAMP,
                product.update_at: TIMESTAMP)
            .execute(ppg)
            .await;

        match query {
            Ok(DataProduct) => Ok(product),
            Err(Error) => Err(Error)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, TypedBuilder)]
pub struct ShowForm {
    pub reader: Uuid,
    pub product_id: Uuid,
}
