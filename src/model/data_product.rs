use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub owner_id: Text<Uuid>,
    pub format: Text<String>,
    pub name: Text<String>,
    pub category: Text<String>,
    pub source: Text<String>,
    #[multipart(rename = "file")]
    pub files: Vec<TempFile>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, TypedBuilder)]
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
