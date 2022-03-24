use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Dataset {
    pub source: Source,
    pub parsers: Vec<Parser>,
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

#[derive(Clone, Deserialize)]
pub struct Parser {
    pub field: Option<String>,
    pub dest: Option<String>,

    #[serde(flatten)]
    pub kind: ParserKind,
}

#[non_exhaustive]
#[derive(Clone, Deserialize)]
#[serde(tag = "parser")]
#[serde(rename_all = "lowercase")]
pub enum ParserKind {
    Dissect { pattern: String },
    UserAgent,
}
