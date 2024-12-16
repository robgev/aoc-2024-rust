use crate::utils::to_num;
use core::f64;
use std::{fs, i64, usize};

fn parse_button(line: &str, button: &str) -> (f64, f64) {
    let button_str = format!("Button {}: X+", button);
    let button_x_start = line.find(&button_str).unwrap() + button_str.len();
    let button_x_end = line.find(",").unwrap();
    let button_y_start = line.find("Y+").unwrap() + "Y+".len();
    let button_y_end = line.trim().len();

    let button_x = to_num(&line[button_x_start..button_x_end]) as f64;
    let button_y = to_num(&line[button_y_start..button_y_end]) as f64;

    (button_x, button_y)
}

fn parse_prize(line: &str) -> (f64, f64) {
    let prize_str = "Prize: X=";
    let prize_x_start = line.find(prize_str).unwrap() + prize_str.len();
    let prize_x_end = line.find(",").unwrap();
    let prize_y_start = line.find("Y=").unwrap() + "Y+".len();
    let prize_y_end = line.trim().len();

    let prize_x = to_num(&line[prize_x_start..prize_x_end]) as f64;
    let prize_y = to_num(&line[prize_y_start..prize_y_end]) as f64;

    (prize_x, prize_y)
}

fn parse_prize_part2(line: &str) -> (f64, f64) {
    let prize_str = "Prize: X=";
    let prize_x_start = line.find(prize_str).unwrap() + prize_str.len();
    let prize_x_end = line.find(",").unwrap();
    let prize_y_start = line.find("Y=").unwrap() + "Y+".len();
    let prize_y_end = line.trim().len();

    let prize_x = 10000000000000_f64 + to_num(&line[prize_x_start..prize_x_end]) as f64;
    let prize_y = 10000000000000_f64 + to_num(&line[prize_y_start..prize_y_end]) as f64;

    (prize_x, prize_y)
}

fn parse_machine(machine: &str) -> [(f64, f64); 3] {
    let lines: Vec<&str> = machine.trim().lines().collect();
    let button_a = parse_button(lines[0], &"A");
    let button_b = parse_button(lines[1], &"B");
    let prize = parse_prize(lines[2]);

    [button_a, button_b, prize]
}

fn parse_machine_part2(machine: &str) -> [(f64, f64); 3] {
    let lines: Vec<&str> = machine.trim().lines().collect();
    let button_a = parse_button(lines[0], &"A");
    let button_b = parse_button(lines[1], &"B");
    let prize = parse_prize_part2(lines[2]);

    [button_a, button_b, prize]
}

fn find_min(config: [(f64, f64); 3]) -> usize {
    // Solve a system of 2 linear equations
    let [(ax, ay), (bx, by), (tx, ty)] = config;

    let count_a: f64 = (tx * by - ty * bx) / (ax * by - ay * bx);
    let count_b: f64 = (tx - ax * count_a) / bx;
    // Check if the solution is integers
    if count_a.floor() == count_a && count_b.floor() == count_b {
        3 * count_a as usize + count_b as usize
    } else {
        0
    }
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let machines: Vec<&str> = contents.split_terminator("\n\n").collect();
    let mut answer = 0;

    for machine in machines {
        let config = parse_machine(machine);
        answer += find_min(config)
    }

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let machines: Vec<&str> = contents.split_terminator("\n\n").collect();
    let mut answer = 0;

    for machine in machines {
        let config = parse_machine_part2(machine);
        answer += find_min(config)
    }

    println!("Part 1 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
