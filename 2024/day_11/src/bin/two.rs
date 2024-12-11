use std::{collections::HashMap, vec};

const BASE: u128 = 10;

fn main() {
    let input: Vec<&str> = include_str!("./input.txt").split_whitespace().collect();
    let mut sum = 0;
    let mut memos: HashMap<(u128, u32), u128> = HashMap::new();

    let mut input_nums: Vec<u128> = vec![];
    for num in input {
        input_nums.push(num.parse().unwrap());
    }

    for num in input_nums {
        sum += recursive_find_num_stones(num, 0, &mut memos);
    }

    println!("{sum}");
}

fn recursive_find_num_stones(
    n: u128,
    cur_step: u32,
    memos: &mut HashMap<(u128, u32), u128>,
) -> u128 {
    let mut sum: u128 = 0;

    if cur_step == 75 {
        return 1;
    }

    if memos.contains_key(&(n, cur_step)) {
        return memos[&(n, cur_step)];
    }

    if n == 0 {
        sum += recursive_find_num_stones(1, cur_step + 1, memos);
    } else if n.to_string().len() % 2 == 0 {
        sum += recursive_find_num_stones(
            n % BASE.pow((n.to_string().len() / 2).try_into().unwrap()),
            cur_step + 1,
            memos,
        );
        sum += recursive_find_num_stones(
            n / BASE.pow((n.to_string().len() / 2).try_into().unwrap()),
            cur_step + 1,
            memos,
        );
    } else {
        sum += recursive_find_num_stones(n * 2024, cur_step + 1, memos);
    }

    memos.insert((n, cur_step), sum);
    return sum;
}
