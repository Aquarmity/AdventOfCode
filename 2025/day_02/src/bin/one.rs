fn main() {
    let input = include_str!("./input.txt").trim();
    let mut sum = 0;
    let ranges = input.split(',');

    for range in ranges {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end): (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());
        let repetition_size: u32 = num_digits(end) / 2;

        for i in 10u64.pow(repetition_size - 1)..10u64.pow(repetition_size) {
            let test_num = i * 10u64.pow(repetition_size) + i;
            if test_num >= start && test_num <= end {
                sum += test_num;
            }
        }
    }

    println!("{sum}");
}

fn num_digits(mut n: u64) -> u32 {
    let mut num_digits = 0;
    while n > 0 {
        n /= 10;
        num_digits += 1
    }
    num_digits
}
