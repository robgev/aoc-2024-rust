use std::{fs, usize};

use crate::utils::to_num;

fn parse_register(line: &str, reg_name: char) -> i32 {
    let pat = format!("Register {}:", reg_name);
    let reg_val_start = line.find(&pat).unwrap() + pat.len();

    to_num(line[reg_val_start..].trim())
}

fn parse_program(line: &str) -> Vec<i8> {
    let pat = "Program:";
    let val_start = line.find(pat).unwrap() + pat.len();

    line[val_start..]
        .trim()
        .split(',')
        .map(|n| to_num(n) as i8)
        .collect()
}

fn get_combo_op_val(operand: i8, registers: &[i32; 3]) -> i32 {
    if operand >= 0 && operand <= 3 {
        operand.into()
    } else if operand >= 4 && operand <= 6 {
        registers[(operand - 4) as usize]
    } else {
        0
    }
}

fn division(operand: i8, save_reg: i8, registers: &mut [i32; 3]) {
    let op_val = get_combo_op_val(operand, registers);
    // A is always the numerator
    let num = registers[0];
    let denom = 2_i32.pow(op_val as u32);

    registers[save_reg as usize] = num / denom;
}

fn jump_not_zero(operand: i8, registers: &[i32; 3], ins_p: usize) -> usize {
    if registers[0] != 0 {
        operand as usize
    } else {
        ins_p + 2
    }
}

fn perform_op(
    (opcode, operand): (i8, i8),
    registers: &mut [i32; 3],
    ins_p: usize,
    output: &mut String,
) -> usize {
    match opcode {
        0 => division(operand, 0, registers),
        1 => registers[1] ^= operand as i32,
        2 => registers[1] = get_combo_op_val(operand, registers) % 8,
        3 => return jump_not_zero(operand, registers, ins_p),
        4 => registers[1] ^= registers[2],
        5 => output.push_str(&format!("{},", get_combo_op_val(operand, registers) % 8)),
        6 => division(operand, 1, registers),
        7 => division(operand, 2, registers),
        _ => {}
    }

    ins_p + 2
}

fn solve_part_1() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut ins_p = 0;
    let mut registers: [i32; 3] = [
        parse_register(lines[0], 'A'),
        parse_register(lines[1], 'B'),
        parse_register(lines[2], 'C'),
    ];
    let program = parse_program(lines[4]);
    let mut answer = String::new();

    while ins_p < program.len() {
        let opcode = program[ins_p];
        let operand = program[ins_p + 1];
        ins_p = perform_op((opcode, operand), &mut registers, ins_p, &mut answer);
    }

    println!("Part 1 Answer: {answer} \n");
}

// Answer is at least 48 bits long
fn trace(program: &Vec<i8>, ans: usize) -> Option<usize> {
    if program.is_empty() {
        return Some(ans);
    }

    for i in 0..8 {
        // Open up 3 bits, to bruteforce all
        // 3 bit combos
        // Reverse engineered ops
        let reg_a = ans << 3 | i;
        let mut reg_b = i ^ 6;
        let reg_c = reg_a >> reg_b;
        reg_b ^= reg_c;
        reg_b ^= 7;

        if reg_b % 8 == *program.last().unwrap() as usize {
            if let Some(rest) = trace(&program[0..program.len() - 1].to_vec(), reg_a) {
                return Some(rest);
            }
        }
    }

    None
}

fn solve_part_2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let program = parse_program(lines[4]);
    let answer = trace(&program, 0).unwrap_or(0);

    println!("Part 2 Answer: {answer} \n");
}

pub fn solve() {
    self::solve_part_1();
    self::solve_part_2();
}
