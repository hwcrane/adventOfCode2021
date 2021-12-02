use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_lines(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No File Found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|line| line.expect("could not parse line"))
        .collect()

}