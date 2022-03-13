mod file;

use crate::{query::{QueryPlan, QueryError}, ResultSet};

pub(crate) trait Source: Send + Sync {
    fn query(&self, plan: QueryPlan) -> Result<ResultSet, QueryError>;

    fn fields(&self) -> Box<dyn Iterator<Item = (String, crate::api::fields::Field)>>;
}

pub(crate) fn new(spec: &crate::config::dataset::SourceKind) -> Box<impl Source> {
    use crate::config::dataset::SourceKind::*;
    match spec {
        FileLines { path } => Box::new(file::FileLines::new(path)),
    }
}
