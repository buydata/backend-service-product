use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, MultipartForm, ToSchema)]
pub struct UploadForm {
    #[schema(value_type = String)]
    pub owner_id: Text<Uuid>,
    #[schema(value_type = String)]
    pub format: Text<String>,
    #[schema(value_type = String)]
    pub name: Text<String>,
    #[schema(value_type = String)]
    pub category: Text<String>,
    #[schema(value_type = String)]
    pub source: Text<String>,
    #[multipart(rename = "file")]
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    pub files: Vec<TempFile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, TypedBuilder, ToSchema)]
pub struct DataProduct {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub status: String,
    pub format: String,
    pub name: String,
    pub category: String,
    pub source: String,
    #[builder(default, setter(strip_option))]
    pub partitions: Option<i16>,
    pub created_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, TypedBuilder)]
pub struct ShowForm {
    pub reader: Uuid,
    pub product_id: Uuid,
}
