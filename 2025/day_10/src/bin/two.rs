use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let buttons_re = Regex::new(r"\((?<w>[\d,]+)").unwrap();
        let buttons: Vec<Vec<usize>> = buttons_re
            .captures_iter(&line)
            .map(|caps| {
                let mut list: Vec<usize> = vec![];
                for n in caps.name("w").unwrap().as_str().split(',') {
                    list.push(n.parse().unwrap());
                }
                list
            })
            .collect();

        let joltages_re = Regex::new(r"\{(?<j>[\d,]+)").unwrap();
        let joltages: Vec<u16> = joltages_re
            .captures_iter(&line)
            .next()
            .unwrap()
            .name("j")
            .unwrap()
            .as_str()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        sum += configure_joltage_levels(
            vec![0; joltages.len()],
            &joltages,
            &buttons,
            &mut HashMap::new(),
        );
        println!("{sum}");
    }

    println!("{sum}");
}

fn configure_joltage_levels(
    current_state: Vec<u16>,
    joltages: &Vec<u16>,
    buttons: &Vec<Vec<usize>>,
    memos: &mut HashMap<Vec<u16>, u32>,
) -> u32 {
    if memos.contains_key(&(current_state.clone())) {
        return memos[&current_state];
    }

    if current_state == *joltages {
        return 0;
    }
    for i in 0..current_state.len() {
        if current_state[i] > joltages[i] {
            return u32::MAX;
        }
    }

    let mut min_presses: u32 = u32::MAX;
    for button in buttons {
        let mut new_state: Vec<u16> = current_state.clone();
        for index in button {
            new_state[*index] += 1;
        }
        let config = configure_joltage_levels(new_state, joltages, buttons, memos);
        if config < min_presses {
            min_presses = config + 1;
            memos.insert(current_state.clone(), min_presses);
        } else {
            memos.insert(current_state.clone(), u32::MAX);
        }
    }

    min_presses
}
