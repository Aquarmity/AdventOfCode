use core::panic;
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    let re = Regex::new(r": (?<n>.*)").unwrap();
    let mut cur_group = re.captures_iter(&input);

    let mut instr_ptr = 0;
    let mut a: u32 = cur_group.next().unwrap()["n"].parse().unwrap();
    let mut b: u32 = cur_group.next().unwrap()["n"].parse().unwrap();
    let mut c: u32 = cur_group.next().unwrap()["n"].parse().unwrap();
    let nums: Vec<u32> = cur_group.next().unwrap()["n"]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    while instr_ptr < nums.len() {
        run_opcode(
            nums[instr_ptr],
            nums[instr_ptr + 1],
            &mut a,
            &mut b,
            &mut c,
            &mut instr_ptr,
        );
    }
}

fn run_opcode(
    opcode: u32,
    operand: u32,
    a: &mut u32,
    b: &mut u32,
    c: &mut u32,
    instr_ptr: &mut usize,
) {
    match opcode {
        0 => {
            *a /= 2_u32.pow(combo(operand, *a, *b, *c));
        }

        1 => {
            *b ^= operand;
        }

        2 => {
            *b = combo(operand, *a, *b, *c) % 8;
        }

        3 => {
            if *a != 0 {
                *instr_ptr = operand as usize;
                return;
            }
        }

        4 => {
            *b ^= *c;
        }

        5 => {
            print!("{},", combo(operand, *a, *b, *c) % 8);
        }

        6 => {
            *b = *a / 2_u32.pow(combo(operand, *a, *b, *c));
        }

        7 => {
            *c = *a / 2_u32.pow(combo(operand, *a, *b, *c));
        }

        _ => {}
    }

    *instr_ptr += 2;
}

fn combo(operand: u32, a: u32, b: u32, c: u32) -> u32 {
    if operand < 4 {
        operand
    } else if operand == 4 {
        a
    } else if operand == 5 {
        b
    } else if operand == 6 {
        c
    } else {
        panic!("invalid operand");
    }
}
