mod file;

use crate::{query::{QueryPlan, QueryError}, ResultSet, ConfigError, FieldDefaults};

pub(crate) trait Source: Send + Sync {
    fn query(&self, plan: QueryPlan) -> Result<ResultSet, QueryError>;

    fn fields(&self) -> Vec<(&str, FieldDefaults)>;
}

pub(crate) fn new(spec: &crate::config::dataset::SourceKind) -> Result<Box<impl Source>, ConfigError> {
    use crate::config::dataset::SourceKind::*;
    Ok(match spec {
        FileLines { path } => Box::new(file::FileLines::new(path).map_err(ConfigError::InvalidConfig)?),
    })
}
