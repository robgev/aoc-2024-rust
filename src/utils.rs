pub fn to_num(num_str: &str) -> i32 {
    let num: i32 = num_str.trim().parse().unwrap();

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
