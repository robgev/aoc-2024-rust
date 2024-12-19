use std::usize;

pub fn to_num(num_str: &str) -> i32 {
    let num: i32 = num_str.trim().parse().unwrap();

    num
}

pub fn to_u_num(num_str: &str) -> usize {
    let num: usize = num_str.trim().parse().unwrap();

    num
}

pub fn find_start_loc(map: &Vec<Vec<char>>, symbol: char) -> (i32, i32) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == symbol {
                return (i as i32, j as i32);
            }
        }
    }

    return (0, 0);
}

pub fn is_in_bounds(sizes: (usize, usize), index: (i32, i32)) -> bool {
    0 <= index.0 && index.0 < sizes.0 as i32 && 0 <= index.1 && index.1 < sizes.1 as i32
}

pub fn print_grid(map: &Vec<Vec<char>>, title: &str) {
    println!("{}", title);
    map.iter()
        .for_each(|line| println!("{}", line.iter().collect::<String>()));
    println!();
}
