use crate::model::data_product::DataProduct;
use crate::model::forms::upload::UploadForm;
use crate::service::data_product_service::{
    create_data_product, show_all_products, show_product_data,
};
use crate::AppState;

use actix_multipart::form::MultipartForm;
use actix_web::http::StatusCode;
use actix_web::{error, get, post, web, Error, HttpResponse};

/// Create product
#[utoipa::path(
    responses(
        (status = 201, description = "Product created successfully"),
        (status = BAD_GATEWAY, description = "Product created successfully fail")
    ),
    request_body(content = UploadForm, content_type = "multipart/form-data")
)]
#[post("/products")]
pub async fn create(
    MultipartForm(form): MultipartForm<UploadForm>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    create_data_product(data, form)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().status(StatusCode::CREATED).finish())
}

/// Get product by id
#[utoipa::path(
    responses(
        (status = 200, description = "Product found successfully", body = DataProduct),
        (status = NOT_FOUND, description = "Product was not found")
    ),
    params(
        ("product_id" = Uuid, Path, description = "Product database id to get Product for"),
    )
)]
#[get("/products/{product_id}")]
pub async fn show(
    data: web::Data<AppState>,
    product_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    log::debug! {"Product id: {:?}", product_id}
    let result = show_product_data(data, product_id.to_string())
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(result))
}

/// Get all products
#[utoipa::path(
    responses(
        (status = 200, description = "Products found successfully", body = Vec<DataProduct>),
        (status = NOT_FOUND, description = "Products was not found")
    )
)]
#[get("/products")]
pub async fn products(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let products = show_all_products(data)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(products))
}
