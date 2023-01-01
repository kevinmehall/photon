use indexmap::IndexMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Fields {
    pub fields: IndexMap<String, Field>,
}

#[derive(Serialize)]
pub struct Field {
    #[serde(rename = "type")]
    pub ty: FieldType,

    #[serde(flatten)]
    pub display: FieldDisplayConfig,
}

#[derive(Copy, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Keyword,
    Phrase,
    Number,
    Timestamp,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct FieldDisplayConfig {
    values: Option<Vec<String>>,
}