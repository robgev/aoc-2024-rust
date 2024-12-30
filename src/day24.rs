use std::{
    collections::{HashMap, VecDeque},
    fmt::format,
    fs,
};

use crate::utils::to_num;

fn perform_op(w1: bool, op: String, w2: bool) -> bool {
    match op.as_str() {
        "XOR" => w1 ^ w2,
        "AND" => w1 & w2,
        "OR" => w1 | w2,
        _ => false,
    }
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let content_parts: Vec<&str> = contents.split("\n\n").collect();
    let mut wires: HashMap<String, bool> = HashMap::new();
    content_parts[0].lines().for_each(|l| {
        let mut parts = l.split(":");
        let name = parts.next().unwrap().to_string();
        let num = parts.next().unwrap().trim() == "1";

        wires.insert(name, num);
    });

    let mut q: VecDeque<(String, String, String, String)> = content_parts[1]
        .lines()
        .map(|l| {
            let mut parts = l.split(" ");
            let first_gate = parts.next().unwrap().to_string();
            let op = parts.next().unwrap().to_string();
            let second_gate = parts.next().unwrap().to_string();
            // skip the arrow
            parts.next();
            let output = parts.next().unwrap().to_string();

            (first_gate, op, second_gate, output)
        })
        .collect();

    while !q.is_empty() {
        let (w1_name, op, w2_name, out_name) = q.pop_front().unwrap();

        if wires.contains_key(&w1_name) && wires.contains_key(&w2_name) {
            let w1 = wires.get(&w1_name).unwrap();
            let w2 = wires.get(&w2_name).unwrap();
            wires.insert(out_name, perform_op(*w1, op, *w2));
        } else {
            q.push_back((w1_name, op, w2_name, out_name));
        }
    }

    let answer = wires.iter().fold(0, |acc, (k, v)| {
        if k.starts_with('z') {
            let bit = to_num(&k[1..]) as u32;
            if *v {
                acc + 2_usize.pow(bit)
            } else {
                acc
            }
        } else {
            acc
        }
    });

    println!("Part 1 Answer: {answer} \n");
}

fn is_correct_op(
    wire_name: &String,
    current_bit: i32,
    ops: &HashMap<String, (String, String, String)>,
    correct_op: &str,
) -> bool {
    if !ops.contains_key(wire_name) {
        return false;
    }

    let (w1_name, op, w2_name) = ops.get(wire_name).unwrap();

    let x_name = format!("x{:0>2}", current_bit);
    let y_name = format!("y{:0>2}", current_bit);

    return *op == correct_op.to_string()
        && (*w1_name == x_name && *w2_name == y_name || *w1_name == y_name && *w2_name == x_name);
}

fn verify_prev(
    wire_name: &String,
    current_bit: i32,
    ops: &HashMap<String, (String, String, String)>,
) -> bool {
    if !ops.contains_key(wire_name) {
        return false;
    }

    let (w1_name, op, w2_name) = ops.get(wire_name).unwrap();
    op == "AND"
        && (is_correct_op(w1_name, current_bit, ops, "XOR")
            && verify_carry(w2_name, current_bit, ops)
            || is_correct_op(w2_name, current_bit, ops, "XOR")
                && verify_carry(w1_name, current_bit, ops))
}

fn verify_carry(
    wire_name: &String,
    current_bit: i32,
    ops: &HashMap<String, (String, String, String)>,
) -> bool {
    if !ops.contains_key(wire_name) {
        return false;
    }

    let (w1_name, op, w2_name) = ops.get(wire_name).unwrap();

    if current_bit == 1 {
        return op == "AND"
            && (w1_name == "x00" && w2_name == "y00" || w1_name == "y00" && w2_name == "x00");
    }

    op == "OR"
        && (is_correct_op(w1_name, current_bit - 1, ops, "AND")
            && verify_prev(w2_name, current_bit - 1, ops)
            || is_correct_op(w2_name, current_bit - 1, ops, "AND")
                && verify_prev(w1_name, current_bit - 1, ops))
}

fn check_z_bit(
    wire_name: &String,
    current_bit: i32,
    ops: &HashMap<String, (String, String, String)>,
) -> bool {
    if !ops.contains_key(wire_name) {
        return false;
    }

    let (w1_name, op, w2_name) = ops.get(wire_name).unwrap();

    // The operation on bit should be X XOR Y
    if op != "XOR" {
        return false;
    }

    if current_bit == 0 {
        return w1_name == "x00" && w2_name == "y00";
    }

    // One of the connected wires should be the X XOR Y
    // And the other one should be the carry bit
    // For the carry bit we need to have a recursive function that
    // verifies all the prev bits
    return (is_correct_op(w1_name, current_bit, ops, "XOR")
        && verify_carry(w2_name, current_bit, ops))
        || (is_correct_op(w2_name, current_bit, ops, "XOR")
            && verify_carry(w1_name, current_bit, ops));
}

fn verify(current_bit: i32, ops: &HashMap<String, (String, String, String)>) -> bool {
    let z_name = format!("z{:0>2}", current_bit);

    check_z_bit(&z_name, current_bit, ops)
}

fn find_faulty_bit_loc(ops: &HashMap<String, (String, String, String)>) -> i32 {
    for i in 0..46 {
        if !verify(i, ops) {
            return i;
        }
    }

    0
}

fn find_swap(ops: &mut HashMap<String, (String, String, String)>) -> (String, String) {
    let current_bit_loc = find_faulty_bit_loc(&ops);
    for (out_name1, op1) in ops.clone() {
        for (out_name2, op2) in ops.clone() {
            if out_name1 != out_name2 {
                ops.entry(out_name1.clone())
                    .and_modify(|entry| *entry = op2.clone());
                ops.entry(out_name2.clone())
                    .and_modify(|entry| *entry = op1.clone());

                if find_faulty_bit_loc(&ops) > current_bit_loc {
                    return (out_name1, out_name2);
                }

                ops.entry(out_name1.clone())
                    .and_modify(|entry| *entry = op1.clone());
                ops.entry(out_name2.clone())
                    .and_modify(|entry| *entry = op2.clone());
            }
        }
    }

    ("".to_string(), "".to_string())
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let content_parts: Vec<&str> = contents.split("\n\n").collect();

    let mut ops: HashMap<String, (String, String, String)> = HashMap::new();
    content_parts[1].lines().for_each(|l| {
        let mut parts = l.split(" ");
        let first_gate = parts.next().unwrap().to_string();
        let op = parts.next().unwrap().to_string();
        let second_gate = parts.next().unwrap().to_string();
        // skip the arrow
        parts.next();
        let output = parts.next().unwrap().to_string();

        ops.insert(output, (first_gate, op, second_gate));
    });

    let mut swaps: Vec<String> = Vec::new();

    for _ in 0..4 {
        let (g1, g2) = find_swap(&mut ops);
        dbg!(&g1, &g2);
        swaps.push(g1);
        swaps.push(g2);
    }

    swaps.sort();
    let answer = swaps.join(",");

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
