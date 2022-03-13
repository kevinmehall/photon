use std::{fs, path::Path};

use indexmap::IndexMap;

pub mod api;
pub mod config;
mod source;
mod parser;
mod filter;
mod query;
mod resultset;

use thiserror::Error;

use query::QueryPlan;
pub use resultset::ResultSet;
pub use query::QueryError;

pub struct Dataset {
    source: Box<dyn source::Source>,
    parsers: IndexMap<String, (String, Box<dyn parser::Parser>)>,
}

impl Dataset {
    pub fn from_config(conf: &config::dataset::Dataset) -> Result<Dataset, ConfigError> {
        let source = source::new(&conf.source.kind);
        let parsers = conf.parsers.iter().map(|p| {
            let field = p.field.clone().unwrap_or("".to_owned());
            let dest = p.dest.clone().unwrap_or_else(|| field.clone());
            let parser = parser::new(&p.kind);
            (dest, (field, parser))
        }).collect();

        Ok(Self { source, parsers })
    }

    pub fn from_config_file(fname: impl AsRef<Path>) -> Result<Dataset, ConfigError> {
        let data = fs::read(fname)?;
        let config = toml::from_slice(&data)?;
        Self::from_config(&config)
    }

    pub fn query(&self, q: &api::query::Query) -> Result<ResultSet, QueryError> {
        let plan = QueryPlan::new(&self.parsers, q)?;
        self.source.query(plan)
    }

    pub fn fields(&self) -> api::fields::Fields {
        let mut fields: IndexMap<String, _> = self.source.fields().collect();

        for (prefix, (_, parser)) in &self.parsers {
            fields.extend(parser.fields().map(|(name, field)| (format!("{prefix}/{name}"), field)))
        }

        api::fields::Fields { fields }
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("TOML error")]
    Toml(#[from] toml::de::Error)
}