use std::fmt;

use bumpalo::Bump;
use bumpalo::collections::{Vec as BVec, CollectIn};
use indexmap::IndexSet;
use serde::de::DeserializeSeed;

use crate::query::FieldVal;

use super::ParserInst;

pub(crate) struct Json {
    fields: IndexSet<String>,
}

impl Json {
    pub(crate) fn new() -> Self {
        Self { fields: IndexSet::new() }
    }
}

struct StrSeed<'b>(&'b Bump);

impl <'de, 'b: 'de> DeserializeSeed<'de> for StrSeed<'b> {
    type Value = &'b str;

    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<&'b str, D::Error> {
        struct Visitor<'b>(&'b Bump);

        impl<'de, 'b> serde::de::Visitor<'de> for Visitor<'b> {
            type Value = &'b str;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string")
            }

            fn visit_str<E>(self, v: &str) -> Result<&'b str, E> {
                Ok(self.0.alloc_str(v))
            }
        }

        deserializer.deserialize_str(Visitor(self.0))
    }
}

struct Seed<'b>(&'b Bump);

impl <'de, 'b: 'de> DeserializeSeed<'de> for Seed<'b> {
    type Value = FieldVal<'b>;

    fn deserialize<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<FieldVal<'b>, D::Error> {
        struct Visitor<'b>(&'b Bump);

        impl<'de, 'b: 'de> serde::de::Visitor<'de> for Visitor<'b> {
            type Value = FieldVal<'b>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid JSON value")
            }

            fn visit_bool<E>(self, v: bool) -> Result<FieldVal<'b>, E> {
                Ok(if v { FieldVal::String("true") } else { FieldVal::String("false") })
            }

            fn visit_i64<E>(self, v: i64) -> Result<FieldVal<'b>, E> {
                Ok(FieldVal::Number(v as f64))
            }

            fn visit_u64<E>(self, v: u64) -> Result<FieldVal<'b>, E> {
                 Ok(FieldVal::Number(v as f64))
            }

            fn visit_f64<E>(self, v: f64) -> Result<FieldVal<'b>, E> {
                 Ok(FieldVal::Number(v))
            }

            fn visit_str<E>(self, v: &str) -> Result<FieldVal<'b>, E> {
                Ok(FieldVal::String(self.0.alloc_str(v)))
            }

            fn visit_none<E>(self) -> Result<FieldVal<'b>, E> {
                Ok(FieldVal::Null)
            }

            fn visit_unit<E>(self) -> Result<FieldVal<'b>, E> {
                Ok(FieldVal::Null)
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                while let Some(serde::de::IgnoredAny) = seq.next_element()? {}
                Ok(FieldVal::Null)
            }

            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut access: A) -> Result<FieldVal<'b>, A::Error> {
                let mut map = BVec::with_capacity_in(access.size_hint().unwrap_or(16), self.0);

                while let Some((key, value)) = access.next_entry_seed(StrSeed(self.0), Seed(self.0))? {
                    map.push((key, value));
                }

                Ok(FieldVal::Map(map.into_bump_slice()))
            }
        }

        deserializer.deserialize_any(Visitor(self.0))
    }
}

impl ParserInst for Json {
    fn require_field(&mut self, field: &str) -> Option<usize> {
        Some(self.fields.insert_full(field.to_owned()).0)
    }

    fn parse<'b>(&self, bump: &'b bumpalo::Bump, input: &mut FieldVal<'b>) -> &'b mut [FieldVal<'b>] {
        let v = match input {
            FieldVal::String(s) => parse(bump, s),
            v => *v,
        };

        self.fields.iter().map(|key| {
            deref(v, key)
        }).collect_in::<BVec<_>>(bump).into_bump_slice_mut()
    }
}

fn parse<'b>(bump: &'b Bump, s: &str) -> FieldVal<'b> {
     Seed(bump).deserialize(&mut serde_json::Deserializer::from_str(s)).unwrap_or(FieldVal::Null)
}

fn deref<'b>(mut v: FieldVal<'b>, path: &str) -> FieldVal<'b> {
    for path_part in path.split(".") {
        if let FieldVal::Map(pairs) = v {
            if let Some((_, child)) = pairs.iter().find(|(k, _)| *k == path_part) {
                v = *child;
            } else {
                return FieldVal::Null;
            }
        } else {
            return FieldVal::Null
        }
    }
    v
}

#[test]
fn test_parse_line() {
    let bump = Bump::new();
    let v = parse(&bump, r#"{"ignored": {}, "foo": 5, "bar": {"baz": "test"}, "dotted.name": 6, "obj": {}, "arr": [5], "bool": false, "n": null}"#);

    assert_eq!(deref(v, "foo"), FieldVal::Number(5.0));
    assert_eq!(deref(v, "bar.baz"), FieldVal::String("test"));
    assert_eq!(deref(v, "x"), FieldVal::Null);
    assert_eq!(deref(v, "n"), FieldVal::Null);
    assert_eq!(deref(v, "arr"), FieldVal::Null);
    assert_eq!(deref(v, "bool"), FieldVal::String("false"));
}

