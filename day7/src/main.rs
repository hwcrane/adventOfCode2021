mod load_data;
use std::{ fs, clone };

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
    //part1();
    part2();
}


fn part1() {
    let data: Vec<i32> = fs::read_to_string("input")
        .expect("File not found")
        .trim()
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect();
    
    let max = &data.iter().max().unwrap().clone();
    let min = &data.iter().min().unwrap().clone();

    let mut lowest_cost = i32::MAX;
    for i in min.clone()..max.clone() {
        let mut cost = 0;
        for point in &data{
            cost += ( point - i).abs();
        }
        if cost < lowest_cost{
            lowest_cost = cost;
        } 
    }
    println!("{:?}", lowest_cost)
}

fn part2() {
    let data: Vec<i64> = fs::read_to_string("input")
        .expect("File not found")
        .trim()
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect();
    
    let max = &data.iter().max().unwrap().clone();
    let min = &data.iter().min().unwrap().clone();

    let mut lowest_cost = i64::MAX;
    for i in min.clone()..(max.clone() + 1) {
        let mut cost = 0;
        for point in &data{
            let n = ( point - i).abs();
            cost += ( n * (n + 1) ) / 2;
        }
        if cost < lowest_cost{
            lowest_cost = cost;
        } 
    }
    println!("{:?}", lowest_cost)
}