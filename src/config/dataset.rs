use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Dataset {
    pub source: Source,

    #[serde(default)]
    pub fields: IndexMap<String, Field>,
}

#[derive(Clone, Deserialize)]
pub struct Source {
    #[serde(flatten)]
    pub kind: SourceKind
}

#[non_exhaustive]
#[derive(Clone, Deserialize)]
#[serde(tag = "source")]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {   
    FileLines { path: String }
}

#[non_exhaustive]
#[derive(Clone, Deserialize)]
#[serde(tag = "parser")]
#[serde(rename_all = "lowercase")]
pub enum ParserKind {
    Dissect { pattern: String },
    UserAgent,
    Timestamp,
}

#[derive(Clone, Deserialize)]
pub struct Field {    
    #[serde(flatten)]
    pub parser: Option<ParserKind>,
}

