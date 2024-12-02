use crate::utils::to_num;
use std::{fs, i32};

fn validate_line(nums: Vec<i32>) -> bool {
    let sign = if nums[1] - nums[0] < 0 { -1 } else { 1 };
    let mut valid = true;

    for i in 1..nums.len() {
        let diff = nums[i] - nums[i - 1];
        let current_sign = if diff < 0 { -1 } else { 1 };

        if diff.abs() < 1 || diff.abs() > 3 || sign != current_sign {
            valid = false;
            break;
        }
    }

    valid
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut answer = 0;

    contents.lines().for_each(|line| {
        let nums: Vec<i32> = line.split(" ").map(to_num).collect();
        let valid = validate_line(nums);

        if valid {
            answer += 1;
        }
    });

    println!("Part 1 Answer: {answer}");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut answer = 0;

    contents.lines().for_each(|line| {
        let nums: Vec<i32> = line.split(" ").map(to_num).collect();
        let mut valid = false;

        for i in 0..nums.len() {
            let mut dampened_nums = nums.clone();
            dampened_nums.remove(i);
            if validate_line(dampened_nums) {
                valid = true
            }
        }

        if valid || validate_line(nums) {
            answer += 1;
        }
    });

    println!("Part 2 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
