use crate::{api::query::QueryFilter, query::FieldVal};

pub(crate) fn filter_test(filter: &QueryFilter, val: &FieldVal) -> bool{
    match (filter, val) {
        (QueryFilter::Present { present }, v) => v.exists() == *present,
        (QueryFilter::Range { min, max }, FieldVal::Number(n)) => 
            *n >= min.unwrap_or(f64::NEG_INFINITY) && *n <= max.unwrap_or(f64::INFINITY),
        (QueryFilter::Range { .. }, _) => false,
        (QueryFilter::String(f), FieldVal::String(s)) => f.contains(s),
        (QueryFilter::String(..), _) => false,
    }
}

