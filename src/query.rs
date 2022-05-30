use indexmap::{IndexMap, IndexSet};
use thiserror::Error;
use time::OffsetDateTime;

use crate::{ api::{self, query::QueryFilter}, parser::{Parser, ParserInst} };

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum FieldVal{
    Null,
    String(String),
    Number(f64),
    Time(OffsetDateTime),
}

impl From<FieldVal> for String {
    fn from(v: FieldVal) -> Self {
        match v {
            FieldVal::Null => String::new(),
            FieldVal::String(s) => s,
            FieldVal::Number(n) => n.to_string(),
            FieldVal::Time(t) => t.format(&time::format_description::well_known::Rfc3339).unwrap(),
        }
    }
}

impl FieldVal {
    pub fn exists(&self) -> bool {
        match self {
            FieldVal::Null => false,
            _ => true
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
    pub (crate) fn new(parser_config: &'a IndexMap<String, (String, Box<dyn Parser>)>, query: &'a api::query::Query) -> Result<QueryPlan<'a>, QueryError> {
        let mut plan = QueryPlan {
            root_fields: IndexSet::new(),
            parsers: IndexMap::new(),
            returning: IndexMap::new(),
            filters: Vec::new(),
        };

        for (field, filter) in query.filter.iter() {
            let loc = plan.require_field(parser_config, field)?;
            plan.filters.push((loc, filter.clone()));
        }

        for field in query.returning.iter() {
            let loc = plan.require_field(parser_config, field)?;
            plan.returning.insert(field, loc);
        }

        Ok(plan)
    }

    fn require_field(&mut self, parser_config: &'a IndexMap<String, (String, Box<dyn Parser>)>, field: &'a str) -> Result<FieldRef, QueryError> {
        Ok(if let Some((parser_dest, leaf_field)) = field.rsplit_once("/") {
            let (src_field_name, parser_impl) = parser_config.get(parser_dest)
                .ok_or_else(|| QueryError::NoParserProvides(parser_dest.to_owned()))?;

            let src = self.require_field(parser_config, src_field_name)?;
            
            let parser_entry = self.parsers.entry(parser_dest);
            let parser_i = parser_entry.index();
            let parser = parser_entry.or_insert_with(|| ParserPlan { 
                src, parser: parser_impl.instance()
             });

            let field_index = parser.parser.require_field(leaf_field)
                .ok_or_else(|| QueryError::FieldNoesNotExist(field.to_owned()))?;

            FieldRef { parser: parser_i + 1, field: field_index }
        } else {
            let field_index = self.root_fields.insert_full(field).0;
            FieldRef{ parser: 0, field: field_index }
        })
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
