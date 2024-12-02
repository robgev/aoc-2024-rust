pub fn to_num(num_str: &str) -> i32 {
    let num: i32 = num_str.trim().parse().unwrap();

    num
}
