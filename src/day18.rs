use std::{
    collections::{BinaryHeap, HashSet},
    fs, usize,
};

use crate::utils::{is_in_bounds, to_num};

fn traverse(map: &mut [[char; 71]; 71], (sr, sc): (i32, i32)) -> Option<usize> {
    let mut pq: BinaryHeap<(i32, (i32, i32))> = BinaryHeap::from([(0, (sr, sc))]);
    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    while let Some((score, pos)) = pq.pop() {
        if !seen.contains(&pos) {
            seen.insert(pos);
            map[pos.0 as usize][pos.1 as usize] = 'X';
            if pos == (70, 70) {
                return Some(-score as usize);
            }
            let (row, col) = pos;
            for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_row = row + d_row;
                let next_col = col + d_col;
                if is_in_bounds((map.len(), map[0].len()), (next_row, next_col)) {
                    if map[next_row as usize][next_col as usize] != '#' {
                        pq.push((score - 1, (next_row, next_col)));
                    }
                }
            }
        }
    }

    None
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut map = [['.'; 71]; 71];

    contents.lines().take(1024).for_each(|l| {
        let coords: Vec<usize> = l.split(",").map(|n| to_num(n) as usize).collect();

        map[coords[1]][coords[0]] = '#';
    });

    let answer = traverse(&mut map, (0, 0)).unwrap();

    println!("Part 1 Answer: {answer} \n");
}

fn can_pass_after_n(lines: &Vec<&str>, n: usize) -> bool {
    let mut map = [['.'; 71]; 71];
    lines.iter().take(n).for_each(|l| {
        let coords: Vec<usize> = l.split(",").map(|n| to_num(n) as usize).collect();

        map[coords[1]][coords[0]] = '#';
    });

    traverse(&mut map, (0, 0)).is_some()
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut left = 0;
    let mut right = lines.len() - 1;
    let mut middle = (right + left) / 2;

    while left.abs_diff(right) > 1 {
        if can_pass_after_n(&lines, middle) {
            left = middle;
        } else {
            right = middle;
        }
        middle = (right + left) / 2;
    }

    let answer = lines[middle];

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
