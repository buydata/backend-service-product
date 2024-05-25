use actix_multipart::form::MultipartForm;
use actix_web::{get, HttpResponse, post, web};
use actix_web::http::{Error, StatusCode};
use actix_web::web::Json;

use crate::AppState;
use crate::model::data_product::{ShowForm, UploadForm};
use crate::service::data_product_service::{create_data_product, show_data_product};

#[post("/create_data_product")]
async fn create(
    MultipartForm(form): MultipartForm<UploadForm>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {

    let result = create_data_product(data, form).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().status(StatusCode::CREATED).json(result)),
        Err(_) => Ok(HttpResponse::Ok().status(StatusCode::BAD_GATEWAY).body(()))
    }
}

#[get("/show_data_product")]
async fn show(
    data: web::Data<AppState>,
    form: Json<ShowForm>
) -> Result<HttpResponse, Error> {

    let result = show_data_product(data, form).await;

    match result {
        Ok(result) => Ok(HttpResponse::Ok().body(result)),
        Err(_) => Ok(HttpResponse::Ok().status(StatusCode::BAD_GATEWAY).body(()))
    }
}