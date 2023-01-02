use bumpalo::Bump;
use crate::query::FieldVal;
use super::ParserInst;

pub(crate) struct KeywordInst;

impl ParserInst for KeywordInst {
    fn require_field(&mut self, _field: &str) -> Option<usize> {
        None
    }

    fn parse<'b>(&self, bump: &'b Bump, input: &mut FieldVal<'b>) -> &'b mut [FieldVal<'b>] {
        match input {
            FieldVal::Number(n) => {
                *input = FieldVal::String(bumpalo::format!(in bump, "{}", n).into_bump_str());
            }
            _ => {}
        }
        &mut []
    }
}

pub(crate) struct NumberInst;

impl ParserInst for NumberInst {
    fn require_field(&mut self, _field: &str) -> Option<usize> {
        None
    }

    fn parse<'b>(&self, _bump: &'b Bump, input: &mut FieldVal<'b>) -> &'b mut [FieldVal<'b>] {
        match input {
            FieldVal::String(n) => {
                if let Ok(n) = n.parse() {
                    *input = FieldVal::Number(n);
                }
            }
            _ => {}
        }
        &mut []
    }
}
