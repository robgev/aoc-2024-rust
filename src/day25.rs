use std::fs;

fn get_key(chart_str: &str) -> [i32; 5] {
    let chart: Vec<Vec<char>> = chart_str.lines().map(|l| l.chars().collect()).collect();
    let mut heights: [i32; 5] = [0; 5];
    for col in 0..chart[0].len() {
        for row in 0..(chart.len() - 1) {
            if chart[row][col] == '#' {
                heights[col] += 1;
            }
        }
    }

    heights
}

fn get_lock(chart_str: &str) -> [i32; 5] {
    let chart: Vec<Vec<char>> = chart_str.lines().map(|l| l.chars().collect()).collect();
    let mut heights: [i32; 5] = [0; 5];
    for col in 0..chart[0].len() {
        for row in 1..chart.len() {
            if chart[row][col] == '#' {
                heights[col] += 1;
            }
        }
    }

    heights
}

fn print_scheme(item: [i32; 5]) {
    println!();
    for i in item {
        print!("{i},");
    }
    println!();
}

fn fits(key: [i32; 5], lock: [i32; 5]) -> bool {
    for i in 0..key.len() {
        if lock[i] + key[i] > 5 {
            return false;
        }
    }

    true
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let charts: Vec<&str> = contents.split("\n\n").collect();
    let mut keys: Vec<[i32; 5]> = Vec::new();
    let mut locks: Vec<[i32; 5]> = Vec::new();

    for chart in charts {
        let lines: Vec<&str> = chart.lines().collect();
        if lines[0] == "#####" && lines[6] == "....." {
            locks.push(get_lock(chart));
        } else {
            keys.push(get_key(chart));
        }
    }

    let mut answer = 0;
    for lock in &locks {
        for key in &keys {
            if fits(*key, *lock) {
                answer += 1;
            }
        }
    }

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let answer = 0;

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
