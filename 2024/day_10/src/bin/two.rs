fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let mut topographic_map: Vec<Vec<u32>> = vec![];

    for line in input.lines() {
        let mut line_nums: Vec<u32> = vec![];
        for c in &line.chars().collect::<Vec<char>>() {
            line_nums.push(c.to_digit(10).unwrap().try_into().unwrap());
        }
        topographic_map.push(line_nums);
    }

    for i in 0..topographic_map.len() {
        for j in 0..topographic_map.len() {
            if topographic_map[i][j] == 0 {
                sum += num_hiking_trails(i, j, &topographic_map);
            }
        }
    }

    println!("{sum}");
}

fn num_hiking_trails(i: usize, j: usize, topographic_map: &Vec<Vec<u32>>) -> i32 {
    let mut temp = 0;
    let cur_num = topographic_map[i][j];

    if cur_num == 9 {
        return 1;
    }

    if i > 0 {
        if topographic_map[i - 1][j] == cur_num + 1 {
            temp += num_hiking_trails(i - 1, j, &topographic_map);
        }
    }
    if j > 0 {
        if topographic_map[i][j - 1] == cur_num + 1 {
            temp += num_hiking_trails(i, j - 1, &topographic_map);
        }
    }
    if i < topographic_map.len() - 1 {
        if topographic_map[i + 1][j] == cur_num + 1 {
            temp += num_hiking_trails(i + 1, j, &topographic_map);
        }
    }
    if j < topographic_map.len() - 1 {
        if topographic_map[i][j + 1] == cur_num + 1 {
            temp += num_hiking_trails(i, j + 1, &topographic_map);
        }
    }

    return temp;
}
