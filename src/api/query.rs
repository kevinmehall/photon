use serde::{ Serialize, Deserialize };
use indexmap::{IndexMap, IndexSet};

#[derive(Deserialize)]
pub struct Query {
    pub filter: IndexMap<String, QueryFilter>,
    pub returning: IndexSet<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum QueryFilter {
    Present { present: bool }, // { present: true }
    KeywordIs { is: IndexSet<String> }, // { is: [] }
    KeywordNot{ not: IndexSet<String> }, // { isNot: [] }
    Range { min: Option<f64>, max: Option<f64> }, // { min: ..., max: ... }
}

#[test]
fn test_deserialize_query_filter() {
    assert_eq!(serde_json::from_str::<QueryFilter>(r#"{"present": true}"#).unwrap(), QueryFilter::Present{present: true});
    assert_eq!(serde_json::from_str::<QueryFilter>(r#"{"min": 5}"#).unwrap(), QueryFilter::Range{min: Some(5.0), max: None});

}

#[derive(Serialize)]
pub struct Response<C> {
    //pub stats: ResponseStats,
    pub results: C,
}

#[derive(Serialize)]
pub struct ResponseStats {
    pub rows_scanned: u64,
}