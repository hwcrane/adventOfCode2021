mod load_data;
use load_data::*;

extern crate recap;
use recap::Recap;
use serde::Deserialize;
use std::{
    fs,
    path::Path,
};

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
    let fish: Vec<u32> = fs::read_to_string("input")
        .expect("File not found")
        .trim()
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect();
    
    let mut num_of_fish = [0;9];
    for f in fish{
        num_of_fish[f as usize] += 1;
    }
    for _ in 0..256{
        let fish_to_grow = num_of_fish[0];
        for j in 0..8{
            num_of_fish[j] = num_of_fish[j + 1];
        }    
        num_of_fish[8] = fish_to_grow;
        num_of_fish[6] += fish_to_grow;
    }

    let sum: u32 = num_of_fish.iter().map(|f|f).sum();
        
    println!("{:?}", sum);

}

fn part2() {
    let fish: Vec<u32> = fs::read_to_string("input")
        .expect("File not found")
        .trim()
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect();
    
    let mut num_of_fish:[u128; 9] = [0;9];
    for f in fish{
        num_of_fish[f as usize] += 1;
    }
    for _ in 0..256{
        let fish_to_grow = num_of_fish[0];
        for j in 0..8{
            num_of_fish[j] = num_of_fish[j + 1];
        }    
        num_of_fish[8] = fish_to_grow;
        num_of_fish[6] += fish_to_grow;
    }

    let sum: u128 = num_of_fish.iter().map(|f|f).sum();
        
    println!("{:?}", sum);
}