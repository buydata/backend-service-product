use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, TypedBuilder)]
pub struct ShowForm {
    pub reader: Uuid,
    pub product_id: Uuid,
}
