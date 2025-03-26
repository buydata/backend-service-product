use std::fs;

use actix_web::web;
use actix_web::web::Json;
use chrono::Utc;
use log::info;
use minio::s3::args::*;
use sqlx::Error;
use uuid::Uuid;

use crate::error::AppError;
use crate::model::data_product::DataProduct;
use crate::model::forms::upload::UploadForm;
use crate::AppState;

pub async fn create_data_product(
    data: web::Data<AppState>,
    form: UploadForm,
) -> Result<Json<DataProduct>, Error> {
    let product_id = Uuid::new_v4();
    let format = form.format.to_owned();

    info!("Bucket: {:?} create...", &product_id);
    data.s3
        .make_bucket(&MakeBucketArgs::new(&product_id.to_string()).unwrap())
        .await
        .unwrap();

    let mut part_counter: i16 = 0;

    info!("Temp files: {:?}", &form.files);
    for f in form.files {
        part_counter += 1;
        let filename: String = format!("{product_id}_{part_counter}.{format}");
        info!("Filename: {:?}", &filename);
        let path = format!("./tmp/{filename}");
        info!("Path: {:?}", &path);

        f.file.persist(&path).expect("Persist fail");

        let s3_resp = data
            .s3
            .upload_object(
                &mut UploadObjectArgs::new(&*product_id.to_string(), &filename, &path).unwrap(),
            )
            .await
            .unwrap();

        info!("{:?}", s3_resp);
        fs::remove_file(path).expect("Unable to delete temporary file")
    }

    let product = DataProduct::builder()
        .id(product_id)
        .owner_id(*form.owner_id)
        .status("modify".to_string())
        .format(format.to_owned())
        .name(form.name.to_owned())
        .category(form.category.to_owned())
        .source(form.source.to_owned())
        .partitions(part_counter)
        .created_at(Utc::now().naive_utc())
        .updated_at(Utc::now().naive_utc())
        .build();

    DataProduct::create(&product, &data.db).await?;

    Ok(Json(product))
}

pub async fn show_product_data(
    data: web::Data<AppState>,
    product_id: String,
) -> Result<String, AppError> {
    // Получаем продукт
    let product = DataProduct::get_by_id(&data.db, &product_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let object_id = format!("{}_{}.{}", product_id, product.partitions, product.format);

    let args = ObjectConditionalReadArgs::new(&product_id, &object_id)
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let object_data = data.s3.get_object(&args).await?.text().await.unwrap();

    Ok(object_data)
}

pub async fn show_all_products(data: web::Data<AppState>) -> Result<Json<Vec<DataProduct>>, Error> {
    let products = DataProduct::all(&data.db).await?;
    Ok(Json(products))
}
