use crate::{query::FieldVal, FieldDefaults, api::fields::FieldType};

pub mod dissect;
pub mod user_agent;
pub mod timestamp;

pub(crate) trait ParserInst: Send {
    fn require_field(&mut self, field: &str) -> Option<usize>;

    fn parse(&self, input: &mut FieldVal) -> Vec<FieldVal>;
}

pub(crate) fn ty(spec: &crate::config::dataset::ParserKind) -> FieldType {
    use crate::config::dataset::ParserKind::*;
    match spec {
        Dissect { .. } => FieldType::Phrase,
        UserAgent => FieldType::Phrase,
        Timestamp { .. } => FieldType::Timestamp,
    }
}

pub(crate) fn child_fields(spec: &crate::config::dataset::ParserKind) -> Vec<(&str, FieldDefaults)> {
    use crate::config::dataset::ParserKind::*;
    match spec {
        Dissect { pattern } => dissect::fields(pattern),
        UserAgent => user_agent::fields(),
        Timestamp { .. } => timestamp::fields(),
    }
}

pub(crate) fn instance<'a>(spec: &'a crate::config::dataset::ParserKind) -> Box<dyn ParserInst + 'a> {
    use crate::config::dataset::ParserKind::*;
    match spec {
        Dissect { pattern } => Box::new(dissect::DissectInst(pattern)),
        UserAgent => Box::new(user_agent::UserAgent),
        Timestamp { format } => Box::new(timestamp::Timestamp { format: format.clone() }),
    }
}
