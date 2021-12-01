use std::fs;

fn main() {
    part1();
    part2()
}

fn part1(){
    let file = fs::read_to_string("input.txt").expect("file not found");
    let lines = file.lines();

    let mut prev_num = u32::MAX;
    let mut num_of_increments: u32 = 0;
    for i in lines{
        let line_as_int = i.parse::<u32>().unwrap();
        if line_as_int > prev_num {
            num_of_increments+=1;            
        } 
        prev_num = line_as_int;
    }

    println!("{}", num_of_increments);
}

fn part2(){
    let file = fs::read_to_string("input.txt").expect("file not found");
    let lines: Vec<u32> = file.lines().map(|s| s.parse::<u32>().unwrap()).collect();

    let mut prev_sum = u32::MAX;
    let mut num_of_increments: u32 = 0;

    for i in 0..lines.len() - 2 {
        if lines[i] + lines[i+1] + lines[i+2] > prev_sum {
            num_of_increments +=1
        }
        prev_sum = lines[i] + lines[i+1] + lines[i+2];
    }

    println!("{}", num_of_increments)
}
