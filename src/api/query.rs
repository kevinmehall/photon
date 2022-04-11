use serde::{ Serialize, Deserialize };
use indexmap::{IndexMap, IndexSet};
use time::{OffsetDateTime, macros::datetime};

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
    TimeRange {
        #[serde(with = "time::serde::rfc3339")]
        min: OffsetDateTime,
        #[serde(with = "time::serde::rfc3339")]
        max: OffsetDateTime,
    },
    Range { min: Option<f64>, max: Option<f64> }, // { min: ..., max: ... }
}

#[test]
fn test_deserialize_query_filter() {
    assert_eq!(serde_json::from_str::<QueryFilter>(r#"{"present": true}"#).unwrap(), QueryFilter::Present{present: true});
    assert_eq!(serde_json::from_str::<QueryFilter>(r#"{"min": 5}"#).unwrap(), QueryFilter::Range{min: Some(5.0), max: None});
    assert_eq!(serde_json::from_str::<QueryFilter>(r#"{"min": "2022-03-30T21:21:23-06:00", "max": "2022-03-30T21:22:01-06:00"}"#).unwrap(), 
        QueryFilter::TimeRange{min: datetime!(2022-03-30 21:21:23-06:00), max: datetime!(2022-03-30 21:22:01-06:00)});
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