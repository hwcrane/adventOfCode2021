use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
    part1();
    part2();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}


fn part1() {
    let lines = fs::read_to_string("input").expect("File not found")
        .lines().map(|line| line.chars()
            .map(|ch| ch)
            .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    let mut score = 0;
    for line in lines {
        let mut stack: Vec<char> = vec!();

        for ch in line{
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' => {
                    let popped = stack.pop().unwrap();
                    if popped != '(' {
                        score += 3;
                        break;
                    } 
                },
                ']' => {
                    let popped = stack.pop().unwrap();
                    if popped != '[' {
                        score += 57;
                        break;
                    } 
                },
                '}' => {
                    let popped = stack.pop().unwrap();
                    if popped != '{' {
                        score += 1197;
                        break;
                    } 
                },
                '>' => {
                    let popped = stack.pop().unwrap();
                    if popped != '<' {
                        score += 25137;
                        break;
                    } 
                }
                _ => panic!("{}", ch)
            }
        }
    }
    println!("{}", score);
}

fn part2() {
    let lines = fs::read_to_string("input").expect("File not found")
        .lines().map(|line| line.chars()
            .map(|ch| ch)
            .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    let mut scores: Vec<u64> = vec!();

    for line in lines {
        let mut stack: Vec<char> = vec!();
        let mut incomplete = false;
        for ch in line{
            match ch {
                '(' | '[' | '{' | '<' => stack.push(ch),
                ')' => {
                    let popped = stack.pop().unwrap();
                    if popped != '(' {
                        incomplete = true;
                        break;
                    } 
                },
                ']' => {
                    let popped = stack.pop().unwrap();
                    if popped != '[' {
                        incomplete = true;
                        break;
                    } 
                },
                '}' => {
                    let popped = stack.pop().unwrap();
                    if popped != '{' {
                        incomplete = true;
                        break;
                    } 
                },
                '>' => {
                    let popped = stack.pop().unwrap();
                    if popped != '<' {
                        incomplete = true;
                        break;
                    } 
                }
                _ => panic!("{}", ch)
            }
        }
        if incomplete {continue;}

        let mut score:u64 = 0;

        stack.reverse();

        for ch in stack{
            score *= 5;
            score += match ch {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0
            }
        }
        scores.push(score)
    }
    scores.sort();

    println!("{}", scores[(scores.len() / 2)]);
}