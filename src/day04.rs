use std::{fs, usize};

fn search_all_directions(grid: &Vec<Vec<char>>, x_row: i32, x_col: i32) -> i32 {
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut found_count = 0;
    let mut symbols: [char; 4] = ['X', 'O', 'O', 'O'];
    let mut row = x_row;
    let mut col = x_col;

    for dir in dirs {
        for i in 1.."XMAS".len() {
            row += dir.0;
            col += dir.1;
            if row >= 0 && (row as usize) < grid.len() && col >= 0 && (col as usize) < grid[0].len()
            {
                symbols[i] = grid[row as usize][col as usize];
            }
        }

        if symbols.iter().collect::<String>() == "XMAS" {
            found_count += 1;
        }

        symbols = ['X', 'O', 'O', 'O'];
        row = x_row;
        col = x_col;
    }

    found_count
}

fn is_x_mas(grid: &Vec<Vec<char>>, a_row: i32, a_col: i32) -> bool {
    let diagonal1 = [(-1, -1), (1, 1)];
    let diagonal2 = [(-1, 1), (1, -1)];
    let diagonals = [diagonal1, diagonal2];
    let mut is_valid = [false, false];

    for i in 0..diagonals.len() {
        let diagonal = diagonals[i];
        let mut symbols = ['O', 'O'];
        for j in 0..diagonal.len() {
            let dir = diagonal[j];
            let row = a_row + dir.0;
            let col = a_col + dir.1;
            if row >= 0 && (row as usize) < grid.len() && col >= 0 && (col as usize) < grid[0].len()
            {
                symbols[j] = grid[row as usize][col as usize];
            }
        }

        is_valid[i] = symbols.contains(&'M') && symbols.contains(&'S');
    }

    is_valid[0] && is_valid[1]
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut answer = 0;
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'X' {
                answer += search_all_directions(&grid, row as i32, col as i32)
            }
        }
    }
    println!("Part 1 Answer: {answer} \n");
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let mut answer = 0;
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'A' {
                if is_x_mas(&grid, row as i32, col as i32) {
                    answer += 1;
                }
            }
        }
    }
    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    //self::solve_part_1();
    self::solve_part_2();
}
