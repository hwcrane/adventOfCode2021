mod load_data;
use load_data::*;

extern crate recap;
use recap::Recap;
use serde::Deserialize;


#[derive(Debug, Recap, Deserialize)]
#[recap(regex=r#"(?P<item1>\S+)\s(?P<item2>\S+)"#)]
pub struct Data {
    item1: String,
    item2: String
}

fn main() {
    //part1();
    part2();
}

fn get_gamma(line: &String) -> char{
    if line.matches("1").count() > line.matches("0").count() {
        return '1';
    }
    '0'
}
fn get_epsilon(line: &String) -> char{
    if line.matches("1").count() > line.matches("0").count() {
        return '0'
    }
    '1'
}

fn part1() {
    let data = read_lines_into_string("input");

    let mut gamma: Vec<String> = vec!();
    let mut epsilon: Vec<String> = vec!();

    for e in 0..data[0].len(){
        gamma.push(String::from(""));
        epsilon.push(String::from(""))
    }
    for line in data {
        let bline = line.as_bytes();
        for ch in 0..line.len(){
            gamma[ch].push(bline[ch] as char);
            epsilon[ch].push(bline[ch] as char);
        }
    }

    let mut outgam: Vec<char> = vec!();
    let mut outeps: Vec<char> = vec!();

    for i in 0..gamma.len() {
        outgam.push( get_gamma(&gamma[i]));
        outeps.push( get_epsilon(&epsilon[i]));
    }

    let gam: String = outgam.iter().collect();
    let ep: String = outeps.iter().collect();

    let gamma = u64::from_str_radix(&gam, 2).unwrap();
    let epsilon = u64::from_str_radix(&ep, 2).unwrap();

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("{}", gamma * epsilon)


}

fn get_oxegen(lines: &Vec<String>, index: usize) -> String{
    if lines.len() == 1{
        for line in lines{
            return line.to_string()
        }
    }
    let mut coloumn_at_index = String::from("");
    for line in lines{
        coloumn_at_index.push(line.chars().nth(index).unwrap())
    }

    let most_common_at_index: char = if coloumn_at_index.matches('0').count() > coloumn_at_index.matches('1').count(){
        '0'
    } else {
        '1'
    };

    let mut newVec: Vec<String> = vec!();

    for line in lines {
        if line.chars().nth(index) == Some(most_common_at_index) {
            newVec.push(line.to_string());
        }
    }

    get_oxegen(&newVec, index + 1)
    
}

fn get_CO2(lines: &Vec<String>, index: usize) -> String{
    if lines.len() == 1{
        for line in lines{
            return line.to_string()
        }
    }
    let mut coloumn_at_index = String::from("");
    for line in lines{
        coloumn_at_index.push(line.chars().nth(index).unwrap())
    }

    let most_common_at_index: char = if coloumn_at_index.matches('0').count() <= coloumn_at_index.matches('1').count(){
        '0'
    } else {
        '1'
    };

    let mut newVec: Vec<String> = vec!();

    for line in lines {
        if line.chars().nth(index) == Some(most_common_at_index) {
            newVec.push(line.to_string());
        }
    }

    get_CO2(&newVec, index + 1)
    
}

fn part2() {
    let data = read_lines_into_string("input");

    let oxygen = get_oxegen(&data, 0);
    let c02 = get_CO2(&data, 0);

    let o = u64::from_str_radix(&oxygen, 2).unwrap();
    let c = u64::from_str_radix(&c02, 2).unwrap();

    println!("{}", o * c)
}