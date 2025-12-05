fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let (ranges, ids): (&str, &str) = input.split_once("\n\n").unwrap();

    let ranges: Vec<(u64, u64)> = ranges
        .split_whitespace()
        .map(|s| {
            s.split_once('-')
                .map(|(min, max)| (min.parse().unwrap(), max.parse().unwrap()))
                .unwrap()
        })
        .collect();

    for line in ids.lines() {
        let id: u64 = line.parse().unwrap();
        for (min, max) in &ranges {
            if id >= *min && id <= *max {
                sum += 1;
                break;
            }
        }
    }

    println!("{sum}");
}
