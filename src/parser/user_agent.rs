use crate::{api::fields::Field, query::FieldVal};

use super::{Parser, ParserInst};

pub struct UserAgent;

static FIELDS: &'static [&'static str] = &["category", "browser", "browser.version", "browser.vendor", "os", "os.version"];

impl Parser for UserAgent {
    fn instance<'s>(&'s self) -> Box<dyn super::ParserInst + 's> {
        Box::new(UserAgent)
    }

    fn fields<'s>(&'s self) -> Box<dyn Iterator<Item = (String, crate::api::fields::Field)> + 's> {
        Box::new(FIELDS.iter().map(|f| (f.to_string(), Field { })))
    }
}

impl ParserInst for UserAgent {
    fn require_field(&mut self, field: &str) -> Option<usize> {
        FIELDS.iter().position(|&x| x == field)
    }

    fn parse(&self, input: &str) -> Vec<crate::query::FieldVal> {
        let res = woothee::parser::Parser::new().parse(input);

        if let Some(res) = res {
            vec![
                FieldVal::String(res.category.to_string()),
                FieldVal::String(res.name.to_string()),
                FieldVal::String(res.version.to_string()),
                FieldVal::String(res.vendor.to_string()),
                FieldVal::String(res.os.to_string()),
                FieldVal::String(res.os_version.to_string()),
                FieldVal::String(res.browser_type.to_string()),
            ]
        } else {
            vec![FieldVal::Null; FIELDS.len()]
        }
    }
}