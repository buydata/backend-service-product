use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
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
    #[schema(value_type = String, format = Binary, content_media_type = "application/octet-stream")]
    pub files: Vec<TempFile>,
}
