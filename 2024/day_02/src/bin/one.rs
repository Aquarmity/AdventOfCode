fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let levels: Vec<&str> = line.split(" ").collect();
        let mut level_nums: Vec<i32> = vec![];
        for level in levels {
            level_nums.push(level.parse().unwrap());
        }

        let mut safe = true;
        if level_nums[1] - level_nums[0] < 0 {
            level_nums.reverse();
        }
        for n in 1..level_nums.len() {
            if (level_nums[n] - level_nums[n - 1]) <= 0 || (level_nums[n] - level_nums[n - 1]) > 3 {
                safe = false;
                break;
            }
        }
        if safe {
            sum += 1;
        }
    }

    println!("{sum}");
}
