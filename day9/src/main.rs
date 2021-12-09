use std::time::Instant;
use std::fs;

fn main() {
    let start = Instant::now();
    part1();
    part2();
    let duration = start.elapsed();    println!("Time elapsed is: {:?}", duration);
}


fn part1() {
    let data: Vec<Vec<u32>> = fs::read_to_string("input")
        .expect("file not found")
        .lines()
        .map(|f| f.chars().map(|a| a as u32 - '0' as u32).collect())
        .collect();

    let mut count: u32 = 0;
    for x in 0..data.len(){
        for y in 0..data[0].len(){
            if data[x][y] < if x > 0 {data[x - 1][y]} else {10}
            && data[x][y] < if y > 0 {data[x][y - 1]} else {10}
            && data[x][y] < if x < data.len() - 1 {data[x + 1][y]} else {10}
            && data[x][y] < if y < data[0].len() - 1 {data[x][y + 1]} else {10}
            {
                count += 1 + data[x][y];
            }
        }
    }
    println!("{:?}",count);
}

fn visit (x: usize, y: usize, data: &mut Vec<bool>) -> u32{
    if data[x + y * 100] == true {
        return 0;
    }
    data[x + y * 100] = true;

    1 
    + if x > 0 {visit(x - 1, y, data)} else {0}
    + if y > 0 {visit(x, y - 1, data)} else {0}
    + if x < 99 {visit(x + 1, y, data)} else {0}
    + if y < 99 {visit(x, y + 1, data)} else {0}
}

fn part2() {
    let mut data: Vec<bool> = fs::read_to_string("input")
        .expect("file not found")
        .replace("\n", "")
        .chars()
        .map(|ch| if ch == '9' {true} else {false}).collect();

    let mut basins: Vec<u32> = vec!();
    for i in 0..data.len(){
        let x = i % 100;
        let y = i / 100;
        basins.push(visit(x, y, &mut data))
    }
    basins.sort_by(|a, b| b.cmp(a));
    println!("{}", basins[0] * basins[1] * basins[2])
}