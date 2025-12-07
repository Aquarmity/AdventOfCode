use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    let mut diagram: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        diagram.push(line.chars().collect());
    }

    let start_index: usize = diagram[0].iter().position(|c| *c == 'S').unwrap();
    let mut split_positions: HashSet<(usize, usize)> = HashSet::new();

    find_splits(&diagram, 1, start_index, &mut split_positions);

    println!("{}", split_positions.iter().count());
}

fn find_splits(
    diagram: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    split_positions: &mut HashSet<(usize, usize)>,
) {
    if i >= diagram.len() {
        return;
    }
    match diagram[i][j] {
        '.' => {
            find_splits(diagram, i + 1, j, split_positions);
        }
        '^' => {
            if split_positions.contains(&(i, j)) {
                return;
            } else {
                split_positions.insert((i, j));
                find_splits(diagram, i + 1, j - 1, split_positions);
                find_splits(diagram, i + 1, j + 1, split_positions);
            }
        }
        _ => {}
    }
}
