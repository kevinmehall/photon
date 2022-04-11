use time::OffsetDateTime;

use crate::{api::fields::Field, query::FieldVal};

use super::{Parser, ParserInst};

pub struct Timestamp;

static FIELDS: &'static [&'static str] = &["timestamp"];

impl Parser for Timestamp {
    fn instance<'s>(&'s self) -> Box<dyn super::ParserInst + 's> {
        Box::new(Timestamp)
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
        let fmt = time::macros::format_description!("[day]/[month repr:short]/[year]:[hour repr:24]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]");
        
        match OffsetDateTime::parse(input, &fmt) {
            Ok(t) => vec![FieldVal::Time(t)],
            Err(_) => vec![FieldVal::Null],
        }
    }
}