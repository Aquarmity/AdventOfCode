use regex::Regex;
use std::cmp::Ordering;

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

        /*let buttons: Vec<u32> = buttons_re
        .captures_iter(&line)
        .map(|caps| {
            let mut toggle: u32 = 0;
            for n in caps.name("w").unwrap().as_str().split(',') {
                toggle += 2u32.pow(n.parse().unwrap());
            }
            toggle
        })
        .collect();*/

        let joltages_re = Regex::new(r"\{(?<j>[\d,]+)").unwrap();
        let joltages: Vec<u32> = joltages_re
            .captures_iter(&line)
            .next()
            .unwrap()
            .name("j")
            .unwrap()
            .as_str()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        /*sum += configure_joltage_levels(
            vec![0; joltages.len()],
            &joltages,
            &buttons,
            &mut HashMap::new(),
        );*/
        let mut level_order: Vec<usize> = vec![];
        for i in 0..joltages.len() {
            level_order.push(i);
        }
        level_order.sort_by(|a, b| {
            if buttons.iter().filter(|v| v.contains(a)).count()
                < buttons.iter().filter(|v| v.contains(b)).count()
            {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        sum += configure_joltage_levels(
            vec![0; buttons.len()],
            &joltages,
            &buttons,
            &level_order,
            0,
            u32::MAX,
        );
        println!("{sum}");
    }

    println!("{sum}");
}

fn configure_joltage_levels(
    current_state: Vec<u32>,
    joltages: &Vec<u32>,
    buttons: &Vec<Vec<usize>>,
    level_order: &Vec<usize>,
    current_level: usize,
    mut absolute_min: u32,
) -> u32 {
    if current_state.iter().sum::<u32>() > absolute_min {
        return u32::MAX;
    }
    if current_level == joltages.len() {
        println!("{:?}", current_state);
        return current_state.iter().sum();
    }

    let level_buttons: Vec<usize> = buttons
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v.contains(&level_order[current_level]) && current_state[i] == 0 {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    let offset: u32 = buttons
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v.contains(&level_order[current_level]) && current_state[i] != 0 {
                Some(current_state[i] as u32)
            } else {
                None
            }
        })
        .sum();
    if offset > joltages[level_order[current_level]] {
        return u32::MAX;
    }
    let mut states: Vec<Vec<u32>> = vec![];
    if level_buttons.len() > 0 {
        possible_states(
            &mut states,
            current_state.clone(),
            &level_buttons,
            joltages[level_order[current_level]] - offset,
            0,
        );
    }
    if states.is_empty() {
        return configure_joltage_levels(
            current_state,
            joltages,
            buttons,
            level_order,
            current_level + 1,
            absolute_min,
        );
    }

    for state in states {
        let config = configure_joltage_levels(
            state.clone(),
            joltages,
            buttons,
            level_order,
            current_level + 1,
            absolute_min,
        );

        if config < absolute_min {
            absolute_min = config;
        }
    }

    absolute_min
}

fn possible_states(
    states: &mut Vec<Vec<u32>>,
    mut current_state: Vec<u32>,
    level_buttons: &Vec<usize>,
    max: u32,
    current_button: usize,
) {
    if current_button == level_buttons.len() - 1 {
        current_state[level_buttons[current_button]] = max;
        states.push(current_state);
    } else {
        for n in 0..=max {
            current_state[level_buttons[current_button]] = n;
            possible_states(
                states,
                current_state.clone(),
                level_buttons,
                max - n,
                current_button + 1,
            );
            current_state[level_buttons[current_button]] = 0;
        }
    }
}

/*fn configure_joltage_levels(
    current_state: Vec<u32>,
    joltages: &Vec<u32>,
    buttons: &Vec<Vec<usize>>,
    cur_index: usize,
    memos: &mut HashMap<(Vec<u32>, usize), u32>,
) -> u32 {
    if current_state == *joltages {
        return 0;
    }

    if memos.contains_key(&(current_state.clone(), cur_index)) {
        println!("hit!");
        return memos[&(current_state, cur_index)];
    }

    if cur_index == buttons.len() - 1 {
        let remaining_presses =
            joltages[buttons[cur_index][0]] - current_state[buttons[cur_index][0]];

        let mut new_state = current_state.clone();
        for index in &buttons[cur_index] {
            new_state[*index] += remaining_presses;
        }
        if new_state == *joltages {
            println!("{remaining_presses}");
            return remaining_presses;
        } else {
            return u32::MAX;
        }
    }

    let mut min_presses: u32 = u32::MAX;
    let mut new_state: Vec<u32> = current_state.clone();
    'outer: for i in 0..=max_state(joltages, &buttons[cur_index]) {
        for index in &buttons[cur_index] {
            new_state[*index] = current_state[*index] + i;
            if new_state[*index] > joltages[*index] {
                break 'outer;
            }
        }
        let config =
            configure_joltage_levels(new_state.clone(), joltages, buttons, cur_index + 1, memos);
        if config != u32::MAX && config + i < min_presses {
            min_presses = config + i;
            memos.insert((current_state.clone(), cur_index), min_presses);
        } else {
            memos.insert((current_state.clone(), cur_index), u32::MAX);
        }
    }
    min_presses
}*/

/*fn configure_joltage_levels(
    current_state: Vec<u32>,
    joltages: &Vec<u32>,
    buttons: &Vec<Vec<usize>>,
    memos: &mut HashMap<Vec<u32>, u32>,
) -> u32 {
    if memos.contains_key(&(current_state.clone())) {
        return memos[&current_state];
    }

    if current_state == *joltages {
        return 0;
    }

    let mut min_presses: u32 = u32::MAX;
    'outer: for button in buttons {
        let mut new_state = current_state.clone();
        for index in button {
            new_state[*index] += 1;
            if new_state[*index] > joltages[*index] {
                break 'outer;
            }
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
}*/
/*fn configure_joltage_levels(
    current_state: Vec<u32>,
    joltages: &Vec<u32>,
    buttons: &Vec<u32>,
    cur_index: usize,
) -> u32 {
    /*
    if current_state[0] < 2 {
        println!("{:?}", current_state);
    }*/
    for i in 0..joltages.len() {
        let mut current_level = 0;
        for (j, button) in buttons.iter().enumerate() {
            if button & 2u32.pow(i as u32) != 0 {
                current_level += current_state[j];
            }
        }
        if current_level != joltages[i] {
            break;
        } else if i == joltages.len() - 1 {
            println!("{:?}, {}", current_state, current_state.iter().sum::<u32>());
            return current_state.iter().sum();
        }
    }
    if cur_index == buttons.len() {
        return u32::MAX;
    }

    let mut new_state = current_state.clone();
    let mut min_presses = u32::MAX;
    for i in 0..=max_state(&joltages, buttons[cur_index]) {
        new_state[cur_index] = current_state[cur_index] + i;
        let config =
            configure_joltage_levels(new_state.clone(), joltages, buttons, cur_index + 1);
        if config < min_presses {
            min_presses = config;
        }
    }

    min_presses
}

fn max_state(joltages: &Vec<u32>, button: u32) -> u32 {
    let mut min = u32::MAX;
    for i in 0..joltages.len() {
        if button & 2u32.pow(i as u32) != 0 {
            if joltages[i] < min {
                min = joltages[i];
            }
        }
    }
    min
}*/

/*fn max_state(joltages: &Vec<u32>, button: &Vec<usize>) -> u32 {
    let mut min = u32::MAX;
    for index in button {
        if joltages[*index] < min {
            min = joltages[*index];
        }
    }
    min
}*/
