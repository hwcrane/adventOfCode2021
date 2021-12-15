use std::time::Instant;
use std::fs;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();
    //part1();
    part2();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn get_next(open: & Vec<(i32, i32)>, costs: &HashMap<(i32, i32), i32>) -> (i32, i32){
    let mut min_cost = i32::MAX;
    let mut next_point = (-1, -1);
    for point in open{
        if costs[point] < min_cost {
            min_cost = costs[point];
            next_point = point.clone();
        }
    }
    next_point
}

fn add_point(
    point: &(i32, i32), 
    new_point:&(i32, i32),
    weights: &Vec<u32>, 
    width: u32, 
    costs: &mut HashMap<(i32, i32), i32>, 
    connections: &mut HashMap<(i32, i32), (i32, i32)>,
    neighbours: &mut  Vec<(i32, i32)>
    ){
        let cost = costs[point] + weights[(new_point.0 + new_point.1 * width as i32) as usize] as i32;

        if costs.contains_key(&new_point){
            if cost < costs[&new_point] {
                *costs.entry(new_point.clone()).or_insert(i32::MAX) = cost;
                *connections.entry(new_point.clone()).or_insert((-1, -1)) = point.clone();
            }
        } else {
            neighbours.push(new_point.clone());
            *costs.entry(new_point.clone()).or_insert(i32::MAX) = cost;
            *connections.entry(new_point.clone()).or_insert((-1, -1)) = point.clone();
        }
    }

fn get_neighbours(point: &(i32, i32), 
    weights: &Vec<u32>, 
    width: u32, 
    costs: &mut HashMap<(i32, i32), i32>, 
    connections: &mut HashMap<(i32, i32), (i32, i32)>) -> Vec<(i32, i32)>{

    let mut neighbours: Vec<(i32, i32)> = vec!();

    if point.0 > 0 {
        let new_point = (point.0 - 1, point.1);
        add_point(point, &new_point, weights, width, costs, connections, &mut neighbours)
    }
    if point.0 < weights.len() as i32 / width as i32 - 1 {
        let new_point = (point.0 + 1, point.1);
        add_point(point, &new_point, weights, width, costs, connections, &mut neighbours)
    }
    if point.1 > 0 {
        let new_point = (point.0, point.1 - 1);
        add_point(point, &new_point, weights, width, costs, connections, &mut neighbours)
    }
    if point.1 <  weights.len() as i32 / width as i32 - 1 {
        let new_point = (point.0, point.1 + 1);
        add_point(point, &new_point, weights, width, costs, connections, &mut neighbours)
    }
    neighbours
}

fn pathfind(weights: Vec<u32>, width: u32, target: (i32, i32), start: (i32, i32)) -> i32{
    let mut open: Vec<(i32, i32)> = vec![start];
    let mut closed: Vec<(i32, i32)> = vec!();
    let mut connections:HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut costs:HashMap<(i32, i32), i32> = HashMap::new();
    *costs.entry(start).or_insert(i32::MAX) = 0;
    let mut next = get_next(&open, &costs);

    while next != target {
        open.append(&mut get_neighbours(&next, &weights, width, &mut costs, &mut connections));
        let open_index = open.iter().position(|element| element == &next).unwrap();
        open.remove(open_index);
        closed.push(next);
        next = get_next(&open, &costs);
    }
    
    costs[&next]
}

fn part1() {    
    let width = 100;
    let weights: Vec<u32> = fs::read_to_string("input").expect("File not found").replace("\n", "").chars().map(|f| f as u32 - '0' as u32).collect();
    let target = (99, 99);
    let start:(i32, i32) = (0, 0);
    let shortest_path = pathfind(weights, width, target, start);
    println!("{}", shortest_path);

}   

fn clamp(num: u32) -> u32 {
    if num > 9 {(num % 10) + 1} else {num}
}
fn part2() {
    let width = 500;
    let weights: Vec<Vec<u32>> = fs::read_to_string("input").expect("File not found").lines().map(|f| f.chars().map(|f| f as u32 - '0' as u32).collect::<Vec<u32>>()).collect();

    let mut new_weights: Vec<u32> = vec!();
    for line in weights {
        for n in 0..5 {
            for bit in &line{
                new_weights.push(clamp(bit.clone() + n))
            }
        }
    }

    for n in 1..5{
        for i in 0..100 {
            for j in 0..500 {
                new_weights.push(clamp(new_weights[j + i * 500] + n))
            }
        }
    }

    let target = (499, 499);
    let start:(i32, i32) = (0, 0);
    let shortest_path = pathfind(new_weights, width, target, start);
    println!("{}", shortest_path);
}