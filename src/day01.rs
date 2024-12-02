use crate::utils::to_num;
use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut heap1: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap2: BinaryHeap<i32> = BinaryHeap::new();

    contents.lines().for_each(|line| {
        let separator_index = line.find("   ").unwrap();
        let number1 = to_num(&line[0..separator_index]);
        let number2_start = separator_index + "   ".len();
        let number2 = to_num(&line[number2_start..]);
        heap1.push(number1);
        heap2.push(number2);
    });

    let mut answer = 0;
    while !heap1.is_empty() {
        let number1 = heap1.pop().unwrap();
        let number2 = heap2.pop().unwrap();
        answer += number1.abs_diff(number2)
    }

    println!("Part 1 Answer: {answer}");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut freqs: HashMap<i32, i32> = HashMap::new();
    let mut numbers: Vec<i32> = Vec::new();

    contents.lines().for_each(|line| {
        let separator_index = line.find("   ").unwrap();
        let number1 = to_num(&line[0..separator_index]);
        let number2_start = separator_index + "   ".len();
        let number2 = to_num(&line[number2_start..]);
        numbers.push(number1);
        *freqs.entry(number2).or_insert(0) += 1;
    });

    let mut answer = 0;
    for num in numbers {
        let freq = *(freqs.get(&num).unwrap_or(&0));
        answer += num * freq;
    }
    println!("Part 2 Answer: {answer}");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
