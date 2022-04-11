use crate::{api::query::QueryFilter, query::FieldVal};

pub(crate) fn filter_test(filter: &QueryFilter, val: &FieldVal) -> bool{
    match (filter, val) {
        (QueryFilter::Present { present }, v) => v.exists() == *present,
        
        (QueryFilter::Range { min, max }, FieldVal::Number(n)) => 
            *n >= min.unwrap_or(f64::NEG_INFINITY) && *n <= max.unwrap_or(f64::INFINITY),
        (QueryFilter::Range { .. }, _) => false,
        
        (QueryFilter::TimeRange { min, max }, FieldVal::Time(t)) => t >= min && t < max,
        (QueryFilter::TimeRange { .. }, _) => false,
        
        (QueryFilter::KeywordIs { is }, FieldVal::String(s)) => is.contains(s),
        (QueryFilter::KeywordNot { not }, FieldVal::String(s)) => !not.contains(s),
        (QueryFilter::KeywordIs{..} | QueryFilter::KeywordNot{..}, _) => false,
    }
}

