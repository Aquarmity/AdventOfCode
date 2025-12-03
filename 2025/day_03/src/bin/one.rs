fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let (mut largest, mut next_largest): (char, char) = ('0', '0');

        let chars: Vec<char> = line.chars().collect();
        for i in 0..chars.len() - 1 {
            if chars[i] > largest {
                largest = chars[i];
                next_largest = '0';
            } else if chars[i] > next_largest {
                next_largest = chars[i];
            }
        }
        if *chars.last().unwrap() > next_largest {
            next_largest = *chars.last().unwrap();
        }

        sum += largest.to_digit(10).unwrap() * 10 + next_largest.to_digit(10).unwrap();
    }

    println!("{sum}");
}
