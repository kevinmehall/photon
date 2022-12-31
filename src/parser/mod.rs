use crate::query::FieldVal;

pub mod dissect;
pub mod user_agent;
pub mod timestamp;

pub(crate) trait ParserInst: Send {
    fn require_field(&mut self, field: &str) -> Option<usize>;

    fn parse(&self, input: &str) -> Vec<FieldVal>;
}

pub(crate) fn child_fields(spec: &crate::config::dataset::ParserKind) -> Vec<&str> {
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
