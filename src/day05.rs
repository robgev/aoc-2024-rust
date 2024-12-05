use crate::utils::to_num;
use std::{cmp::Ordering, collections::HashMap, fs};

fn check_is_valid(nums: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    for i in 0..nums.len() {
        let num = nums[i];
        if let Some(items_after) = rules.get(&num) {
            for item in items_after {
                if nums[0..i].contains(item) {
                    return false;
                }
            }
        }
    }

    true
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let content_parts: Vec<&str> = contents.split("\n\n").collect();
    let rules_map: HashMap<i32, Vec<i32>> =
        content_parts[0]
            .lines()
            .fold(HashMap::new(), |mut acc, item| {
                let pair: Vec<i32> = item.split("|").map(|num| to_num(num)).collect();
                acc.entry(pair[0]).or_insert(Vec::new()).push(pair[1]);

                acc
            });
    dbg!(rules_map.clone());
    let answer = content_parts[1].lines().fold(0, |total, update| {
        let nums: Vec<i32> = update.split(",").map(|num| to_num(num)).collect();
        if check_is_valid(&nums, &rules_map) {
            total + nums[nums.len() / 2]
        } else {
            total
        }
    });

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let content_parts: Vec<&str> = contents.split("\n\n").collect();
    let rules_map: HashMap<i32, Vec<i32>> =
        content_parts[0]
            .lines()
            .fold(HashMap::new(), |mut acc, item| {
                let pair: Vec<i32> = item.split("|").map(|num| to_num(num)).collect();
                acc.entry(pair[0]).or_insert(Vec::new()).push(pair[1]);

                acc
            });
    let answer = content_parts[1].lines().fold(0, |total, update| {
        let mut nums: Vec<i32> = update.split(",").map(|num| to_num(num)).collect();
        if !check_is_valid(&nums, &rules_map) {
            nums.sort_by(|num1, num2| {
                if let Some(items_after) = rules_map.get(&num1) {
                    if items_after.contains(num2) {
                        return Ordering::Greater;
                    }
                }

                if let Some(items_after) = rules_map.get(&num2) {
                    if items_after.contains(num1) {
                        return Ordering::Less;
                    }
                }

                Ordering::Equal
            });
            total + nums[nums.len() / 2]
        } else {
            total
        }
    });

    println!("Part 1 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
