use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct Fields {
    pub fields: IndexMap<String, Field>,
}

#[derive(Serialize)]
pub struct Field {

}