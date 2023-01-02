use bumpalo::Bump;

use crate::{query::FieldVal, FieldDefaults, api::fields::FieldType};

pub mod dissect;
pub mod user_agent;
pub mod timestamp;
mod casts;
mod json;

pub(crate) trait ParserInst: Send {
    fn require_field(&mut self, field: &str) -> Option<usize>;

    fn parse<'b>(&self, bump: &'b Bump, input: &mut FieldVal<'b>) -> &'b mut [FieldVal<'b>];
}

pub(crate) fn ty(spec: &crate::config::dataset::ParserKind) -> FieldType {
    use crate::config::dataset::ParserKind::*;
    match spec {
        Keyword => FieldType::Keyword,
        Number => FieldType::Number,
        Dissect { .. } => FieldType::Phrase,
        UserAgent => FieldType::Phrase,
        Timestamp { .. } => FieldType::Timestamp,
        Json => FieldType::Phrase,
    }
}

pub(crate) fn child_fields(spec: &crate::config::dataset::ParserKind) -> Vec<(&str, FieldDefaults)> {
    use crate::config::dataset::ParserKind::*;
    match spec {
        Keyword | Number => vec![],
        Dissect { pattern } => dissect::fields(pattern),
        UserAgent => user_agent::fields(),
        Timestamp { .. } => timestamp::fields(),
        Json => vec![],
    }
}

pub(crate) fn instance<'a>(spec: &'a crate::config::dataset::ParserKind) -> Box<dyn ParserInst + 'a> {
    use crate::config::dataset::ParserKind::*;
    match spec {
        Keyword => Box::new(casts::KeywordInst),
        Number => Box::new(casts::NumberInst),
        Dissect { pattern } => Box::new(dissect::DissectInst(pattern)),
        UserAgent => Box::new(user_agent::UserAgent),
        Timestamp { format, assume_utc} => Box::new(timestamp::Timestamp { format: format.clone(), assume_utc: *assume_utc }),
        Json => Box::new(json::Json::new()),
    }
}

