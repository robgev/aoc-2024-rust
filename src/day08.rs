use crate::utils::is_in_bounds;
use std::{
    collections::{HashMap, HashSet},
    fs, usize,
};

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let bounds = (map.len(), map[0].len());

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != '.' {
                if let Some(other_antennas) = antennas.get(&map[i][j]) {
                    for antenna in other_antennas {
                        let slope = (antenna.0 as i32 - i as i32, antenna.1 as i32 - j as i32);
                        let antinode1 = (i as i32 - slope.0, j as i32 - slope.1);
                        let antinode2 = (antenna.0 as i32 + slope.0, antenna.1 as i32 + slope.1);

                        if is_in_bounds(bounds, antinode1) {
                            antinodes.insert(antinode1);
                        }

                        if is_in_bounds(bounds, antinode2) {
                            antinodes.insert(antinode2);
                        }
                    }
                }
                antennas.entry(map[i][j]).or_insert(Vec::new()).push((i, j))
            }
        }
    }

    let answer = antinodes.len();
    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let map: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let bounds = (map.len(), map[0].len());

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] != '.' {
                if let Some(other_antennas) = antennas.get(&map[i][j]) {
                    for antenna in other_antennas {
                        let slope = (antenna.0 as i32 - i as i32, antenna.1 as i32 - j as i32);
                        let mut antinode1 = (antenna.0 as i32, antenna.1 as i32);
                        let mut antinode2 = (i as i32, j as i32);

                        while is_in_bounds(bounds, antinode1) {
                            antinodes.insert(antinode1);
                            antinode1 = (antinode1.0 - slope.0, antinode1.1 - slope.1);
                        }

                        while is_in_bounds(bounds, antinode2) {
                            antinodes.insert(antinode2);
                            antinode2 = (antinode2.0 + slope.0, antinode2.1 + slope.1);
                        }
                    }
                }
                antennas.entry(map[i][j]).or_insert(Vec::new()).push((i, j))
            }
        }
    }

    let answer = antinodes.len();
    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
