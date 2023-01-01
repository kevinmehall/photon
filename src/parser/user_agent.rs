use bumpalo::Bump;

use crate::{query::FieldVal, FieldDefaults, api::fields::FieldType};

use super::ParserInst;

pub struct UserAgent;

const FIELDS: &'static [&'static str] = &["category", "browser", "browser.version", "browser.vendor", "os", "os.version"];

pub(crate) fn fields() -> Vec<(&'static str, FieldDefaults)> {
    FIELDS.iter().map(|&name| (name, FieldDefaults { ty : FieldType::Keyword })).collect()
}

impl ParserInst for UserAgent {
    fn require_field(&mut self, field: &str) -> Option<usize> {
        FIELDS.iter().position(|&x| x == field)
    }

    fn parse<'b>(&self, bump: &'b Bump, input: &mut FieldVal) -> &'b mut [FieldVal<'b>] {
        let input = input.as_str().unwrap_or_default();
        let res = woothee::parser::Parser::new().parse(input);

        if let Some(res) = res {
            bump.alloc([
                FieldVal::String(bump.alloc_str(res.category)),
                FieldVal::String(bump.alloc_str(res.name)),
                FieldVal::String(bump.alloc_str(res.version)),
                FieldVal::String(bump.alloc_str(res.vendor)),
                FieldVal::String(bump.alloc_str(res.os)),
                FieldVal::String(bump.alloc_str(bump.alloc_str(res.os_version.as_ref()))),
                FieldVal::String(bump.alloc_str(res.browser_type)),
            ])
        } else {
            bump.alloc([FieldVal::Null; FIELDS.len()])
        }
    }
}