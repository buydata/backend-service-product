use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Error, Pool, Postgres};
use typed_builder::TypedBuilder;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, TypedBuilder, ToSchema)]
pub struct DataProduct {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub status: String,
    pub format: String,
    pub name: String,
    pub category: String,
    pub source: String,
    pub partitions: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl DataProduct {
    pub async fn create(&self, ppg: &Pool<Postgres>) -> Result<(), Error> {
        sqlx::query!(
            r#"
                INSERT INTO data_products (id, owner_id, status, format, name, category, source, partitions, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            self.id,
            self.owner_id,
            self.status,
            self.format,
            self.name,
            self.category,
            self.source,
            self.partitions,
            self.created_at,
            self.updated_at)
            .execute(ppg)
            .await?;

        Ok(())
    }

    pub async fn all(ppg: &Pool<Postgres>) -> Result<Vec<DataProduct>, Error> {
        let products = sqlx::query_as!(
            Self,
            r#"
                SELECT * FROM data_products
            "#
        )
        .fetch_all(ppg)
        .await?;

        Ok(products)
    }

    pub async fn get_by_id(
        ppg: &Pool<Postgres>,
        id: &str,
    ) -> Result<Option<DataProduct>, sqlx::Error> {
        let uid = Uuid::parse_str(id).unwrap();

        let product = sqlx::query_as!(
            DataProduct,
            r#"
                SELECT * FROM data_products
                WHERE id = $1
            "#,
            uid
        )
        .fetch_optional(ppg)
        .await?;

        Ok(product)
    }
}
