use std::fs;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    part1();
    part2();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}


fn part1() {
    let data = fs::read_to_string("input")
        .expect("File not found")
        .lines()
        .map(|a| String::from(a))
        .collect::<Vec<String>>();

    let mut num = 0;
    for line in data {
        let linedata = &line.split("|").map(|f| String::from(f)).collect::<Vec<String>>()[1];
        let letters: Vec<String> = linedata.trim().split(" ").map(|f| String::from(f)).collect();
        for letter in letters{
            match letter.len() {
                2 | 4 | 3 | 7 => num += 1,
                _ => continue,
            }
        }
    }

    println!("{}", num);
}

fn string_to_hashset(string: String) -> HashSet<char>{
    let mut set = HashSet::new();
    for letter in string.chars(){ set.insert(letter); }
    set
}

fn get_number(num: &HashSet<char>, one: &HashSet<char>, four: &HashSet<char>) -> u32{
    match num.len() {
        2 => 1, 3 => 7, 4 => 4, 7 => 8,
        5 => {
            if num.intersection(&four).count() == 2 { 2 } 
            else if num.intersection(&one).count() == 2 { 3 } 
            else { 5 }
        }
        6 => {
            if num.intersection(&four).count() == 4 { 9 } 
            else if num.intersection(&one).count() == 2{ 0 }
            else { 6 }
        }
        _ => 10
    }
}

fn part2() {
    let data = fs::read_to_string("input")
        .expect("File not found")
        .lines()
        .map(|a| String::from(a))
        .collect::<Vec<String>>();

    let mut count = 0;
    
    for line in data {
        let linedata = &line.split("|").map(|f| String::from(f)).collect::<Vec<String>>();
        let numbers: Vec<HashSet<char>> = linedata[0]
            .trim()
            .split(" ")
            .map(|f| string_to_hashset(String::from(f)))
            .collect();

        let mut one= HashSet::new();
        let mut four= HashSet::new();

        for n in numbers{
            if n.len() == 2 {
                one = n;
            } else if n.len() == 4 {
                four = n;
            }
        }

        let code:  Vec<HashSet<char>> = linedata[1]
            .trim()
            .split(" ")
            .map(|f| string_to_hashset(String::from(f)))
            .collect();
        
        count += 1000 * get_number(&code[0], &one, &four) 
                + 100 * get_number(&code[1], &one, &four) 
                + 10 * get_number(&code[2], &one, &four) 
                + get_number(&code[3], &one, &four);
    }
    println!("{}", count)
}