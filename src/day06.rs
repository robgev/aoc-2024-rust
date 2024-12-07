use crate::utils::find_start_loc;
use std::{collections::HashSet, fs, usize};

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // up right down left
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut current_dir = 0;
    let mut loc = find_start_loc(&map, '^');
    let mut answer = 1;

    loop {
        let dir = dirs[current_dir];
        let next_in_dir = (loc.0 + dir.0, loc.1 + dir.1);
        map[loc.0 as usize][loc.1 as usize] = 'X';
        if 0 <= next_in_dir.0
            && next_in_dir.0 < map.len() as i32
            && 0 <= next_in_dir.1
            && next_in_dir.1 < map[0].len() as i32
        {
            let next_char = map[next_in_dir.0 as usize][next_in_dir.1 as usize];
            if next_char == '.' || next_char == '^' || next_char == 'X' {
                loc = next_in_dir;
                answer += if next_char == '.' { 1 } else { 0 };
            } else {
                current_dir = (current_dir + 1) % dirs.len();
            }
        } else {
            break;
        }
    }

    println!("Part 1 Answer: {answer} \n");
}

fn can_loop(map: &Vec<Vec<char>>, start_loc: (i32, i32)) -> bool {
    let mut loc = start_loc;
    let mut current_dir = 0;
    let mut loop_visits: HashSet<(i32, i32, usize)> = HashSet::new();
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    loop {
        if loop_visits.contains(&(loc.0, loc.1, current_dir)) {
            return true;
        }
        loop_visits.insert((loc.0, loc.1, current_dir));

        let dir = dirs[current_dir];
        let next_in_dir = (loc.0 + dir.0, loc.1 + dir.1);
        if 0 <= next_in_dir.0
            && next_in_dir.0 < map.len() as i32
            && 0 <= next_in_dir.1
            && next_in_dir.1 < map[0].len() as i32
        {
            let next_char = map[next_in_dir.0 as usize][next_in_dir.1 as usize];
            if next_char == '#' {
                current_dir = (current_dir + 1) % dirs.len();
            } else {
                loc = next_in_dir;
            }
        } else {
            return false;
        }
    }
}

// If we are at a visited cell and on right we will go in the same direction
// as on the first visit - we are gonna be in a loop
fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut current_dir = 0;
    let start_loc = find_start_loc(&map, '^');
    let mut loc = start_loc;
    let mut answer = 0;

    loop {
        let dir = dirs[current_dir];
        let next_in_dir = (loc.0 + dir.0, loc.1 + dir.1);
        map[loc.0 as usize][loc.1 as usize] = 'X';
        if 0 <= next_in_dir.0
            && next_in_dir.0 < map.len() as i32
            && 0 <= next_in_dir.1
            && next_in_dir.1 < map[0].len() as i32
        {
            let next_char = map[next_in_dir.0 as usize][next_in_dir.1 as usize];
            if next_char == '.' || next_char == '^' || next_char == 'X' {
                loc = next_in_dir;
            } else {
                current_dir = (current_dir + 1) % dirs.len();
            }
        } else {
            break;
        }
    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if (i as i32, j as i32) != start_loc && map[i][j] == 'X' {
                map[i][j] = '#';
                if can_loop(&map, start_loc) {
                    answer += 1;
                }
                map[i][j] = 'X';
            }
        }
    }

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
