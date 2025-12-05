use std::cmp;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let mut sum: u64 = 0;

    let (ranges, _): (&str, &str) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(u64, u64)> = ranges
        .split_whitespace()
        .map(|s| {
            s.split_once('-')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
                .unwrap()
        })
        .collect();

    let mut skip_indices: HashSet<usize> = HashSet::new();

    for _ in 0..ranges.len() {
        for i in 0..ranges.len() {
            if skip_indices.contains(&i) {
                continue;
            }
            for j in i + 1..ranges.len() {
                if skip_indices.contains(&j) {
                    continue;
                }
                if overlap(ranges[i], ranges[j]) {
                    ranges[i] = (
                        cmp::min(ranges[i].0, ranges[j].0),
                        cmp::max(ranges[i].1, ranges[j].1),
                    );
                    skip_indices.insert(j);
                }
            }
        }
    }

    for i in 0..ranges.len() {
        if skip_indices.contains(&i) {
            continue;
        }
        sum += ranges[i].1 - ranges[i].0 + 1;
    }

    println!("{sum}");
}

fn overlap((start1, end1): (u64, u64), (start2, end2): (u64, u64)) -> bool {
    (start2 >= start1 && start2 <= end1)
        || (end2 >= start1 && end2 <= end1)
        || (start1 >= start2 && start1 <= end2)
        || (end1 >= start2 && end1 <= end2)
}
