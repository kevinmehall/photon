use std::{fs, path::{Path, PathBuf}, io};

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

pub struct Dataset {
    source: Box<dyn source::Source>,
    parsers: IndexMap<String, (String, Box<dyn parser::Parser>)>,
}

impl Dataset {
    pub fn from_config(conf: &config::dataset::Dataset) -> Result<Dataset, ConfigError> {
        let source = source::new(&conf.source.kind)?;
        let mut parsers = IndexMap::new();

        for p in conf.parsers.iter() {
            let field = p.field.clone().unwrap_or("".to_owned());
            let dest = p.dest.clone().unwrap_or_else(|| field.clone());
            let parser = parser::new(&p.kind)?;
            parsers.insert(dest, (field, parser));
        }

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
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Toml(#[from] toml::de::Error),

    #[error("{0}")]
    InvalidConfig(&'static str),
}