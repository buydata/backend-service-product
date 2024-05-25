use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, TypedBuilder)]
pub struct ShowForm {
    pub reader: Uuid,
    pub product_id: Uuid,
}
