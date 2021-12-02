mod load_data;
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
    let data = read_lines_into_data("input");

    for line in data {

    }
}

fn part2() {
    let data = read_lines_into_data("input");

    for line in data {
        
    }
}