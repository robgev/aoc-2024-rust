use crate::utils::{find_start_loc, is_in_bounds, print_grid};
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs, i32, usize,
};

type PosAndDir = (i32, i32, i32, i32);
type Track = HashMap<PosAndDir, HashSet<PosAndDir>>;

fn traverse(map: &Vec<Vec<char>>, (sr, sc): (i32, i32), (sdr, sdc): (i32, i32)) -> usize {
    let mut pq: BinaryHeap<(i32, PosAndDir)> = BinaryHeap::from([(0, (sr, sc, sdr, sdc))]);
    let mut seen: HashSet<PosAndDir> = HashSet::new();

    while let Some((score, pos)) = pq.pop() {
        if !seen.contains(&pos) {
            seen.insert(pos);
            if map[pos.0 as usize][pos.1 as usize] == 'E' {
                return -score as usize;
            }
            let (row, col, dir_row, dir_col) = pos;
            for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_row = row + d_row;
                let next_col = col + d_col;
                if is_in_bounds((map.len(), map[0].len()), (next_row, next_col)) {
                    if map[next_row as usize][next_col as usize] != '#' {
                        if d_row == dir_row && d_col == dir_col {
                            pq.push((score - 1, (next_row, next_col, d_row, d_col)));
                        } else {
                            pq.push((score - 1001, (next_row, next_col, d_row, d_col)));
                        }
                    }
                }
            }
        }
    }

    0
}

fn find_tracks(map: &Vec<Vec<char>>, (sr, sc): (i32, i32), (sdr, sdc): (i32, i32)) -> Track {
    let mut pq: BinaryHeap<(i32, PosAndDir, Option<PosAndDir>)> =
        BinaryHeap::from([(0, (sr, sc, sdr, sdc), None)]);
    let mut scores: HashMap<PosAndDir, i32> = HashMap::from([((sr, sc, sdr, sdc), 0)]);
    // We could instead just use the number from the first part as well
    let mut min_score = i32::MAX;
    let mut reached_from: Track = HashMap::new();

    while let Some((score, pos, prev_pos)) = pq.pop() {
        if -score <= *(scores.get(&pos).unwrap_or(&i32::MAX)) {
            *(scores.entry(pos).or_insert(-score)) = -score;
            if map[pos.0 as usize][pos.1 as usize] == 'E' {
                if -score > min_score {
                    return reached_from;
                }
                min_score = -score;
            }

            // By now we know this is a position reached with lowest cost (it's a heap)
            // So this position can be reached in minimal cost from the previous pos
            if let Some(pp) = prev_pos {
                reached_from.entry(pos).or_insert(HashSet::new()).insert(pp);
            }

            let (row, col, dir_row, dir_col) = pos;
            for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_row = row + d_row;
                let next_col = col + d_col;
                if is_in_bounds((map.len(), map[0].len()), (next_row, next_col)) {
                    if map[next_row as usize][next_col as usize] != '#'
                        && -score <= *(scores.get(&pos).unwrap_or(&i32::MAX))
                    {
                        if d_row == dir_row && d_col == dir_col {
                            pq.push((score - 1, (next_row, next_col, d_row, d_col), Some(pos)));
                        } else {
                            pq.push((score - 1001, (next_row, next_col, d_row, d_col), Some(pos)));
                        }
                    }
                }
            }
        }
    }

    reached_from
}

fn backtrack_count(tracks: &Track, pos: PosAndDir) -> i32 {
    let mut stack: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    stack.push(pos);

    while !stack.is_empty() {
        let current_pos = stack.pop().unwrap();
        positions.insert((current_pos.0, current_pos.1));
        if let Some(reached_from) = tracks.get(&current_pos) {
            for pos in reached_from {
                stack.push(*pos);
            }
        }
    }

    positions.len() as i32
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_loc = find_start_loc(&map, 'S');
    let answer = traverse(&map, start_loc, (0, 1));

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let start_loc = find_start_loc(&map, 'S');
    let end_loc = find_start_loc(&map, 'E');
    let tracks = find_tracks(&map, start_loc, (0, 1));
    let answer = tracks.iter().fold(0, |acc, (pos, _)| {
        if pos.0 == end_loc.0 && pos.1 == end_loc.1 {
            acc + backtrack_count(&tracks, *pos)
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
