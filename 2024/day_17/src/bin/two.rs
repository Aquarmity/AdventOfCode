use core::panic;
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");

    let re = Regex::new(r": (?<n>.*)").unwrap();
    let cur_group = re.captures_iter(&input);

    let program: Vec<u64> = cur_group.last().unwrap()["n"]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let a = place_bits(0, &program, program.len() - 1);

    println!("{a}");
}

fn place_bits(a: u64, program: &Vec<u64>, cur_num: usize) -> u64 {
    for i in 0..0b111 {
        if run_opcodes(a + i, 0, 0, 0, program) == program[cur_num] {
            if cur_num == 0 {
                return a + i;
            }
            let remaining = place_bits((a + i) << 3, program, cur_num - 1);

            if remaining != 0 {
                return remaining;
            }
        }
    }

    return 0;
}

fn run_opcodes(
    mut a: u64,
    mut b: u64,
    mut c: u64,
    mut instr_ptr: usize,
    program: &Vec<u64>,
) -> u64 {
    while instr_ptr < program.len() {
        let opcode = program[instr_ptr];
        let operand = program[instr_ptr + 1];

        match opcode {
            0 => {
                a >>= combo(operand, a, b, c);
            }

            1 => {
                b ^= operand;
            }

            2 => {
                b = combo(operand, a, b, c) % 8;
            }

            3 => {
                if a != 0 {
                    instr_ptr = operand as usize;
                    continue;
                }
            }

            4 => {
                b ^= c;
            }

            5 => {
                return combo(operand, a, b, c) % 8;
            }

            6 => {
                b = a >> combo(operand, a, b, c);
            }

            7 => {
                c = a >> combo(operand, a, b, c);
            }

            _ => {}
        }

        instr_ptr += 2;
    }

    return 999;
}

fn combo(operand: u64, a: u64, b: u64, c: u64) -> u64 {
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
