use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

use crate::utils::to_num;

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut secrets: Vec<usize> = contents.lines().map(|l| to_num(l) as usize).collect();

    for i in 0..secrets.len() {
        let mut num = secrets[i];
        for _ in 0..2000 {
            let shift1 = num << 6;
            // 0 XOR last 6 bits
            num ^= shift1;
            // result bitmasked with ones
            num &= 2_usize.pow(24) - 1;
            let shift2 = num >> 5;
            num ^= shift2;
            num &= 2_usize.pow(24) - 1;
            let shift3 = num << 11;
            num ^= shift3;
            num &= 2_usize.pow(24) - 1;
        }

        secrets[i] = num;
    }
    let answer: usize = secrets.iter().sum();

    println!("Part 1 Answer: {answer} \n");
}

type Seq = (i8, i8, i8, i8);
fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let secrets: Vec<usize> = contents.lines().map(|l| to_num(l) as usize).collect();
    let mut totals_for_seqs: HashMap<Seq, i32> = HashMap::new();

    for i in 0..secrets.len() {
        let mut prices: Vec<i8> = Vec::new();
        let mut num = secrets[i];
        prices.push((num % 10) as i8);
        for _ in 0..2000 {
            let shift1 = num << 6;
            // 0 XOR last 6 bits
            num ^= shift1;
            // result bitmasked with ones
            num &= 2_usize.pow(24) - 1;
            let shift2 = num >> 5;
            num ^= shift2;
            num &= 2_usize.pow(24) - 1;
            let shift3 = num << 11;
            num ^= shift3;
            num &= 2_usize.pow(24) - 1;
            prices.push((num % 10) as i8);
        }

        let mut seen_seqs: HashSet<Seq> = HashSet::new();
        prices.windows(5).for_each(|p| {
            let sequence = (p[1] - p[0], p[2] - p[1], p[3] - p[2], p[4] - p[3]);
            if !seen_seqs.contains(&sequence) {
                seen_seqs.insert(sequence);
                *(totals_for_seqs.entry(sequence).or_insert(0)) += p[4] as i32;
            }
        })
    }

    let answer = totals_for_seqs.values().max().unwrap();

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
