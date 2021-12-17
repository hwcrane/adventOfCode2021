use std::time::Instant;
use std::fs;
extern crate recap;
use recap::Recap;
use serde::Deserialize;


#[derive(Debug, Recap, Deserialize)]
#[recap(regex=r#"target area: x=(?P<xmin>\S+) (?P<xmax>\S+), y=(?P<ymin>\S+) (?P<ymax>\S+)"#)]
pub struct Data {
    xmax: i32,
    xmin: i32,
    ymax: i32,
    ymin: i32
}
fn main() {
    let start = Instant::now();
    //part1();
    part2();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn get_ymax(mut vel: (i32, i32), target: &Data) -> Option<i32>{
    let mut position = (0, 0);
    let mut hit = false;
    let mut ymax = 0;
    while vel.0 != 0 || position.1 >= target.ymin{
        position.0 += vel.0;
        position.1 += vel.1;
        ymax = i32::max(ymax, position.1);
        vel.1 -= 1;
        vel.0 += if vel.0 > 0 {-1} else if vel.0 < 0 {1} else {0};
        if position.0 <= target.xmax && position.0 >= target.xmin && position.1 <= target.ymax && position.1 >= target.ymin{
            hit = true;
        }
    }
    if hit {Some(ymax)} else {None}
}

fn part1() {
    let data = fs::read_to_string("input").unwrap().parse::<Data>().unwrap();
    let mut ymax = 0;
    for x in -1000..1000 {
        for y in -1000..1000 {
            if let Some(temp) = get_ymax((x, y), &data){
                ymax = i32::max(ymax, temp);
            }
        }
    }
    println!("{:?}", ymax)
}

fn part2() {
    let data = fs::read_to_string("input").unwrap().parse::<Data>().unwrap();
    let mut count = 0;
    for x in -1000..1000 {
        for y in -1000..1000 {
            if let Some(temp) = get_ymax((x, y), &data){
                count += 1;
            }
        }
    }
    println!("{:?}", count)
}