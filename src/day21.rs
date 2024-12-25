use std::{
    collections::{BinaryHeap, HashMap},
    fs, usize,
};

use crate::utils::{is_in_bounds, to_num};

type KeyCode = (i32, i32);
type LevelAndPath = (KeyCode, KeyCode, i32);

fn dir_to_key(dir: (i32, i32)) -> KeyCode {
    match dir {
        (-1, 0) => (0, 1),
        (1, 0) => (1, 1),
        (0, 1) => (1, 2),
        (0, -1) => (1, 0),
        _ => (0, 0),
    }
}

fn find_path_on_keyboard(
    (sr, sc): KeyCode,
    (tr, tc): KeyCode,
    is_numpad: bool,
) -> Vec<Vec<KeyCode>> {
    let invalid_loc = if is_numpad { (3, 0) } else { (0, 0) };
    let size = if is_numpad { (4, 3) } else { (2, 3) };
    let a_loc = if is_numpad { key_to_loc('A') } else { (0, 2) };
    let mut pq: BinaryHeap<(i32, (i32, i32, Vec<KeyCode>))> =
        BinaryHeap::from([(0, (sr, sc, Vec::new()))]);
    let mut scores: HashMap<KeyCode, i32> = HashMap::from([((sr, sc), 0)]);
    let mut min_score = i32::MAX;
    let mut possible_paths: Vec<Vec<KeyCode>> = Vec::new();

    while let Some((score, pos)) = pq.pop() {
        let (row, col, p) = pos;
        if -score <= *(scores.get(&(row, col)).unwrap_or(&i32::MAX)) {
            *(scores.entry((row, col)).or_insert(-score)) = -score;
            if (row, col) == (tr, tc) {
                if -score > min_score {
                    return possible_paths;
                }

                min_score = -score;
                let mut possible_path = p.clone();
                if !is_numpad {
                    possible_path.push(a_loc);
                }
                possible_paths.push(possible_path);
            }

            for (d_row, d_col) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next_row = row + d_row;
                let next_col = col + d_col;
                if is_in_bounds(size, (next_row, next_col))
                    && (next_row, next_col) != invalid_loc
                    && -score <= *(scores.get(&(row, col)).unwrap_or(&i32::MAX))
                {
                    let current_key = dir_to_key((d_row, d_col));
                    let mut new_path = p.clone();
                    new_path.push(current_key);
                    pq.push((score - 1, (next_row, next_col, new_path)));
                }
            }
        }
    }

    possible_paths
}

// Because each character has a distinct combination independent of next/prev
// characters, we chop the problem to solve it just for one char
// and then do it for more than one char in a for loop
fn length_for_keys(
    from: KeyCode,
    to: KeyCode,
    level: i32,
    cache: &mut HashMap<LevelAndPath, usize>,
) -> usize {
    if level == 1 {
        let possible_paths = find_path_on_keyboard(from, to, false);

        return possible_paths[0].len();
    }

    if let Some(hit) = cache.get(&(from, to, level)) {
        *hit
    } else {
        //Find all possible paths from key X to key Y:
        // For each of the possible key stroke combos to go from X to Y
        // Obfuscate the combination to the needed level and find the minimum length one
        // The full shortest combo will always be the combo where each character has the
        // shortest combo
        let possible_paths = find_path_on_keyboard(from, to, false);
        let shortest_length = possible_paths
            .iter()
            .map(|path| {
                let mut length = 0;
                for i in 0..path.len() {
                    let to_in = path[i];
                    let from_in = if i == 0 { (0, 2) } else { path[i - 1] };

                    length += length_for_keys(from_in, to_in, level - 1, cache);
                }

                length
            })
            .min()
            .unwrap();

        *(cache.entry((from, to, level)).or_insert(0)) = shortest_length;

        shortest_length
    }
}

fn code_to_keys(sequence: &Vec<(i32, i32)>) -> Vec<Vec<(i32, i32)>> {
    let a_loc = key_to_loc('A');
    // We always start on A
    let mut current_loc = a_loc;
    let mut all_paths = Vec::new();

    for key_code in sequence {
        let possible_paths = find_path_on_keyboard(current_loc, *key_code, true);

        if all_paths.is_empty() {
            for mut path in possible_paths {
                path.push((0, 2));
                all_paths.push(path);
            }
        } else {
            all_paths = all_paths
                .iter()
                .map(|p: &Vec<(i32, i32)>| {
                    let mut result: Vec<Vec<(i32, i32)>> = Vec::new();
                    for path in &possible_paths {
                        let mut new_path = p.clone();
                        new_path.extend(path.iter());
                        new_path.push((0, 2));
                        result.push(new_path);
                    }

                    result
                })
                .flatten()
                .collect();
        }

        current_loc = *key_code;
    }

    all_paths
}

fn key_to_loc(key: char) -> (i32, i32) {
    match key {
        'A' => (3, 2),
        '0' => (3, 1),
        '1' => (2, 0),
        '2' => (2, 1),
        '3' => (2, 2),
        '4' => (1, 0),
        '5' => (1, 1),
        '6' => (1, 2),
        '7' => (0, 0),
        '8' => (0, 1),
        '9' => (0, 2),
        _ => (-1, -1),
    }
}

fn loc_to_key(loc: (i32, i32)) -> char {
    match loc {
        (0, 1) => '^',
        (1, 1) => 'v',
        (1, 0) => '<',
        (1, 2) => '>',
        (0, 2) => 'A',
        _ => '.',
    }
}

fn print_seq(seq: &Vec<(i32, i32)>) {
    let result = seq.iter().map(|key| loc_to_key(*key)).collect::<String>();
    println!("{result}")
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut answer = 0;
    let mut cache: HashMap<LevelAndPath, usize> = HashMap::new();
    for line in contents.lines() {
        let num = to_num(&line[0..3]);
        let sequence: Vec<(i32, i32)> = line.trim().chars().map(|ch| key_to_loc(ch)).collect();
        let possible_inputs = code_to_keys(&sequence);
        let shortest_length = possible_inputs
            .iter()
            .map(|path| {
                let mut length = 0;
                for i in 0..path.len() {
                    let to = path[i];
                    let from = if i == 0 { (0, 2) } else { path[i - 1] };

                    length += length_for_keys(from, to, 2, &mut cache);
                }

                length
            })
            .min()
            .unwrap();
        answer += num as usize * shortest_length;
    }

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut answer = 0;
    let mut cache: HashMap<LevelAndPath, usize> = HashMap::new();
    for line in contents.lines() {
        let num = to_num(&line[0..3]);
        let sequence: Vec<(i32, i32)> = line.trim().chars().map(|ch| key_to_loc(ch)).collect();
        let possible_inputs = code_to_keys(&sequence);
        let shortest_length = possible_inputs
            .iter()
            .map(|path| {
                let mut length = 0;
                for i in 0..path.len() {
                    let to = path[i];
                    let from = if i == 0 { (0, 2) } else { path[i - 1] };

                    length += length_for_keys(from, to, 25, &mut cache);
                }

                length
            })
            .min()
            .unwrap();
        answer += num as usize * shortest_length;
    }

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
