use std::{collections::VecDeque, fs, usize};

use crate::utils::find_start_loc;

fn get_dir(dir: char) -> (i32, i32) {
    match dir {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => (0, 0),
    }
}

fn make_move(map: &mut Vec<Vec<char>>, loc: (i32, i32), dir: char) -> (i32, i32) {
    let (dy, dx) = get_dir(dir);
    // Pointer to the currently moved item (box or robot);
    let mut next_loc = (loc.0 + dy, loc.1 + dx);

    // Find all the boxes
    while map[next_loc.0 as usize][next_loc.1 as usize] == 'O' {
        next_loc = (next_loc.0 + dy, next_loc.1 + dx);
    }

    // If at the end is not a wall, move everything
    if map[next_loc.0 as usize][next_loc.1 as usize] != '#' {
        while next_loc.0 != loc.0 || next_loc.1 != loc.1 {
            let item = map[next_loc.0 as usize][next_loc.1 as usize];
            let prev_loc = (next_loc.0 - dy, next_loc.1 - dx);
            map[next_loc.0 as usize][next_loc.1 as usize] =
                map[prev_loc.0 as usize][prev_loc.1 as usize];
            map[prev_loc.0 as usize][prev_loc.1 as usize] = item;
            next_loc = prev_loc;
        }

        (next_loc.0 + dy, next_loc.1 + dx)
    } else {
        loc
    }
}

fn calculate_gps_sum(map: &Vec<Vec<char>>, box_char: char) -> usize {
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == box_char {
                sum += 100 * i + j;
            }
        }
    }

    sum
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut map_and_moves = contents.split_terminator("\n\n");
    let mut map: Vec<Vec<char>> = map_and_moves
        .nth(0)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut start_loc = find_start_loc(&map, '@');
    let moves = map_and_moves.nth(0).unwrap();
    moves.chars().for_each(|ch| {
        if ch != '\n' {
            start_loc = make_move(&mut map, start_loc, ch);
        }
    });

    let answer = calculate_gps_sum(&map, 'O');

    println!("Part 1 Answer: {answer} \n");
}

fn find_pair(map: &Vec<Vec<char>>, box_loc: (i32, i32)) -> i32 {
    let box_char = map[box_loc.0 as usize][box_loc.1 as usize];
    let box_pair = if box_char == '[' {
        1
    } else if box_char == ']' {
        -1
    } else {
        0
    };

    box_pair
}

fn touches_box(map: &Vec<Vec<char>>, box_loc: (i32, i32), (dy, dx): (i32, i32)) -> bool {
    let box_pair = find_pair(map, box_loc);
    let next_loc = (box_loc.0 + dy, box_loc.1 + dx);
    if box_loc.0 == 5 && (box_loc.1 == 20 || box_loc.1 == 21) {}

    let next_char = map[next_loc.0 as usize][next_loc.1 as usize];
    let next_loc_pair = (next_loc.0, next_loc.1 + box_pair);
    let next_char_pair = map[next_loc_pair.0 as usize][next_loc_pair.1 as usize];

    next_char == '['
        || next_char == ']'
        || (dx == 0 && (next_char_pair == '[' || next_char_pair == ']'))
}

fn move_box_vertical(map: &mut Vec<Vec<char>>, box_loc: (i32, i32), (dy, dx): (i32, i32)) {
    // Robot applies force in the given location, which is either [ or ]
    let box_char = map[box_loc.0 as usize][box_loc.1 as usize];
    let next_loc = (box_loc.0 + dy, box_loc.1 + dx);
    let box_pair = find_pair(map, box_loc);
    let next_pair_loc = (next_loc.0, next_loc.1 + box_pair);
    let box_pair_char = map[box_loc.0 as usize][(box_loc.1 + box_pair) as usize];

    if box_char == '[' || box_char == ']' {
        if touches_box(map, box_loc, (dy, dx)) {
            map[box_loc.0 as usize][box_loc.1 as usize] = '|';
            map[box_loc.0 as usize][box_loc.1 as usize] = box_char;
            move_box_vertical(map, next_loc, (dy, dx))
        }
        // In x direction, make sure to look "behind" the box instead of "in"
        if touches_box(map, (box_loc.0, box_loc.1 + box_pair), (dy, dx)) {
            move_box_vertical(map, next_pair_loc, (dy, dx))
        }

        let mut item = map[next_loc.0 as usize][next_loc.1 as usize];
        map[next_loc.0 as usize][next_loc.1 as usize] = box_char;
        map[box_loc.0 as usize][box_loc.1 as usize] = item;

        item = map[next_pair_loc.0 as usize][next_pair_loc.1 as usize];
        map[next_pair_loc.0 as usize][next_pair_loc.1 as usize] = box_pair_char;
        map[box_loc.0 as usize][(box_loc.1 + box_pair) as usize] = item;
    }
}

fn move_box_horizontal(map: &mut Vec<Vec<char>>, box_loc: (i32, i32), (dy, dx): (i32, i32)) {
    let next_loc = (box_loc.0 + dy, box_loc.1 + dx);
    let next_pair_loc = (box_loc.0 + dy, box_loc.1 + 2 * dx);
    if touches_box(map, next_loc, (dy, dx)) {
        move_box_horizontal(map, next_pair_loc, (dy, dx))
    }

    let temp = map[next_loc.0 as usize][next_loc.1 as usize];
    map[next_loc.0 as usize][next_loc.1 as usize] = map[box_loc.0 as usize][box_loc.1 as usize];
    map[box_loc.0 as usize][box_loc.1 as usize] =
        map[next_pair_loc.0 as usize][next_pair_loc.1 as usize];
    map[next_pair_loc.0 as usize][next_pair_loc.1 as usize] = temp;
}

fn move_box(map: &mut Vec<Vec<char>>, box_loc: (i32, i32), (dy, dx): (i32, i32)) {
    if dx == 0 {
        move_box_vertical(map, box_loc, (dy, dx));
    } else {
        move_box_horizontal(map, box_loc, (dy, dx));
    }
}

fn is_hitting_wall(map: &Vec<Vec<char>>, loc: (i32, i32), (dy, dx): (i32, i32)) -> bool {
    let current_char = map[loc.0 as usize][loc.1 as usize];
    if current_char == '#' {
        return true;
    }

    if current_char == '.' {
        return false;
    }

    if current_char == '@' {
        return is_hitting_wall(map, (loc.0 + dy, loc.1 + dx), (dy, dx));
    }

    // We know by now we deal with a box
    if dx != 0 {
        // If we move in x direction, next_loc is
        // always going to be the other half of the box
        let next_loc = (loc.0 + dy, loc.1 + 2 * dx);
        return is_hitting_wall(map, next_loc, (dy, dx));
    }

    let box_pair = find_pair(map, loc);
    is_hitting_wall(map, (loc.0 + dy, loc.1 + dx), (dy, dx))
        || is_hitting_wall(map, (loc.0 + dy, loc.1 + dx + box_pair), (dy, dx))
}

fn make_move_part2(map: &mut Vec<Vec<char>>, robot_loc: (i32, i32), dir: char) -> (i32, i32) {
    let (dy, dx) = get_dir(dir);
    let next_loc = (robot_loc.0 + dy, robot_loc.1 + dx);
    let next_char = map[next_loc.0 as usize][next_loc.1 as usize];

    // Check if any of the moved boxes or the robot
    // are going to hit the wall
    if !is_hitting_wall(map, robot_loc, (dy, dx)) {
        // If there are boxes involved move them
        // map gets mutated
        if next_char == '[' || next_char == ']' {
            move_box(map, next_loc, (dy, dx));
        }

        // After boxes are moved, robot should walk as well
        let item = map[next_loc.0 as usize][next_loc.1 as usize];
        map[next_loc.0 as usize][next_loc.1 as usize] =
            map[robot_loc.0 as usize][robot_loc.1 as usize];
        map[robot_loc.0 as usize][robot_loc.1 as usize] = item;

        next_loc
    } else {
        robot_loc
    }
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut map_and_moves = contents.split_terminator("\n\n");
    let mut map: Vec<Vec<char>> = map_and_moves
        .nth(0)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    other => [other, other],
                })
                .flatten()
                .collect()
        })
        .collect();

    let mut start_loc = find_start_loc(&map, '@');
    let moves = map_and_moves.nth(0).unwrap();

    moves.chars().for_each(|ch| {
        if ch != '\n' {
            start_loc = make_move_part2(&mut map, start_loc, ch);
        }
    });

    let answer = calculate_gps_sum(&map, '[');

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
