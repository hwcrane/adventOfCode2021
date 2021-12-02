use std::{
    fs,
    path::Path,
};

use crate::Data;

pub fn read_lines_into_data(filename: impl AsRef<Path>) -> Vec<Data> {
    fs::read_to_string(filename)
        .expect("File not found")
        .lines()
        .map(|s| s.parse::<Data>().unwrap())
        .collect()
}

pub fn read_lines_into_string(filename: impl AsRef<Path>) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("File not found")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}