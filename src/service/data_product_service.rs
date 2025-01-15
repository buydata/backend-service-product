use std::fs;

use actix_web::web;
use actix_web::web::Json;
use chrono::Utc;
use minio::s3::args::*;
use minio::s3::error::Error as MinioError;
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

use crate::model::data_product::{DataProduct, UploadForm};
use crate::AppState;

impl DataProduct {
    pub async fn create(
        product: DataProduct,
        ppg: &Pool<Postgres>,
        cnt: i16,
    ) -> Result<DataProduct, Error> {
        let query = sqlx::query!(
            r#"
                INSERT INTO data_products (id, owner_id, status, format, name, category, source, partitions, created_at, update_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            product.id,
            product.owner_id,
            product.status,
            product.format,
            product.name,
            product.category,
            product.source,
            cnt,
            product.created_at,
            product.update_at)
            .execute(ppg)
            .await;

        match query {
            Ok(_) => Ok(product),
            Err(error) => Err(error),
        }
    }

    pub async fn show_all(ppg: &Pool<Postgres>) -> Result<Vec<DataProduct>, Error> {
        let query = sqlx::query_as!(
            DataProduct,
            r#"
                SELECT * FROM data_products
            "#
        )
        .fetch_all(ppg)
        .await;

        match query {
            Ok(query) => Ok(query),
            Err(error) => Err(error),
        }
    }
}

pub async fn create_data_product(
    data: web::Data<AppState>,
    form: UploadForm,
) -> Result<Json<DataProduct>, Error> {
    let product_id = Uuid::new_v4();
    let format = form.format.to_owned();

    let product = DataProduct::builder()
        .id(product_id)
        .owner_id(*form.owner_id)
        .status("modify".to_string())
        .format(format.to_owned())
        .name(form.name.to_owned())
        .category(form.category.to_owned())
        .source(form.source.to_owned())
        .created_at(Utc::now().naive_utc())
        .update_at(Utc::now().naive_utc())
        .build();

    let product_id = &product.id;
    let format = &product.format;

    let exists = data
        .s3
        .bucket_exists(&BucketExistsArgs::new(&product_id.to_string()).unwrap())
        .await
        .unwrap();

    if !exists {
        data.s3
            .make_bucket(&MakeBucketArgs::new(&product_id.to_string()).unwrap())
            .await
            .unwrap();
    }

    let mut part_counter: i16 = 0;

    for f in form.files {
        let filename: String = format!("{product_id}_{part_counter}.{format}");
        let path = format!("./tmp/{filename}");
        f.file.persist(&path).unwrap();

        data.s3
            .upload_object(
                &mut UploadObjectArgs::new(&*product_id.to_string(), &filename, &path).unwrap(),
            )
            .await
            .unwrap();

        part_counter += 1;
        fs::remove_file(path).expect("Unable to delete temporary file")
    }

    let product = DataProduct::create(product, &data.db, part_counter).await?;

    Ok(Json(product))
}

pub async fn show_data_product(
    data: web::Data<AppState>,
    product_id: String,
) -> Result<String, MinioError> {
    let topic = &product_id;
    let object_id = &format!("{topic}_0.json");
    let args = ObjectConditionalReadArgs::new(topic, object_id).unwrap();

    let data = data.s3.get_object(&args).await?.text().await?;

    Ok(data)
}

pub async fn show_all_products(data: web::Data<AppState>) -> Result<Json<Vec<DataProduct>>, Error> {
    let products = DataProduct::show_all(&data.db).await?;
    Ok(Json(products))
}
