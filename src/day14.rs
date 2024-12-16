use crate::utils::to_num;
use std::{fs, i32, thread::sleep, time::Duration, usize};

fn parse_metric(line: &str, metric: &str) -> (i32, i32) {
    let pat = format!("{}=", metric);
    let x_start = line.find(&pat).unwrap() + pat.len();
    let x_end = x_start + line[x_start..].find(",").unwrap();
    let y_start = x_end + 1;
    let y_end = y_start + line[y_start..].find(" ").unwrap_or(line.len() - y_start);

    let x = to_num(&line[x_start..x_end]);
    let y = to_num(&line[y_start..y_end]);

    (x, y)
}

fn parse(line: &str) -> (i32, i32, i32, i32) {
    let (px, py) = parse_metric(line, &"p");
    let (vx, vy) = parse_metric(line, &"v");

    (px, py, vx, vy)
}

fn find_quadrant(px: i32, py: i32) -> usize {
    if px < (101 / 2) && py < (103 / 2) {
        return 0;
    }

    if px > (101 / 2) && py < (103 / 2) {
        return 1;
    }

    if px < (101 / 2) && py > (103 / 2) {
        return 2;
    }

    if px > (101 / 2) && py > (103 / 2) {
        return 3;
    }

    return 4;
}

fn clean_grid() -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for _ in 0..103 {
        let mut row: Vec<char> = vec!['.'; 101];
        row.push('\n');
        result.push(row);
    }

    result
}

fn print_grid(grid: Vec<Vec<char>>) {
    grid.iter().for_each(|row| {
        let row_str: String = row.iter().collect();
        println!("{}", row_str);
    });
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let robots: Vec<(i32, i32, i32, i32)> = contents.lines().map(|l| parse(l)).collect();
    let mut quadrants = [0, 0, 0, 0, 0];

    for robot in robots {
        let (px, py, vx, vy) = robot;
        let final_px = (px + 100 * vx).rem_euclid(101);
        let final_py = (py + 100 * vy).rem_euclid(103);

        quadrants[find_quadrant(final_px, final_py)] += 1;
    }

    let answer = quadrants[0..4].iter().fold(1, |acc, count| acc * count);

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let robots: Vec<(i32, i32, i32, i32)> = contents.lines().map(|l| parse(l)).collect();
    let mut min_safety = i32::MAX;
    let mut answer = 0;

    for t in 0..(101 * 103) {
        let mut quadrants = [0, 0, 0, 0, 0];
        for robot in &robots {
            let (px, py, vx, vy) = robot;
            let final_px = (px + t * vx).rem_euclid(101);
            let final_py = (py + t * vy).rem_euclid(103);
            quadrants[find_quadrant(final_px, final_py)] += 1;
        }

        let safety = quadrants[0..4].iter().fold(1, |acc, count| acc * count);
        if safety < min_safety {
            min_safety = safety;
            answer = t;
        }
    }

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
