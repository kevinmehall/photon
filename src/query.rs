use indexmap::{IndexMap, IndexSet};
use thiserror::Error;
use time::OffsetDateTime;

use crate::{ api::{self, query::QueryFilter}, parser::{ParserInst, self}, Dataset };

#[derive(PartialEq, Clone, Copy, Debug)]
pub(crate) enum FieldVal<'b>{
    Null,
    String(&'b str),
    Number(f64),
    Time(OffsetDateTime),
}

impl<'b> std::fmt::Display for FieldVal<'b> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldVal::Null => Ok(()),
            FieldVal::String(s) => f.write_str(s),
            FieldVal::Number(n) => n.fmt(f),
            FieldVal::Time(t) => f.write_str(&t.format(&time::format_description::well_known::Rfc3339).unwrap()), // https://github.com/time-rs/time/issues/375
        }
    }
}

impl<'b> From<FieldVal<'b>> for String {
    fn from(v: FieldVal) -> Self {
        match v {
            FieldVal::Null => String::new(),
            FieldVal::String(s) => s.to_string(),
            FieldVal::Number(n) => n.to_string(),
            FieldVal::Time(t) => t.format(&time::format_description::well_known::Rfc3339).unwrap(),
        }
    }
}

impl<'b> FieldVal<'b> {
    pub fn exists(&self) -> bool {
        match self {
            FieldVal::Null => false,
            _ => true
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            FieldVal::String(s) => Some(s),
            _ => None,
        }
    }
}
pub (crate) struct QueryPlan<'a> {
    pub root_fields: IndexSet<&'a str>,
    pub parsers: IndexMap<&'a str, ParserPlan<'a>>,
    pub returning: IndexMap<&'a str, FieldRef>,
    pub filters: Vec<(FieldRef, QueryFilter)>,
}

pub (crate) struct ParserPlan<'a> {
    pub src: FieldRef,
    pub parser: Box<dyn ParserInst + 'a>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct FieldRef {
    pub parser: usize,
    pub field: usize,
}

impl<'a> QueryPlan<'a> {
    pub (crate) fn new(dataset: &'a Dataset, query: &'a api::query::Query) -> Result<QueryPlan<'a>, QueryError> {
        let mut plan = QueryPlan {
            root_fields: IndexSet::new(),
            parsers: IndexMap::new(),
            returning: IndexMap::new(),
            filters: Vec::new(),
        };

        for (field, filter) in query.filter.iter() {
            let loc = plan.require_field(dataset, field)?;
            plan.filters.push((loc, filter.clone()));
        }

        for field in query.returning.iter() {
            let loc = plan.require_field(dataset, field)?;
            plan.returning.insert(field, loc);
        }

        Ok(plan)
    }

    fn require_parser<'s>(&'s mut self, dataset: &'a Dataset, field: &'a str) -> Result<(FieldRef, Option<(usize, &'s mut ParserPlan<'a>)>) , QueryError> {
        let src = if let Some((parent_field_name, leaf_field)) = field.rsplit_once("/") {
            let (parser_i, parser) = self.require_parser(dataset, parent_field_name)?.1
                .ok_or_else(|| QueryError::NoParserProvides(parent_field_name.to_owned()))?;

            let field_index = parser.parser.require_field(leaf_field)
                .ok_or_else(|| QueryError::FieldNoesNotExist(field.to_owned()))?;

            FieldRef { parser: parser_i + 1, field: field_index }
        } else {
            let field_index = self.root_fields.insert_full(field).0;
            FieldRef{ parser: 0, field: field_index }
        };

        let p = dataset.fields.get(field).and_then(|f| f.parser.as_ref()).map(|parser_conf| {
            let parser_entry = self.parsers.entry(field);
            let parser_i = parser_entry.index();
            let parser = parser_entry.or_insert_with(|| ParserPlan { 
                src, parser: parser::instance(parser_conf)
            });
            (parser_i, parser)
        });

        Ok((src, p))
    }

    fn require_field(&mut self, dataset: &'a Dataset, field: &'a str) -> Result<FieldRef, QueryError> {
        Ok(self.require_parser(dataset, field)?.0)
    }
}

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("No parser provides field `{0}`")]
    NoParserProvides(String),

    #[error("Field `{0}` does not exist")]
    FieldNoesNotExist(String),
}
