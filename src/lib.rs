use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use std::ffi::OsStr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("hi");
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
pub struct Line {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

impl Line {
    pub fn new(x1: isize, y1: isize, x2: isize, y2: isize) -> Line {
        Line { x1, y1, x2, y2 }
    }
}

struct DXFObject {
    lines: Vec<Line>,
}

pub fn readDXF(path: &Path) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
}

fn next_coord(mut it: impl Iterator<Item = String>) -> isize {
    it.skip(1).next().unwrap().parse::<f64>().unwrap() as _
}

pub fn parse(path: &Path) -> Vec<Line> {
    let mut lines = readDXF(path).map(Result::unwrap);
    let mut lines_store: Vec<Line> = Vec::new();

    while let Some(line) = lines.find(|line| line == "AcDbLine") {
        println!("Line Found");

        let x1 = next_coord(&mut lines);
        let y1 = next_coord(&mut lines);
        let x2 = next_coord(&mut lines);
        let y2 = next_coord(&mut lines);

        lines_store.push(Line::new(x1, y1, x2, y2));
    }

    lines_store
}
