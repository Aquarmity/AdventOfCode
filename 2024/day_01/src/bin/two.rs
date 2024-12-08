fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let nums: Vec<&str> = line.split("   ").collect();
        let left_entry: i32 = nums[0].parse().unwrap();
        let right_entry: i32 = nums[1].parse().unwrap();
        left_list.push(left_entry);
        right_list.push(right_entry);
    }

    left_list.sort();
    right_list.sort();

    for val in left_list {
        sum += val * (right_list.iter().filter(|&n| *n == val).count() as i32);
    }

    println!("{sum}");
}