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
    let data: Vec<Vec<u32>> = fs::read_to_string("input")
        .expect("file not found")
        .lines()
        .map(|f| f.chars().map(|a| a as u32 - '0' as u32).collect())
        .collect();

    
    println!("{}, {}", data.len(), data[3].len());
    let mut count: u32 = 0;
    for x in 0..data.len(){
        for y in 0..data[0].len(){
            println!("{},{}", x, y);
            if x == 0 || data[x - 1][y] > data[x][y]
            && y == 0 || data[x][y - 1] > data[x][y]
            && x == data.len() - 1 || data[x + 1][y] > data[x][y]
            && y == data[0].len() - 1 || data[x][y + 1] > data[x][y]{
                count += 1 + data[x][y];
            }
        }
    }
    println!("{:?}",count);
}

fn part2() {
    
}