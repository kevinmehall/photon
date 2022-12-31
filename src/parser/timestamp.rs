use serde::{Deserialize, Deserializer};
use time::{OffsetDateTime, format_description::OwnedFormatItem};

use crate::query::FieldVal;

use super::ParserInst;


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

#[derive(Clone)]
pub struct Timestamp {
    pub(super) format: TimeFormat
}

static FIELDS: &'static [&'static str] = &["timestamp"];

pub(crate) fn fields() -> Vec<&'static str> {
    FIELDS.into()
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