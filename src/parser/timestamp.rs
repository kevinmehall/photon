use bumpalo::Bump;
use serde::{Deserialize, Deserializer};
use time::{OffsetDateTime, format_description::OwnedFormatItem, PrimitiveDateTime};

use crate::{query::FieldVal, FieldDefaults};

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
    pub(super) format: TimeFormat,
    pub(super) assume_utc: bool,
}

pub(crate) fn fields() -> Vec<(&'static str, FieldDefaults)> {
    vec![]
}

impl ParserInst for Timestamp {
    fn require_field(&mut self, _field: &str) -> Option<usize> {
        None
    }

    fn parse<'b>(&self, _bump: &'b Bump, input: &mut FieldVal) -> &'b mut [FieldVal<'b>] {
        match input {
            FieldVal::String(s) => {
                let t = if self.assume_utc {
                    PrimitiveDateTime::parse(s, self.format.as_format()).map(|t| t.assume_utc())
                } else {
                    OffsetDateTime::parse(s, self.format.as_format())
                };

                if let Ok(t) = t {
                    *input = FieldVal::Time(t);
                }                
            },
            FieldVal::Null | FieldVal::Number(_) | FieldVal::Time(_) | FieldVal::Map(_) => {},
        }
        &mut []
    }
}