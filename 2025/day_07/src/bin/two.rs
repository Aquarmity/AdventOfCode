use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let mut diagram: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        diagram.push(line.chars().collect());
    }

    let start_index: usize = diagram[0].iter().position(|c| *c == 'S').unwrap();
    let mut timelines_after_split: HashMap<(usize, usize), u64> = HashMap::new();

    println!(
        "{}",
        count_timelines(&diagram, 1, start_index, &mut timelines_after_split)
    );
}

fn count_timelines(
    diagram: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    timelines_after_split: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if i >= diagram.len() {
        1
    } else {
        match diagram[i][j] {
            '.' => count_timelines(diagram, i + 1, j, timelines_after_split),
            '^' => {
                if timelines_after_split.contains_key(&(i, j)) {
                    return timelines_after_split[&(i, j)];
                } else {
                    let split_timelines =
                        count_timelines(diagram, i + 1, j - 1, timelines_after_split)
                            + count_timelines(diagram, i + 1, j + 1, timelines_after_split);
                    timelines_after_split.insert((i, j), split_timelines);
                    split_timelines
                }
            }
            _ => 0,
        }
    }
}
