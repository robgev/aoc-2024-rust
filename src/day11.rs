use crate::utils::to_u_num;
use std::{collections::HashMap, fs, usize};

fn evolve_single(num: usize) -> Vec<usize> {
    if num == 0 {
        return vec![1];
    }

    let digits = num.checked_ilog10().unwrap() + 1;
    if digits % 2 == 0 {
        return vec![
            num / 10_i32.pow(digits / 2) as usize,
            num % 10_i32.pow(digits / 2) as usize,
        ];
    }

    vec![num * 2024]
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut nums: Vec<Vec<usize>> = contents
        .trim()
        .split(" ")
        .map(|num| vec![to_u_num(num)])
        .collect();
    let mut new_vec: Vec<usize> = Vec::new();

    for j in 0..25 {
        for i in 0..nums.len() {
            new_vec = Vec::new();
            let vec = &nums[i];
            for num in vec {
                let mut result = evolve_single(*num);
                new_vec.append(&mut result);
            }

            nums[i] = new_vec;
        }
    }

    let answer = nums.iter().fold(0, |acc, result| acc + result.len());

    println!("Part 1 Answer: {answer} \n");
}

fn count_for(num: usize, steps: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    let result = if steps == 0 {
        1
    } else if let Some(hit) = cache.get(&(num, steps)) {
        *hit
    } else if num == 0 {
        count_for(1, steps - 1, cache)
    } else {
        let digits = num.checked_ilog10().unwrap() + 1;
        if digits % 2 == 0 {
            count_for(num / 10_i32.pow(digits / 2) as usize, steps - 1, cache)
                + count_for(num % 10_i32.pow(digits / 2) as usize, steps - 1, cache)
        } else {
            count_for(num * 2024, steps - 1, cache)
        }
    };

    cache.insert((num, steps), result);

    result
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let nums: Vec<usize> = contents
        .trim()
        .split(" ")
        .map(|num| to_u_num(num))
        .collect();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    let answer = nums
        .iter()
        .fold(0, |acc, num| acc + count_for(*num, 75, &mut cache));

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
