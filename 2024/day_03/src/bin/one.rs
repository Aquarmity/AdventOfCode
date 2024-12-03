use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let re = Regex::new(r"mul\((?<one>[0-9]{1,3})\,(?<two>[0-9]{1,3})\)").unwrap();

    let mul_nums: Vec<(i32, i32)> = re
        .captures_iter(&input)
        .map(|caps| {
            let one: i32 = caps.name("one").unwrap().as_str().parse().unwrap();
            let two: i32 = caps.name("two").unwrap().as_str().parse().unwrap();
            (one, two)
        })
        .collect();

    for (one, two) in mul_nums {
        sum += one * two;
    }

    println!("{sum}");
}
