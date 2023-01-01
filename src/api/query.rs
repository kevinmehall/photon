use serde::{ Serialize, Deserialize };
use indexmap::{IndexMap, IndexSet};
use time::OffsetDateTime;

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
    KeywordNot{ not: IndexSet<String> }, // { not: [] }
    TimeRange {
        #[serde(with = "time::serde::rfc3339")]
        after: OffsetDateTime,
        #[serde(with = "time::serde::rfc3339")]
        before: OffsetDateTime,
    },
    TimeSince {
        since: f64, // seconds
    },
    Range { min: Option<f64>, max: Option<f64> }, // { min: ..., max: ... }
}

#[test]
fn test_deserialize_query_filter() {
    use time::macros::datetime;
    assert_eq!(serde_json::from_str::<QueryFilter>(r#"{"present": true}"#).unwrap(), QueryFilter::Present{present: true});
    assert_eq!(serde_json::from_str::<QueryFilter>(r#"{"min": 5}"#).unwrap(), QueryFilter::Range{min: Some(5.0), max: None});
    assert_eq!(serde_json::from_str::<QueryFilter>(r#"{"after": "2022-03-30T21:21:23-06:00", "before": "2022-03-30T21:22:01-06:00"}"#).unwrap(), 
        QueryFilter::TimeRange{after: datetime!(2022-03-30 21:21:23-06:00), before: datetime!(2022-03-30 21:22:01-06:00)});
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