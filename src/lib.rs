use std::{fs, path::{Path, PathBuf}, io};
use indexmap::IndexMap;

pub mod api;
pub mod config;
mod source;
mod parser;
mod filter;
mod query;
mod resultset;

use source::Source;
use thiserror::Error;

use query::QueryPlan;
pub use resultset::ResultSet;
pub use query::QueryError;

pub struct Config {
    config_dir: PathBuf,
    datasets: IndexMap<String, Result<Dataset, ConfigError>>,
}

impl Config {
    pub fn load(config_dir: PathBuf) -> Result<Config, io::Error> {
        let mut config = Config { config_dir, datasets: Default::default() };
        config.reload()?;
        Ok(config)
    }

    pub fn reload(&mut self) -> Result<(), io::Error> {
        self.datasets = fs::read_dir(&self.config_dir)?
            .filter_map(|f| f.ok())
            .filter_map(|f| {
                if let Some(name) = f.file_name().to_str().and_then(|name| name.strip_suffix(".dataset.toml")) {
                    let dataset = Dataset::from_config_file(f.path());
                    
                    if let Err(e) = &dataset {
                        eprintln!("Configuration error for dataset `{name}`: {e}")
                    }

                    Some((name.to_owned(), dataset))
                } else {
                    None
                }
            })
            .collect();
        Ok(())
    }

    pub fn datasets(&self) -> impl Iterator<Item=(&str, Result<&Dataset, &ConfigError>)> {
        self.datasets.iter()
            .map(|(name, ds)| (&name[..], ds.as_ref()))
    }

    pub fn dataset(&self, name: &str) -> Option<Result<&Dataset, &ConfigError>> {
        self.datasets.get(name).map(|x| x.as_ref())
    }
}

#[derive(Default)]
pub(crate) struct Field {
    pub(crate) parser: Option<Box<dyn parser::Parser>>,
}

pub struct Dataset {
    source: Box<dyn source::Source>,
    fields: IndexMap<String, Field>,
}

impl Dataset {
    pub fn from_config(conf: &config::dataset::Dataset) -> Result<Dataset, ConfigError> {
        let source = source::new(&conf.source.kind)?;
        let mut fields: IndexMap<String, Field> = IndexMap::new();

        for (field_name, _) in source.fields() {
            fields.entry(field_name).or_default();
        }

        for (field_name, conf_field) in &conf.fields {
            let (parser, child_fields) = if let Some(p) = &conf_field.parser {
                let parser = parser::new(p)?;
                let child_fields = parser.fields().collect();
                (Some(parser), child_fields)
            } else {
                (None, Vec::new())
            };

            

            let mut f = fields.entry(field_name.clone()).or_default();
            assert!(f.parser.is_none());
            f.parser = parser;

            // Create child fields defined by parser
            for (child_field, _) in child_fields {
                fields.entry(format!("{field_name}/{child_field}")).or_default();
            }
        }

        Ok(Self { source, fields })
    }

    pub fn from_config_file(fname: impl AsRef<Path>) -> Result<Dataset, ConfigError> {
        let data = fs::read(fname)?;
        let config = toml::from_slice(&data)?;
        Self::from_config(&config)
    }

    pub fn query(&self, q: &api::query::Query) -> Result<ResultSet, QueryError> {
        let plan = QueryPlan::new(&self, q)?;
        self.source.query(plan)
    }

    pub fn fields(&self) -> api::fields::Fields {
        let fields = self.fields.iter().map(|(k, _)| {
            (k.to_owned(), api::fields::Field {})
        }).collect();
        api::fields::Fields { fields }
    }
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Toml(#[from] toml::de::Error),

    #[error("{0}")]
    InvalidConfig(&'static str),
}