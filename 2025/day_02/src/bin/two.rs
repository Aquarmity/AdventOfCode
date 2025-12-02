fn main() {
    let input = include_str!("./input.txt").trim();
    let mut sum = 0;
    let ranges = input.split(',');

    for range in ranges {
        let (start, end) = range.split_once('-').unwrap();
        let (start, end): (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());

        for i in 1..(num_digits(end) / 2 + 1) {
            for j in 10u64.pow(i - 1)..10u64.pow(i) {
                if i % 2 == 0 {
                    if j / 10u64.pow(i / 2) == j % 10u64.pow(i / 2) {
                        continue;
                    }
                } else {
                    let repetition_digit = j % 10;
                    if j / 10u64.pow(i / 2 + 1) == (j / 10) % 10u64.pow(i / 2)
                        && repetition_digit == (j / 10) % 10
                    {
                        continue;
                    }
                }

                let mut test_num: u64 = j * 10u64.pow(i) + j;
                while test_num < start {
                    test_num = test_num * 10u64.pow(i) + j;
                }
                while test_num <= end {
                    if test_num >= start && test_num <= end {
                        sum += test_num;
                    }
                    test_num = test_num * 10u64.pow(i) + j;
                }
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
