use std::fs;
use std::collections::HashSet;

fn main() {
    //part1();
    part2();
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

fn part2() {
    let data = fs::read_to_string("input")
        .expect("File not found")
        .lines()
        .map(|a| String::from(a))
        .collect::<Vec<String>>();

    for line in data {
        let linedata = &line.split("|").map(|f| String::from(f)).collect::<Vec<String>>();
        let numbers: Vec<String> = linedata[0].trim().split(" ").map(|f| String::from(f)).collect();
        for n in numbers{
            
        }
    }
}