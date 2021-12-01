use std::fs;

fn main() {
    part1();
    part2()
}

fn part1(){
    let lines: Vec<u32> = fs::read_to_string("input.txt").expect("file not found").lines().map(|s| s.parse::<u32>().unwrap()).collect();

    let mut num_of_increments: u32 = 0;
    for i in 1..lines.len(){
        if lines[i] > lines[i - 1] {
            num_of_increments+=1;            
        } 
    }

    println!("{}", num_of_increments);
}

fn part2(){
    let lines: Vec<u32> = fs::read_to_string("input.txt").expect("file not found").lines().map(|s| s.parse::<u32>().unwrap()).collect();

    let mut num_of_increments: u32 = 0;

    for i in 1..lines.len() - 2 {
        if lines[i] + lines[i+1] + lines[i+2] > lines[i - 1] + lines[i] + lines[i+1] {
            num_of_increments +=1
        }
    }

    println!("{}", num_of_increments)
}
