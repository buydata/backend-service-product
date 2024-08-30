use std::fs;
use actix_web::web;
use actix_web::web::Json;
use chrono::Utc;
use minio::s3::args::*;
use minio::s3::error::Error as MinioError;
use sqlx::Error;
use uuid::Uuid;

use crate::AppState;
use crate::model::data_product::{DataProduct, ShowForm, UploadForm};

pub async fn create_data_product(
    data: web::Data<AppState>,
    form: UploadForm,
) -> Result<Json<DataProduct>, Error> {
    let product_id = Uuid::new_v4();
    let type_format = form.type_x.to_owned();

    let product = DataProduct::builder()
        .id(product_id)
        .owner_id(Some(*form.owner_id))
        .status("modify".to_string())
        .type_format(type_format.to_owned())
        .category("hz".to_string())
        .created_at(Utc::now())
        .update_at(Utc::now())
        .build();

    let product_id = &product.id;
    let type_format = &product.type_format;

    let exists = data
        .s3
        .bucket_exists(&BucketExistsArgs::new(&*product_id.to_string()).unwrap())
        .await
        .unwrap();

    if !exists {
        data.s3
            .make_bucket(&MakeBucketArgs::new(&*product_id.to_string()).unwrap())
            .await
            .unwrap();
    }

    let mut part_counter: i16 = 0;

    for f in form.files {
        let filename: String = format!("{product_id}_{part_counter}.{type_format}");
        let path = format!("./tmp/{filename}");
        f.file.persist(&path).unwrap();

        data.s3
            .upload_object(&mut UploadObjectArgs::new(&*product_id.to_string(), &filename, &path).unwrap())
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
    form: Json<ShowForm>
) -> Result<String, MinioError> {
    let topic = &*form.product_id.to_string();
    let object_id = &*format!("{topic}_0.csv");
    let args = ObjectConditionalReadArgs::new(
        topic,
        object_id
    ).unwrap();

    let data = data.s3
        .get_object(&args)
        .await?
        .text()
        .await?;

    Ok(data)
}
