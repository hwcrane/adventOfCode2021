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
    part1();
    part2();
}

fn has_won(table: &Vec<String>) -> bool {
    let mut won = false;
    for i in 0..5{
        if table[5 * i] == "x" &&
            table[5 * i + 1] == "x" &&
            table[5 * i + 2] == "x" &&
            table[5 * i + 3] == "x" &&
            table[5 * i + 4] == "x" ||
            table[i] == "x" &&
            table[i + 5] == "x" &&
            table[i + 10] == "x" &&
            table[i + 15] == "x" &&
            table[i + 20] == "x" {
                won = true
            }
    }
    won
}

fn part1() {
    let data = read_lines_into_string("input");

    let random_nums: Vec<String> = data[0].split(",").map(|x| x.parse().unwrap()).collect();

    let lines: Vec<Vec<String>> = data.iter()
        .skip(2)
        .map(|x| x.replace("  ", " ")
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect())
        .collect();

    let mut bingo_tables: Vec<Vec<String>> = vec!();

    let mut table_num = 0;
    for line in &lines{
        if bingo_tables.len() <= table_num {
            bingo_tables.push(line.clone());
            continue;
        }

        if line.len() == 1 {
            table_num += 1;
            continue;
        }

        bingo_tables[table_num].append(&mut line.clone());
    }
    
    for n in random_nums{
        for table in &mut bingo_tables{
            let index = table.iter().position(|x| x == &n);

            if let Some(x) = index{
                table[x] = String::from("x");
            }

            if has_won(table) {
                let without_cross: Vec<&String> = table.iter()
                .map(|x| x)
                .filter(|y| y != &&String::from("x"))
                .collect();

                let sum: u32 = without_cross.iter().map(|x| x.parse::<u32>().unwrap()).sum();


                println!("{:?}", sum * n.parse::<u32>().unwrap());
                return
            }
        }
        
    }

}

fn part2() {
    let data = read_lines_into_string("input");

    let random_nums: Vec<String> = data[0].split(",").map(|x| x.parse().unwrap()).collect();

    let lines: Vec<Vec<String>> = data.iter()
        .skip(2)
        .map(|x| x.replace("  ", " ").trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect())
        .collect();

    let mut bingo_tables: Vec<Vec<String>> = vec!();

    let mut table_num = 0;
    for line in &lines{
        if bingo_tables.len() == table_num {
            bingo_tables.push(line.clone());
            continue;
        }

        if line.len() == 1 {
            table_num += 1;
            continue;
        }

        bingo_tables[table_num].append(&mut line.clone());
    }

    let mut score: u32 = 0;

    let mut winning_tables: Vec<u32> = vec!();

    for n in random_nums{
        for i in 0..bingo_tables.len(){

            let index = bingo_tables[i].iter().position(|x| x == &n);

            if let Some(x) = index {
                bingo_tables[i][x] = String::from("x");
            }

            if has_won(&bingo_tables[i]) && winning_tables.iter().position(|x| x == &(i as u32)) == None{
                let without_cross: Vec<&String> = bingo_tables[i].iter()
                .map(|x| x)
                .filter(|y| y != &&String::from("x"))
                .collect();

                score = without_cross.iter().map(|x| x.parse::<u32>().expect("")).sum::<u32>() * n.parse::<u32>().unwrap();                
                winning_tables.push(i as u32);
                
            }
        }
    }
    println!("{}", score)
}