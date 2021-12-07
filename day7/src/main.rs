mod load_data;
use std::{io::BufRead, fs};

use load_data::*;

extern crate recap;
use recap::Recap;
use serde::Deserialize;


#[derive(Debug, Recap, Deserialize)]
#[recap(regex=r#"(?P<item1>\S+)\s(?P<item2>\S+)"#)]
pub struct Data {
    item1: String,
    item2: String
}

fn main() {
    part1();
    //part2();
}


fn part1() {
    let data: Vec<u32> = fs::read_to_string("input")
        .expect("File not found")
        .trim()
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect();
    
    
    println!("{:?}", av)
}

fn part2() {
    let data = read_lines_into_data("input");

    for line in data {
        
    }
}