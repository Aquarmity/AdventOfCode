use std::collections::HashMap;

fn main() {
    let input: Vec<char> = include_str!("./input.txt").chars().collect();
    let mut sum: u64 = 0;

    let mut files: HashMap<u32, usize> = HashMap::new();
    let mut free_spaces: Vec<u32> = vec![];
    for i in (0..input.len()).step_by(2) {
        files.insert((i / 2).try_into().unwrap(), input[i].to_digit(10).unwrap() as usize);

        if i + 1 < input.len() - 1 {
            free_spaces.push(input[i + 1].to_digit(10).unwrap());
        }
    }

    let mut len: usize = 0;
    for (_, file_len) in &files {
        len += file_len;
    }
    for num in &free_spaces {
        len += *num as usize;
    }

    let mut file_system: Vec<u32> = vec![];
    let mut free_space_pos: Vec<(usize, usize)> = vec![];
    for i in 0..files.len() {
        for _j in 0..files[&(i as u32)] {
            file_system.push(i.try_into().unwrap());
        }
        if i < free_spaces.len() {
            free_space_pos.push((file_system.len(), free_spaces[i] as usize));
            for _j in 0..(free_spaces[i] as usize) {
                file_system.push(u32::MAX);
            }
        }
    }

    let mut cur_swap = len - 1;
    while !files.is_empty() {
        let mut cur_id = file_system[cur_swap];
        while cur_id == u32::MAX || !files.contains_key(&cur_id) {
            cur_swap -= 1;
            cur_id = file_system[cur_swap];
        }
        
        for i in 0..free_space_pos.len() {
            if free_space_pos[i].0 > cur_swap {
                if cur_swap >= files[&cur_id] {
                    cur_swap -= files[&cur_id];
                }
                break;
            }
            
            if free_space_pos[i].1 >= files[&cur_id] {
                for _i in 0..files[&cur_id] {          
                    file_system[free_space_pos[i].0] = cur_id;
                    free_space_pos[i].0 += 1;
                    free_space_pos[i].1 -= 1;
                    file_system[cur_swap] = u32::MAX;
                    cur_swap -= 1;
                }
                break;
            }
        }
        files.remove(&cur_id);
    }

    for i in 0..len {
        if file_system[i] == u32::MAX {
            continue;
        }
        sum += (i as u64) * (file_system[i] as u64);
    }

    println!("{sum}");
}