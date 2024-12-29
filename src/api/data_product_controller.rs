use crate::model::data_product::UploadForm;
use crate::service::data_product_service::{
    create_data_product, show_all_products, show_data_product,
};
use crate::AppState;
use actix_multipart::form::MultipartForm;
use actix_web::http::{Error, StatusCode};
use actix_web::{get, post, web, HttpResponse};

#[post("/create_data_product")]
pub async fn create(
    MultipartForm(form): MultipartForm<UploadForm>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let result = create_data_product(data, form).await;

    log::debug!("REQ: {result:?}");

    match result {
        Ok(result) => Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(result)),
        Err(_) => Ok(HttpResponse::Ok().status(StatusCode::BAD_GATEWAY).body(())),
    }
}

#[get("/show_data_product/{product_id}")]
pub async fn show(
    data: web::Data<AppState>,
    product_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    log::debug! {"Product id: {:?}", product_id}
    let result = show_data_product(data, product_id.to_string()).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().body(result)),
        Err(_) => Ok(HttpResponse::Ok().status(StatusCode::BAD_GATEWAY).body(())),
    }
}

#[get("/catalog/products")]
pub async fn products(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let result = show_all_products(data).await;
    match result {
        Ok(result) => Ok(HttpResponse::Ok().json(result)),
        Err(_) => Ok(HttpResponse::Ok().status(StatusCode::BAD_GATEWAY).body(())),
    }
}
