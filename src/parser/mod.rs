use crate::query::FieldVal;

mod dissect;
mod user_agent;

pub(crate) trait Parser: Send + Sync {
    fn instance<'s>(&'s self) -> Box<dyn ParserInst + 's>;

    fn fields<'s>(&'s self) -> Box<dyn Iterator<Item = (String, crate::api::fields::Field)> + 's>;
}

pub(crate) trait ParserInst: Send {
    fn require_field(&mut self, field: &str) -> Option<usize>;

    fn parse(&self, input: &str) -> Vec<FieldVal>;
}

pub(crate) fn new(spec: &crate::config::dataset::ParserKind) -> Box<dyn Parser> {
    use crate::config::dataset::ParserKind::*;
    match spec {
        Dissect { pattern } => Box::new(dissect::Dissect::new(pattern).unwrap()),
        UserAgent => Box::new(user_agent::UserAgent),
    }
}
