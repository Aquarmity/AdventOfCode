#[derive(Clone)]
struct Key {
    is_before: Vec<i32>,
    is_after: Vec<i32>,
}

impl Key {
    fn new() -> Self {
        Self {
            is_before: vec![],
            is_after: vec![],
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;
    let mut keys = vec![Key::new(); 100];

    let inputs: Vec<&str> = input.split("\n\n").collect();

    for line in inputs[0].lines() {
        let key_and_rule: Vec<&str> = line.split('|').collect();
        keys[key_and_rule[0].parse::<usize>().unwrap()]
            .is_before
            .push(key_and_rule[1].parse().unwrap());
        keys[key_and_rule[1].parse::<usize>().unwrap()]
            .is_after
            .push(key_and_rule[0].parse().unwrap());
    }

    for line in inputs[1].lines() {
        let update: Vec<&str> = line.split(',').collect();
        let mut update_nums: Vec<usize> = vec![];
        for u in update {
            update_nums.push(u.parse().unwrap());
        }
        let mut incorrect_order = false;
        let mut seen: Vec<i32> = vec![];
        'check_rules: for n in update_nums.clone() {
            for after in seen.clone() {
                if !keys[n].is_after.contains(&after) {
                    incorrect_order = true;
                    break;
                }
            }
            if incorrect_order {
                let mut fixed_update_nums: Vec<usize> = vec![];
                let mut next_number = true;
                let line_len = update_nums.len();
                while fixed_update_nums.len() != line_len {
                    for i in 0..update_nums.len() {
                        for j in 0..update_nums.len() {
                            if keys[update_nums[i]]
                                .is_after
                                .contains(&(update_nums[j] as i32))
                            {
                                next_number = false;
                                break;
                            }
                        }
                        if next_number {
                            fixed_update_nums.push(update_nums[i]);
                            update_nums.remove(i);
                            break;
                        }
                        next_number = true;
                    }
                }

                update_nums = fixed_update_nums;
                break 'check_rules;
            }

            seen.push(n.try_into().unwrap());
        }
        seen.clear();

        if incorrect_order {
            let index = update_nums.len() / 2;
            sum += update_nums[index];
        }
    }

    println!("{sum}");
}
