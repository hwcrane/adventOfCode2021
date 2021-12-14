use std::hash::Hash;
use std::time::Instant;
use std::fs;
use std::collections::HashMap;

extern crate recap;
use recap::Recap;
use serde::Deserialize;


#[derive(Debug, Recap, Deserialize)]
#[recap(regex=r#"(?P<pattern>\S+) -> (?P<insert>\S+)"#)]
pub struct Data {
    pattern: String,
    insert: char
}


fn main() {
    let start = Instant::now();
    //part1();
    part2();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}


fn part1() {
    let data: Vec<String> = fs::read_to_string("input").unwrap().lines().map(|f| f.parse().unwrap()).collect();
    let mut template = data[0].clone();
    let patterns: Vec<Data> = data[2..].iter().map(|line| line.parse().unwrap()).collect();

    for i in 0..10 {
        let chars: Vec<char> =  template.chars().collect();
        let mut temp = String::new();
        for j in 0..(chars.len() - 1){
            temp.push(chars[j]);
            let pat_to_check = format!("{}{}", chars[j], chars[j + 1]);
            for pat in &patterns{
                if pat.pattern == pat_to_check {
                    temp.push(pat.insert)
                }
            }
        }
        temp.push(chars[chars.len() - 1]);
        
        template = temp;
        println!("step: {}", i )
    }
    let mut min = u32::MAX;
    let mut max:u32 = 0;
    for ch in 65..91{
        let count = template.matches(char::from_u32(ch).unwrap()).count() as u32;
        if count > 0 {
            min = min.min(count);
            max = max.max(count);
        }
        
    }
    println!("{}", max - min);
}

fn part2() {
    let data: Vec<String> = fs::read_to_string("input").unwrap().lines().map(|f| f.parse().unwrap()).collect();
    let template: Vec<char> = data[0].clone().chars().collect();
    let patterns: Vec<Data> = data[2..].iter().map(|line| line.parse().unwrap()).collect();
    let mut rules= HashMap::<String, char>::new();
    let mut pairs = HashMap::<String, u128>::new();

    for pattern in patterns{
        rules.insert(pattern.pattern, pattern.insert);
    }
    for i  in 0..template.len() - 1 {
        *pairs.entry(format!("{}{}", template[i], template[i + 1])).or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut new_pairs = HashMap::<String, u128>::new();
        for (pair, value) in pairs.clone(){
            if let Some(ch) = rules.get(&pair){
                *new_pairs.entry(format!("{}{}", pair.chars().collect::<Vec<char>>()[0], ch)).or_insert(0) += value;
                *new_pairs.entry(format!("{}{}", ch, pair.chars().collect::<Vec<char>>()[1])).or_insert(0) += value;
            } 
        }
        pairs = new_pairs
    }

    let mut chars = HashMap::<char, u128>::new();
    for pair in pairs {
        for ch in pair.0.chars().collect::<Vec<char>>(){
            *chars.entry(ch).or_insert(0) += pair.1
        }
    }

    let mut min = u128::MAX;
    let mut max:u128 = 0;
    for ch in chars{
        if ch.1 > 0 {
            min = min.min(ch.1);
            max = max.max(ch.1);
        }
        
    }
    println!("{}", (max + 1) / 2 - (min + 1) / 2);
}