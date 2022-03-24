use crate::{query::FieldVal, ConfigError};

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

pub(crate) fn new(spec: &crate::config::dataset::ParserKind) -> Result<Box<dyn Parser>, ConfigError> {
    use crate::config::dataset::ParserKind::*;
    Ok(match spec {
        Dissect { pattern } => Box::new(dissect::Dissect::new(pattern).map_err(ConfigError::InvalidConfig)?),
        UserAgent => Box::new(user_agent::UserAgent),
    })
}
