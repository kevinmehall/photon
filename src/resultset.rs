use std::{iter, slice};

use serde::Serialize;

pub struct ResultSet {
    cols: Vec<String>,
    ptrs: Vec<usize>,
    buf: String,
}

impl ResultSet {
    pub fn new(cols: Vec<String>) -> Self {
        Self { cols, ptrs: Vec::new(), buf: String::new() }
    }

    pub fn push(&mut self, s: &str) {
        self.buf.push_str(s);
        self.ptrs.push(self.buf.len())
    }

    pub fn end_row(&mut self) {
        if !self.cols.is_empty() {
            assert_eq!(self.ptrs.len() % self.cols.len(), 0);
        }
    }

    pub fn len(&self) -> usize {
        self.ptrs.len() / self.cols.len()
    }

    pub fn cols(&self) -> impl Iterator<Item = &str> {
        self.cols.iter().map(|x| &x[..])
    }

    pub fn rows(&self) -> ResultSetIter {
        ResultSetIter { cols: &self.cols[..], ptrs: &self.ptrs[..], buf: &self.buf[..], pos: 0 }
    }
}

#[derive(Clone)]
pub struct ResultSetIter<'a> {
    cols: &'a [String],
    ptrs: &'a [usize],
    buf: &'a str,
    pos: usize,
}

impl<'a> Iterator for ResultSetIter<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.cols.is_empty() && self.ptrs.len() >= self.cols.len() {
            let pos = self.pos;
            let (row, rest) = self.ptrs.split_at(self.cols.len());
            self.pos = row.last().copied().unwrap_or(0);
            self.ptrs = rest;
            Some(Row {
                pos,
                npos: row.iter(),
                buf: &self.buf,
                cols: self.cols,
            })
        } else { None }
    }
}

#[derive(Clone)]
pub struct Row<'a> {
    pos: usize,
    npos: slice::Iter<'a, usize>,
    buf: &'a str,
    cols: &'a [String],
}

impl<'a> Row<'a> {
    pub fn with_col_names(self) -> impl Iterator<Item=(&'a str, &'a str)> {
        let cols = self.cols.iter().map(|x| &x[..]);
        iter::zip(cols, self)
    }
}

impl<'a> Iterator for Row<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.npos.next().map(|&end| {
            let start = self.pos;
            self.pos = end;
            &self.buf[start..end]
        })
    }
}


impl Serialize for ResultSet {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        s.collect_seq(self.rows())
    }
}

impl<'a> Serialize for Row<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        s.collect_map(self.clone().with_col_names())
    }
}

#[test]
fn test() {
    let mut rs = ResultSet::new(vec!["foo".to_owned(), "bar".to_owned()]);
    assert_eq!(rs.len(), 0);

    rs.push("abcdefg");
    rs.push("qw");
    rs.end_row();
    assert_eq!(rs.len(), 1);

    rs.push("c123");
    rs.push("d456");
    rs.end_row();
    assert_eq!(rs.len(), 2);

    assert_eq!(rs.rows().map(|row| row.collect::<Vec<_>>()).collect::<Vec<Vec<&str>>>(), vec![vec!["abcdefg", "qw"], vec!["c123", "d456"]]);

    println!("{}", serde_json::to_string(&rs).unwrap());
    assert_eq!(serde_json::to_string(&rs).unwrap(), r#"[{"foo":"abcdefg","bar":"qw"},{"foo":"c123","bar":"d456"}]"#);
}
