fn main() {
    let input: Vec<char> = include_str!("./input.txt").chars().collect();
    let mut sum: u64 = 0;

    let mut files: Vec<(u32, u32)> = vec![];
    let mut free_spaces: Vec<u32> = vec![];
    for i in (0..input.len()).step_by(2) {
        files.push((input[i].to_digit(10).unwrap(), (i / 2).try_into().unwrap()));

        if i + 1 < input.len() - 1 {
            free_spaces.push(input[i + 1].to_digit(10).unwrap());
        }
    }

    let mut len: usize = 0;
    for num in &files {
        len += num.0 as usize;
    }
    for num in &free_spaces {
        len += *num as usize;
    }

    let mut file_system: Vec<u32> = vec![];
    for i in 0..files.len() {
        for _j in 0..(files[i].0 as usize) {
            file_system.push(files[i].1);
        }
        if i < free_spaces.len() {
            for _j in 0..(free_spaces[i] as usize) {
                file_system.push(u32::MAX);
            }
        }
    }

    let mut cur_swap = len - 1;
    while file_system.contains(&u32::MAX) {
        let index: usize = file_system.iter().position(|x| *x == u32::MAX).unwrap();
        file_system[index] = file_system[cur_swap];
        file_system[cur_swap] = u32::MAX - 1;
        cur_swap -= 1;
    }

    for i in 0..len {
        if file_system[i] == u32::MAX - 1 {
            break;
        }
        sum += (i as u64) * (file_system[i] as u64);
    }

    println!("{sum}");
}
