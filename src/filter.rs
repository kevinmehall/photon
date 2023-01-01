use time::OffsetDateTime;
use time::ext::NumericalDuration;

use crate::{api::query::QueryFilter, query::FieldVal};

pub(crate) fn filter_test(filter: &QueryFilter, val: &FieldVal) -> bool{
    match (filter, val) {
        (QueryFilter::Present { present }, v) => v.exists() == *present,
        
        (QueryFilter::Range { min, max }, FieldVal::Number(n)) => 
            *n >= min.unwrap_or(f64::NEG_INFINITY) && *n <= max.unwrap_or(f64::INFINITY),
        (QueryFilter::Range { .. }, _) => false,
        
        (QueryFilter::TimeRange { after, before }, FieldVal::Time(t)) => t >= after && t < before,
        (QueryFilter::TimeRange { .. }, _) => false,

        (QueryFilter::TimeSince { since }, FieldVal::Time(t)) => t > &(OffsetDateTime::now_utc() - since.seconds()),
        (QueryFilter::TimeSince { .. }, _) => false,
        
        (QueryFilter::KeywordIs { is }, FieldVal::String(s)) => is.contains(s),
        (QueryFilter::KeywordNot { not }, FieldVal::String(s)) => !not.contains(s),
        (QueryFilter::KeywordIs{..} | QueryFilter::KeywordNot{..}, _) => false,
    }
}

