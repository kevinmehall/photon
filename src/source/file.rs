use std::{fs::File, io::{BufRead, BufReader}};
use bumpalo::Bump;
use bumpalo::collections::String as BString;
use bumpalo::collections::Vec as BVec;

use crate::{query::{QueryPlan, QueryError, FieldVal}, ResultSet, filter::filter_test, FieldDefaults, api::fields::FieldType};

use super::Source;

pub(crate) struct FileLines {
    glob_pattern: glob::Pattern,
}

impl FileLines {
    pub(crate) fn new(path_glob: &str) -> Result<FileLines, &'static str> {
        Ok(Self { glob_pattern: glob::Pattern::new(path_glob).map_err(|x| x.msg)? })
    }
}

impl Source for FileLines {
    fn query(&self, plan: QueryPlan) -> Result<ResultSet, QueryError> {
        let mut files = glob::glob(self.glob_pattern.as_str())
            .unwrap() // Pattern is already checked, but `glob` provides no API to avoid re-parsing the `Pattern`
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        
        files.sort_by(|a, b|
            natord::compare(&a.to_string_lossy(), &b.to_string_lossy()).reverse()
        );
       
        let mut rows_scanned = 0;
        let mut results = ResultSet::new(plan.returning.keys().map(|n| n.to_string()).collect());

        for fname in files {
            let mut file = BufReader::new(File::open(&fname)?);
            let fname_str = fname.to_string_lossy();

            let b = file.fill_buf()?;
            if b.starts_with(&[0x1f, 0x8b]) {
                let reader = BufReader::new(flate2::bufread::GzDecoder::new(file));
                read_lines(&fname_str, reader, &plan, &mut results, &mut rows_scanned)?;
            } else {
                read_lines(&fname_str, file, &plan, &mut results, &mut rows_scanned)?;
            }
        }
        
       Ok(results)
    }

    fn fields(&self) -> Vec<(&str, FieldDefaults)> {
        vec![
            ("filename", FieldDefaults { ty: FieldType::Keyword }),
            ("line",     FieldDefaults { ty: FieldType::Number }),
            ("offset",   FieldDefaults { ty: FieldType::Number }),
        ]
    }
}

fn read_lines(fname: &str, mut file: impl BufRead, plan: &QueryPlan, results: &mut ResultSet, rows_scanned: &mut i32) -> Result<(), QueryError> {
    let mut bump = Bump::new();
    bump.set_allocation_limit(Some(16 * 1024 * 1024));
    let mut buf = Vec::new();
    let mut pos = 0;
    'line: loop {
        *rows_scanned += 1;
    
        buf.clear();
        let read_size = file.read_until(b'\n', &mut buf)?;
        if read_size == 0 { break; }

        bump.reset();
    
        let mut root_data = BVec::new_in(&bump);
    
        for &field in &plan.root_fields {
            let v = match field {
                "filename" => FieldVal::String(fname),
                "line" => FieldVal::String(
                    std::str::from_utf8(&buf).unwrap_or_else(|_|{
                        BString::from_utf8_lossy_in(&*buf, &bump).into_bump_str()
                    }).trim_end_matches('\n')
                ),
                "offset" => FieldVal::Number(pos as f64),
                _ => FieldVal::Null,
            };
    
            root_data.push(v);
        }
    
        let mut data = BVec::new_in(&bump);
        data.push(&mut root_data[..]);
    
        for parser in plan.parsers.values() {
            let vals = parser.parser.parse(&bump, &mut data[parser.src.parser][parser.src.field]);
            data.push(vals);
        }
    
        for (loc, filter) in &plan.filters {
            if !filter_test(filter, &data[loc.parser][loc.field]) {
                continue 'line;
            }
        }
    
        for loc in plan.returning.values() {
            results.push_fmt(&data[loc.parser][loc.field]);
        }
        results.end_row();
        pos += read_size;
    }
    Ok(())
}
