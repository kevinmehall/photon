use bumpalo::Bump;
use bumpalo::collections::Vec as BVec;
use serde::{Deserialize, Deserializer};

use crate::{query::FieldVal, FieldDefaults, api::fields::FieldType};

use super::ParserInst;

#[derive(Clone)]
pub struct Dissect {
    literals: Vec<String>,
    fields: Vec<String>,
}

impl Dissect {
    pub fn new(mut pattern: &str) -> Result<Dissect, &'static str> {
        let mut literals = Vec::new();
        let mut fields = Vec::new();

        while let Some((prefix, rest)) = pattern.split_once("%{") {
            literals.push(prefix.to_owned());

            if let Some((field, rest)) = rest.split_once("}") {
                fields.push(field.to_owned());
                pattern = rest;
            } else { return Err("unterminated matcher in dissect pattern") };
        }

        if !pattern.is_empty() {
            literals.push(pattern.to_owned());
        }

        Ok(Dissect { literals, fields })
    }


    fn parse_with<'a>(&self, mut s: &'a str, mut push: impl FnMut(&'a str)) -> bool {
        if let Some(next) = s.strip_prefix(&self.literals[0]) { s = next } else { return false };

        for delim in &self.literals[1..] {
            if let Some((extract, next)) = s.split_once(delim) {
                push(extract);
                s = next;
            } else {
                return false;
            }
        }

        if self.literals.len() == self.fields.len() {
            push(s);
        } else if !s.is_empty() {
            return false;
        }
    
        true
    }

    #[cfg(test)]
    fn parse<'a>(&self, s: &'a str) -> Option<Vec<&'a str>> {
        let mut results = Vec::new();
        if self.parse_with(s, |v| results.push(v)) {
            debug_assert_eq!(results.len(), self.fields.len());
            Some(results)
        } else { None }
    }
}

impl<'de> Deserialize<'de> for Dissect {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Dissect::new(&s).map_err(serde::de::Error::custom)
    }
}

pub(crate) fn fields(s: &Dissect) -> Vec<(&str, FieldDefaults)> {
    s.fields.iter().map(|x| (&x[..], FieldDefaults { ty: FieldType::Keyword })).collect()
}

pub(crate) struct DissectInst<'req>(pub(crate) &'req Dissect);

impl<'req> ParserInst for DissectInst<'req> {
    fn require_field(&mut self, field: &str) -> Option<usize> {
        self.0.fields.iter().position(|x| x == field)
    }    

    fn parse<'b>(&self, bump: &'b Bump, input: &mut FieldVal<'b>) -> &'b mut [FieldVal<'b>] {
        let input = input.as_str().unwrap_or_default();
        let mut results = BVec::with_capacity_in(self.0.fields.len(), bump);
        if self.0.parse_with(input, |v| results.push(FieldVal::String(bump.alloc_str(v)))) {
            debug_assert_eq!(results.len(), self.0.fields.len());
            results.into_bump_slice_mut()
        } else { &mut [] }
    }
}

#[test]
fn test() {
    let literal = Dissect::new("literal").unwrap();
    assert!(literal.fields.is_empty());
    assert_eq!(literal.parse("literal"), Some(vec![]));
    assert_eq!(literal.parse("xliteral"), None);
    assert_eq!(literal.parse("literalx"), None);
    assert_eq!(literal.parse(""), None);

    let full = Dissect::new("%{m}").unwrap();
    assert_eq!(full.fields, vec!["m"]);
    assert_eq!(full.parse("abc"), Some(vec!["abc"]));
    assert_eq!(full.parse(""), Some(vec![""]));

    let prefix = Dissect::new("=%{m}").unwrap();
    assert_eq!(prefix.fields, vec!["m"]);
    assert_eq!(prefix.parse("abc"), None);
    assert_eq!(prefix.parse("=abc"), Some(vec!["abc"]));
    assert_eq!(prefix.parse("="), Some(vec![""]));

    let suffix = Dissect::new("%{m}=").unwrap();
    assert_eq!(suffix.fields, vec!["m"]);
    assert_eq!(suffix.parse("abc"), None);
    assert_eq!(suffix.parse("abc="), Some(vec!["abc"]));
    assert_eq!(suffix.parse("="), Some(vec![""]));

    let sep = Dissect::new("%{field1},%{field2}").unwrap();
    assert_eq!(sep.fields, vec!["field1", "field2"]);
    assert_eq!(sep.parse("abc"), None);
    assert_eq!(sep.parse("abc,def"), Some(vec!["abc", "def"]));

    let example = Dissect::new(r#"%{clientip} %{ident} %{auth} [%{timestamp}] "%{verb} %{request} HTTP/%{httpversion}" %{status} %{size}"#).unwrap();
    assert_eq!(example.fields, vec!["clientip", "ident", "auth", "timestamp", "verb", "request", "httpversion", "status", "size"]);
    assert_eq!(example.parse("1.2.3.4 - - [30/Apr/1998:22:00:52 +0000] \"GET /some/path?a=b HTTP/1.0\" 200 3171"),
        Some(vec!["1.2.3.4", "-", "-", "30/Apr/1998:22:00:52 +0000", "GET", "/some/path?a=b", "1.0", "200", "3171"]));
}