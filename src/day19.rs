use std::{collections::HashMap, fs, usize};

fn can_construct(
    design: &str,
    towels: &Vec<&str>,
    max_len: usize,
    cache: &mut HashMap<String, usize>,
) -> usize {
    let mut result = 0;
    if design == "" {
        return 1;
    } else if let Some(hit) = cache.get(design) {
        return *hit;
    } else {
        for i in 0..((max_len.min(design.len())) + 1) {
            let slice = &design[0..i];
            if towels.contains(&slice) {
                let the_rest = can_construct(&design[i..], towels, max_len, cache);
                if the_rest > 0 {
                    result += the_rest;
                }
            }
        }
    }

    cache.insert(design.to_string(), result);
    result
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut cache: HashMap<String, usize> = HashMap::new();
    let lines: Vec<&str> = contents.lines().collect();
    let towels: Vec<&str> = lines[0].split(", ").collect();
    let mut max_len = 0;

    for towel in &towels {
        if towel.len() > max_len {
            max_len = towel.len();
        }
    }

    let answer = lines[2..].iter().fold(0, |acc, design| {
        if can_construct(design, &towels, max_len, &mut cache) > 0 {
            acc + 1
        } else {
            acc
        }
    });
    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut cache: HashMap<String, usize> = HashMap::new();
    let lines: Vec<&str> = contents.lines().collect();
    let towels: Vec<&str> = lines[0].split(", ").collect();
    let mut max_len = 0;

    for towel in &towels {
        if towel.len() > max_len {
            max_len = towel.len();
        }
    }

    let answer = lines[2..].iter().fold(0, |acc, design| {
        acc + can_construct(design, &towels, max_len, &mut cache)
    });

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
