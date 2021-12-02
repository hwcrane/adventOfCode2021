use std::fs;

fn main() {
    part2()
}

fn part2(){
    let file = fs::read_to_string("input").expect("file not found");
    let lines = file.lines();
    let mut position = (0, 0, 0);

    for line in lines {
        let splitline: Vec<&str> = line.split(" ").collect();
        let number = splitline[1].parse::<i32>().unwrap();

    match splitline[0]{
        "forward" => {
            position.0 += number;
            position.1 += position.2 * number;
        } 
        "down" => position.2 += number,
        "up" => position.2 -= number,
        &_ => println!("{}", splitline[0])
    }
    
    }
    println!("{}", position.0 * position.1)

}
fn part1(){
    let file = fs::read_to_string("input").expect("file not found");
    let lines = file.lines();
    let mut position = (0, 0);

    for line in lines{
        let splitline: Vec<&str> = line.split(" ").collect();

        let number = splitline[1].parse::<i32>().unwrap();
        match splitline[0] {
            "forward" => position.0 += number,
            "down" => position.1 += number,
            "up" => position.1 -= number,
            &_ => println!("{}", splitline[0])
        }
    }

    println!("{}", position.1 * position.0)
}
