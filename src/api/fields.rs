use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct Fields {
    pub fields: IndexMap<String, Field>,
}

#[derive(Serialize)]
pub struct Field {
    #[serde(rename = "type")]
    pub ty: FieldType,
}

#[derive(Copy, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Keyword,
    Phrase,
    Number,
    Timestamp,
}