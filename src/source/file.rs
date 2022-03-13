use std::{fs::File, io::{BufRead, BufReader, Seek}, path::{PathBuf}, mem};
use crate::{query::{QueryPlan, QueryError, FieldVal}, ResultSet, filter::filter_test};

use super::Source;

pub(crate) struct FileLines {
    fname: PathBuf,
}

impl FileLines {
    pub(crate) fn new(fname: &str) -> Self {
        Self { fname: PathBuf::from(fname) }
    }
}

impl Source for FileLines {
    fn query(&self, plan: QueryPlan) -> Result<ResultSet, QueryError> {
        let mut file = BufReader::new(File::open(&self.fname)?);
        let mut buf = Vec::new();

        let mut rows_scanned = 0;

        let mut results = ResultSet::new(plan.returning.keys().map(|n| n.to_string()).collect());

        'line: loop {
            rows_scanned += 1;
            
            buf.clear();
            let pos = file.stream_position()?;
            let read_size = file.read_until(b'\n', &mut buf)?;
            if read_size == 0 { break; }

            let mut root_data = Vec::new();

            for &field in &plan.root_fields {
                let v = match field {
                    "line" => FieldVal::String(String::from_utf8_lossy(&buf).trim_end_matches('\n').to_string()),
                    "offset" => FieldVal::Number(pos as f64),
                    _ => FieldVal::Null,
                };

                root_data.push(v);
            }

            let mut data = Vec::new();
            data.push(root_data);

            for parser in plan.parsers.values() {
                data.push(parser.parser.parse(&String::from(data[parser.src.parser][parser.src.field].clone())))
            }

            for (loc, filter) in &plan.filters {
                if !filter_test(filter, &data[loc.parser][loc.field]) {
                    continue 'line;
                }
            }

            for loc in plan.returning.values() {
                results.push(&String::from(mem::replace(&mut data[loc.parser][loc.field], FieldVal::Null)));
            }
            results.end_row();
       }

       Ok(results)
    }

    fn fields(&self) -> Box<dyn Iterator<Item = (String, crate::api::fields::Field)>> {
        Box::new(["line", "offset"].iter().map(|x| (x.to_string(), crate::api::fields::Field {} )))
    }
}
