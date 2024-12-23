use std::{
    collections::{BinaryHeap, HashSet},
    fs, usize,
};

use crate::utils::{find_start_loc, is_in_bounds};

fn get_scores(map: &Vec<Vec<char>>, (sr, sc): (i32, i32)) -> Vec<Vec<i32>> {
    let mut pq: BinaryHeap<(i32, (i32, i32, i32, i32))> = BinaryHeap::from([(0, (sr, sc, -1, -1))]);
    let mut scores: Vec<Vec<i32>> = vec![vec![-1; map[0].len()]; map.len()];
    scores[sr as usize][sc as usize] = 0;

    while let Some((score, pos)) = pq.pop() {
        let (row, col, wall_r, wall_c) = pos;
        if map[pos.0 as usize][pos.1 as usize] == 'E' {
            return scores;
        }
        for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next_row = row + d_row;
            let next_col = col + d_col;
            if is_in_bounds((map.len(), map[0].len()), (next_row, next_col)) {
                if map[next_row as usize][next_col as usize] != '#'
                    && scores[next_row as usize][next_col as usize] == -1
                {
                    scores[next_row as usize][next_col as usize] =
                        scores[row as usize][col as usize] + 1;
                    pq.push((score - 1, (next_row, next_col, wall_r, wall_c)));
                }
            }
        }
    }

    scores
}

fn count_shortcuts(scores: &Vec<Vec<i32>>, radius: i32) -> i32 {
    let mut count = 0;
    for row in 0..scores.len() {
        for col in 0..scores[0].len() {
            if scores[row][col] != -1 {
                // All possible wall jumps
                for r in 2..(radius + 1) {
                    for dr in 0..(r + 1) {
                        let dc = r - dr;
                        for (d_row, d_col) in
                            // In some cases we will have duplicates, make sure we don't
                            HashSet::from([(dr, dc), (dr, -dc), (-dr, dc), (-dr, -dc)])
                        {
                            let next_row = row as i32 + d_row;
                            let next_col = col as i32 + d_col;
                            if is_in_bounds((scores.len(), scores[0].len()), (next_row, next_col)) {
                                let next_score = scores[next_row as usize][next_col as usize];
                                if next_score != -1 {
                                    // Wall jump takes 2 seconds for the first part and radius
                                    // seconds for the second part, so total saved time is radius
                                    // seconds less
                                    let diff = scores[row][col] - next_score - r;
                                    if diff >= 100 {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_loc = find_start_loc(&map, 'S');
    let track_with_scores = get_scores(&map, start_loc);
    let answer = count_shortcuts(&track_with_scores, 2);

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_loc = find_start_loc(&map, 'S');
    let track_with_scores = get_scores(&map, start_loc);
    let answer = count_shortcuts(&track_with_scores, 20);

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
