use std::{collections::HashMap, fs, usize};

use crate::utils::is_in_bounds;

fn count_perimeter(map: &Vec<Vec<char>>, loc: (i32, i32), current_plant: char) -> i32 {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut result = 0;

    for dir in dirs {
        let neighbor = (loc.0 + dir.0, loc.1 + dir.1);
        if is_in_bounds((map.len(), map[0].len()), neighbor) {
            let neighbor_plant = map[neighbor.0 as usize][neighbor.1 as usize];
            result += if neighbor_plant != current_plant {
                1
            } else {
                0
            }
        } else {
            result += 1;
        }
    }

    result
}

fn is_same(map: &Vec<Vec<char>>, current_plant: char, neighbor: (i32, i32)) -> bool {
    if is_in_bounds((map.len(), map[0].len()), neighbor) {
        let neighbor_plant = map[neighbor.0 as usize][neighbor.1 as usize];
        current_plant == neighbor_plant
    } else {
        false
    }
}

fn count_corners(map: &Vec<Vec<char>>, loc: (i32, i32), current_plant: char) -> i32 {
    // up, right, down, left
    let mut neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut result = 0;

    for i in 0..neighbors.len() {
        let dir = neighbors[i];
        neighbors[i] = (loc.0 + dir.0, loc.1 + dir.1);
    }

    // up & right
    if !is_same(map, current_plant, neighbors[0]) && !is_same(map, current_plant, neighbors[1]) {
        result += 1;
    }

    // right and down
    if !is_same(map, current_plant, neighbors[1]) && !is_same(map, current_plant, neighbors[2]) {
        result += 1;
    }

    // down and left
    if !is_same(map, current_plant, neighbors[2]) && !is_same(map, current_plant, neighbors[3]) {
        result += 1;
    }

    // left and up
    if !is_same(map, current_plant, neighbors[3]) && !is_same(map, current_plant, neighbors[0]) {
        result += 1;
    }

    // up & right
    if is_same(map, current_plant, neighbors[0]) && is_same(map, current_plant, neighbors[1]) {
        let top_right = (loc.0 - 1, loc.1 + 1);
        if !is_same(map, current_plant, top_right) {
            result += 1;
        }
    }

    // right and down
    if is_same(map, current_plant, neighbors[1]) && is_same(map, current_plant, neighbors[2]) {
        let down_right = (loc.0 + 1, loc.1 + 1);
        if !is_same(map, current_plant, down_right) {
            result += 1;
        }
    }

    // down and left
    if is_same(map, current_plant, neighbors[2]) && is_same(map, current_plant, neighbors[3]) {
        let down_left = (loc.0 + 1, loc.1 - 1);
        if !is_same(map, current_plant, down_left) {
            result += 1;
        }
    }

    // left and up
    if is_same(map, current_plant, neighbors[3]) && is_same(map, current_plant, neighbors[0]) {
        let top_left = (loc.0 - 1, loc.1 - 1);
        if !is_same(map, current_plant, top_left) {
            result += 1;
        }
    }

    result
}

fn group_loc(
    seen: &mut Vec<Vec<char>>,
    loc: (i32, i32),
    current_plant: char,
    groups: &mut Vec<Vec<i32>>,
    group_id: &mut i32,
) {
    if seen[loc.0 as usize][loc.1 as usize] != '.' {
        *group_id += 1;
        let mut tiles: Vec<(i32, i32)> = Vec::new();
        tiles.push(loc);
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        while !tiles.is_empty() {
            let tile = tiles.pop().unwrap();
            seen[tile.0 as usize][tile.1 as usize] = '.';
            groups[tile.0 as usize][tile.1 as usize] = *group_id;
            for dir in dirs {
                let neighbor = (tile.0 + dir.0, tile.1 + dir.1);
                if is_in_bounds((seen.len(), seen[0].len()), neighbor) {
                    let neighbor_plant = seen[neighbor.0 as usize][neighbor.1 as usize];
                    if neighbor_plant == current_plant {
                        tiles.push(neighbor);
                    }
                }
            }
        }
    }
}

fn group_plants(map: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let mut seen = map.clone();
    let mut groups: Vec<Vec<i32>> = seen.iter().map(|s| s.iter().map(|_| 0).collect()).collect();
    let mut group_id = 0;

    for i in 0..seen.len() {
        for j in 0..seen[0].len() {
            group_loc(
                &mut seen,
                (i as i32, j as i32),
                map[i][j],
                &mut groups,
                &mut group_id,
            );
        }
    }

    groups
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // hashmap between location and (area, perimeter) tuples
    let mut measurements: HashMap<i32, (i32, i32)> = HashMap::new();
    let groups = group_plants(&map);
    //groups.iter().for_each(|r| {
    //    r.iter().for_each(|c| print!("{0: <10}", c));
    //    println!()
    //});

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let loc = (i as i32, j as i32);
            // Traverse with seen
            // Look around to find unseen locations with the same char
            // Count the area and perimeter immediately for each region

            let entry = measurements.entry(groups[i][j]).or_insert((0, 0));
            (*entry).0 += 1;
            (*entry).1 += count_perimeter(&map, loc, map[i][j]);
        }
    }
    let answer = measurements
        .values()
        .fold(0, |acc, (area, perimeter)| acc + area * perimeter);

    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    // hashmap between location and (area, perimeter) tuples
    let mut measurements: HashMap<i32, (i32, i32)> = HashMap::new();
    let groups = group_plants(&map);

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let loc = (i as i32, j as i32);
            // Traverse with seen
            // Look around to find unseen locations with the same char
            // Count the area and perimeter immediately for each region

            let entry = measurements.entry(groups[i][j]).or_insert((0, 0));
            (*entry).0 += 1;
            (*entry).1 += count_corners(&map, loc, map[i][j]);
        }
    }
    let answer = measurements
        .values()
        .fold(0, |acc, (area, sides)| acc + area * sides);

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
