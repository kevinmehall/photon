use indexmap::IndexMap;
use serde::{Deserialize, Deserializer};
use time::format_description::OwnedFormatItem;

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
    Timestamp { format: TimeFormat },
}

#[derive(Clone)]
pub enum TimeFormat {
    Custom(OwnedFormatItem),
    WellKnown(&'static (dyn time::parsing::Parsable + Send + Sync))
}

impl<'de> Deserialize<'de> for TimeFormat {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;

        if s.eq_ignore_ascii_case("rfc2822") {
            Ok(TimeFormat::WellKnown(&time::format_description::well_known::Rfc2822))
        } else if s.eq_ignore_ascii_case("rfc3339") {
            Ok(TimeFormat::WellKnown(&time::format_description::well_known::Rfc3339))
        } else if s.eq_ignore_ascii_case("iso8601") {
            Ok(TimeFormat::WellKnown(&time::format_description::well_known::Iso8601::PARSING))
        } else {
            time::format_description::parse_owned(&s)
                .map(TimeFormat::Custom)
                .map_err(serde::de::Error::custom)
        }
    }
}

impl TimeFormat {
    pub fn as_format(&self) -> &dyn time::parsing::Parsable {
        match self {
            TimeFormat::Custom(c) => c,
            TimeFormat::WellKnown(f) => f,
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Field {    
    #[serde(flatten)]
    pub parser: Option<ParserKind>,
}

