use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let re = Regex::new(r"mul\((?<one>[0-9]{1,3})\,(?<two>[0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut count = true;
    let mul_nums: Vec<(i32, i32)> = re
        .captures_iter(&input)
        .map(|caps| {
            if caps.get(0).unwrap().as_str() == "do()" {
                count = true;
                (0, 0)
            } else if caps.get(0).unwrap().as_str() == "don't()" {
                count = false;
                (0, 0)
            } else if count {
                let one: i32 = caps.name("one").unwrap().as_str().parse().unwrap();
                let two: i32 = caps.name("two").unwrap().as_str().parse().unwrap();
                (one, two)
            } else {
                (0, 0)
            }
        })
        .collect();

    for (one, two) in mul_nums {
        sum += one * two;
    }

    println!("{sum}");
}
