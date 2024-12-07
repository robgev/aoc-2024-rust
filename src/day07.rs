use crate::utils::to_u_num;
use regex::Regex;
use std::{fs, u32, usize};

fn is_valid(target: usize, nums: Vec<usize>, total: usize) -> bool {
    if total > target {
        return false;
    }

    if nums.len() == 0 {
        return total == target;
    }

    is_valid(target, nums[1..].to_vec(), total * nums[0])
        || is_valid(target, nums[1..].to_vec(), total + nums[0])
}

fn is_valid_concat(target: usize, nums: Vec<usize>, total: usize) -> bool {
    if total > target {
        return false;
    }

    if nums.len() == 0 {
        return total == target;
    }

    is_valid_concat(target, nums[1..].to_vec(), total * nums[0])
        || is_valid_concat(target, nums[1..].to_vec(), total + nums[0])
        || is_valid_concat(
            target,
            nums[1..].to_vec(),
            to_u_num(&(total.to_string() + &nums[0].to_string())),
        )
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");

    let answer = contents.lines().fold(0, |acc, line| {
        let colon_idx = line.find(":").unwrap();
        let target = to_u_num(&line[0..colon_idx]) as usize;
        let nums: Vec<usize> = line[colon_idx + 1..]
            .trim()
            .split(" ")
            .map(|s| to_u_num(s) as usize)
            .collect();

        if is_valid(target, nums[1..].to_vec(), nums[0]) {
            acc + target
        } else {
            acc
        }
    });

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");

    let answer = contents.lines().fold(0, |acc, line| {
        let colon_idx = line.find(":").unwrap();
        let target = to_u_num(&line[0..colon_idx]) as usize;
        let nums: Vec<usize> = line[colon_idx + 1..]
            .trim()
            .split(" ")
            .map(|s| to_u_num(s) as usize)
            .collect();

        if is_valid_concat(target, nums[1..].to_vec(), nums[0]) {
            acc + target
        } else {
            acc
        }
    });

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
