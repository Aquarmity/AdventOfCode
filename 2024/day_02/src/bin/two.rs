fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let levels: Vec<&str> = line.split(" ").collect();
        let mut level_nums: Vec<i32> = vec![];
        for level in levels {
            level_nums.push(level.parse().unwrap());
        }

        let mut safe = false;

        for n in 0..level_nums.len() {
            let mut dampened_levels: Vec<i32> = vec![];
            for l in 0..level_nums.len() {
                if l != n {
                    dampened_levels.push(level_nums[l]);
                }
            }

            let mut still_safe = true;
            if dampened_levels[1] < dampened_levels[0] {
                dampened_levels.reverse();
            }
            for n in 1..dampened_levels.len() {
                if dampened_levels[n] - dampened_levels[n - 1] <= 0
                    || dampened_levels[n] - dampened_levels[n - 1] > 3
                {
                    still_safe = false;
                    break;
                }
            }
            if still_safe {
                safe = true;
                break;
            }
        }
        if safe {
            sum += 1;
        }
    }

    println!("{sum}");
}
