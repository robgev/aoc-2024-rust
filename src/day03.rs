use crate::utils::to_num;
use regex::Regex;
use std::fs;

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mull_regex = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let answer = mull_regex.captures_iter(&contents).fold(0, |acc, item| {
        let number1 = to_num(item.get(1).unwrap().as_str());
        let number2 = to_num(item.get(2).unwrap().as_str());

        acc + number1 * number2
    });

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mull_regex = Regex::new(r"mul\((\d*),(\d*)\)|do\(\)|don't\(\)").unwrap();
    let mut is_enabled = true;

    let answer = mull_regex.captures_iter(&contents).fold(0, |acc, item| {
        let instruction = item.get(0).unwrap().as_str();
        if instruction == "do()" {
            is_enabled = true;
            acc
        } else if instruction == "don't()" {
            is_enabled = false;
            acc
        } else {
            let number1 = to_num(item.get(1).unwrap().as_str());
            let number2 = to_num(item.get(2).unwrap().as_str());

            acc + if is_enabled { number1 * number2 } else { 0 }
        }
    });

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
