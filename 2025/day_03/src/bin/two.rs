use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let mut memos: HashMap<(&str, u32, u32), u128> = HashMap::new();
        sum += find_largest_twelve(line, 0, 12, &mut memos);
    }

    println!("{sum}");
}

fn find_largest_twelve<'a>(
    joltages: &'a str,
    cur_index: u32,
    digits_remaining: u32,
    memos: &mut HashMap<(&'a str, u32, u32), u128>,
) -> u128 {
    if digits_remaining == 0 {
        0
    } else if digits_remaining as usize > joltages.len() - cur_index as usize {
        0
    } else if memos.contains_key(&(joltages, cur_index, digits_remaining)) {
        memos[&(joltages, cur_index, digits_remaining)]
    } else {
        let exclude_cur = find_largest_twelve(joltages, cur_index + 1, digits_remaining, memos);
        let include_cur = joltages.chars().collect::<Vec<char>>()[cur_index as usize]
            .to_digit(10)
            .unwrap() as u128
            * 10u128.pow(digits_remaining - 1)
            + find_largest_twelve(joltages, cur_index + 1, digits_remaining - 1, memos);

        if exclude_cur > include_cur {
            memos.insert((joltages, cur_index, digits_remaining), exclude_cur);
            exclude_cur
        } else {
            memos.insert((joltages, cur_index, digits_remaining), include_cur);
            include_cur
        }
    }
}
