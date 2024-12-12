use std::{
    collections::{BinaryHeap, HashSet},
    fs, usize,
};

// BFS using binary heap priority queue
fn count_score(map: &Vec<Vec<u32>>, start_pos: (usize, usize)) -> i32 {
    let (sr, sc) = start_pos;
    let mut pq: BinaryHeap<(u32, (i32, i32))> = BinaryHeap::from([(0, (sr as i32, sc as i32))]);
    let mut answer = 0;
    let mut seen_positions: HashSet<(i32, i32)> = HashSet::new();

    while let Some((steps, pos)) = pq.pop() {
        if !seen_positions.contains(&pos) {
            seen_positions.insert(pos);
            if steps == 9 {
                answer += 1;
            }
            let (row, col) = pos;
            for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_row = row + d_row;
                let next_col = col + d_col;
                if next_row >= 0
                    && (next_row as usize) < map.len()
                    && next_col >= 0
                    && (next_col as usize) < map[0].len()
                {
                    if map[next_row as usize][next_col as usize] > steps
                        && map[next_row as usize][next_col as usize] - steps == 1
                    {
                        pq.push((steps + 1, (next_row, next_col)));
                    }
                }
            }
        }
    }

    answer
}

// DFS
fn count_rating(map: &Vec<Vec<u32>>, start_pos: (usize, usize)) -> i32 {
    let mut stack: Vec<(i32, i32)> = Vec::new();
    stack.push((start_pos.0 as i32, start_pos.1 as i32));
    let mut answer = 0;

    while let Some(pos) = stack.pop() {
        if map[pos.0 as usize][pos.1 as usize] == 9 {
            answer += 1;
        }
        let (row, col) = pos;
        for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next_row = row + d_row;
            let next_col = col + d_col;
            if next_row >= 0
                && (next_row as usize) < map.len()
                && next_col >= 0
                && (next_col as usize) < map[0].len()
            {
                if map[next_row as usize][next_col as usize] > map[pos.0 as usize][pos.1 as usize]
                    && map[next_row as usize][next_col as usize]
                        - map[pos.0 as usize][pos.1 as usize]
                        == 1
                {
                    stack.push((next_row, next_col));
                }
            }
        }
    }

    answer
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let mut answer = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                answer += count_score(&map, (i, j))
            }
        }
    }

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<u32>> = contents
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let mut answer = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                answer += count_rating(&map, (i, j))
            }
        }
    }

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
