use crate::utils::to_num;
use regex::Regex;
use std::{fs, usize};

fn extract_number(contents: &Vec<char>, char_loc: &mut usize) -> i32 {
    let mut number = 0;
    while contents[*char_loc].is_digit(10) {
        number = 10 * number + contents[*char_loc].to_digit(10).unwrap();
        *char_loc += 1;
    }

    number as i32
}

fn extract_mult(contents: &Vec<char>, char_loc: &mut usize) -> i32 {
    let mul_start: String = contents[*char_loc..*char_loc + "mul(".len()]
        .iter()
        .collect();
    // Validate the token
    if mul_start == "mul(" {
        // Move the pointer to the start reading the number
        *char_loc += "mul(".len();
        let number1 = extract_number(&contents, char_loc);
        // After number the location pointer should be at ,
        if contents[*char_loc] == ',' {
            *char_loc += 1;
            // Extract the second number after comma
            let number2 = extract_number(&contents, char_loc);
            // Check if it ends with a closing brace
            if contents[*char_loc] == ')' {
                return number1 * number2;
            }
        }
    }

    0
}

fn solve_part_1() {
    let input = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let contents: Vec<char> = input.chars().collect();
    let mut char_loc = 0;
    let mut answer = 0;

    while char_loc < contents.len() {
        let ch = contents[char_loc];
        if ch == 'm' {
            answer += extract_mult(&contents, &mut char_loc);
        }

        char_loc += 1;
    }

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let input = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let contents: Vec<char> = input.chars().collect();
    let mut char_loc = 0;
    let mut answer = 0;
    let mut is_enabled = true;

    while char_loc < contents.len() {
        let ch = contents[char_loc];
        if ch == 'm' {
            answer += if is_enabled {
                extract_mult(&contents, &mut char_loc)
            } else {
                0
            }
        } else if ch == 'd' {
            let is_do = contents[char_loc..char_loc + "do()".len()]
                .iter()
                .collect::<String>()
                == "do()";
            let is_dont = contents[char_loc..char_loc + "don't()".len()]
                .iter()
                .collect::<String>()
                == "don't()";
            if is_do {
                char_loc += "do()".len() - 1;
                is_enabled = true;
            }

            if is_dont {
                char_loc += "don't()".len() - 1;
                is_enabled = false;
            }
        }

        char_loc += 1;
    }

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
