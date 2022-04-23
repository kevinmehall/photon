use indexmap::IndexMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct RootResponse {
    pub version: &'static str,
    pub datasets: IndexMap<String, Dataset>,
}

#[derive(Serialize)]
pub struct Dataset {
    pub ok: bool,
}