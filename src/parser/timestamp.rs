use time::OffsetDateTime;

use crate::{api::fields::Field, query::FieldVal, config::dataset::TimeFormat};

use super::{Parser, ParserInst};

#[derive(Clone)]
pub struct Timestamp {
    pub(super) format: TimeFormat
}

static FIELDS: &'static [&'static str] = &["timestamp"];

impl Parser for Timestamp {
    fn instance<'s>(&'s self) -> Box<dyn super::ParserInst + 's> {
        Box::new(self.clone())
    }

    fn fields<'s>(&'s self) -> Box<dyn Iterator<Item = (String, crate::api::fields::Field)> + 's> {
        Box::new(FIELDS.iter().map(|f| (f.to_string(), Field { })))
    }
}

impl ParserInst for Timestamp {
    fn require_field(&mut self, field: &str) -> Option<usize> {
        FIELDS.iter().position(|&x| x == field)
    }

    fn parse(&self, input: &str) -> Vec<FieldVal> {
        match OffsetDateTime::parse(input, self.format.as_format()) {
            Ok(t) => vec![FieldVal::Time(t)],
            Err(_) => vec![FieldVal::Null],
        }
    }
}