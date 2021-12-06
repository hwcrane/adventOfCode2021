mod load_data;
use load_data::*;
use std::collections::HashMap;
extern crate recap;
use recap::Recap;
use serde::Deserialize;


#[derive(Debug, Recap, Deserialize, Clone, Copy)]
#[recap(regex=r#"(?P<xstart>\S+),(?P<ystart>\S+) -> (?P<xend>\S+),(?P<yend>\S+)"#)]
pub struct Data {
    xstart: u32,
    ystart: u32,
    xend: u32,
    yend: u32
}

fn main() {
    part1();
    part2();
}

fn get_cordinates(data: &&Data) -> Vec<(u32, u32)>{

    let mut points: Vec<(u32, u32)> = vec!();
    // if horesontal
    if data.xstart == data.xend{
        let ystart = u32::min(data.ystart, data.yend);
        let yend = u32::max(data.ystart, data.yend);
        for y in ystart..(yend + 1){
            points.push((data.xstart, y))
        }
    }

    // if vertical
    if data.ystart == data.yend{
        let xstart = u32::min(data.xstart, data.xend);
        let xend = u32::max(data.xstart, data.xend);
        for x in xstart..(xend + 1){
            points.push((x, data.ystart))
        }
    }

    points
}
fn part1() {
    let data = read_lines_into_data("input");
    let filtered_data: Vec<&Data> = data.iter()
        .map(|f| f)
        .filter(|f| f.xstart == f.xend || f.ystart == f.yend)
        .collect();

    let mut grid = [0; 1000000];

    for data in &filtered_data{
        let points = get_cordinates(data);
        for point in points{
            grid[(point.0 + 1000 * point.1) as usize] += 1;
        }
    }

    let count = grid.iter()
        .map(|x| x)
        .filter(|x| x.clone() >= &2)
        .count();
    println!("{}",count)
    
        
}

fn get_cordinates2(data: Data) -> Vec<(u32, u32)>{

    let mut points: Vec<(u32, u32)> = vec!();
    
    let direction: (i32, i32) = 
    (if data.xstart == data.xend {0} else if data.xstart > data.xend {-1} else {1}, 
        if data.ystart == data.yend {0} else if data.ystart > data.yend {-1} else {1});
    
    let mut x: i32 = data.xstart as i32;
    let mut y: i32 = data.ystart as i32;

    while (x, y) != (data.xend as i32, data.yend as i32) {
        points.push((x.try_into().unwrap(), y.try_into().unwrap()));
        x += direction.0;
        y += direction.1;
    }
    points.push((x.try_into().unwrap(), y.try_into().unwrap()));
    
    points
}

fn part2() {    
    let mut grid = [0; 1000000];

    let data = read_lines_into_data("input");


    for data in data{
        let points = get_cordinates2(data);
        for point in points{
            grid[(point.0 + 1000 * point.1) as usize] += 1;
        }
    }

    let count = grid.iter()
        .map(|x| x)
        .filter(|x| x.clone() >= &2)
        .count();
    println!("{}",count)
}