fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let mut diagram: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        diagram.push(line.chars().collect());
    }

    for i in 0..diagram.len() {
        for j in 0..diagram[i].len() {
            if diagram[i][j] == '@' {
                if num_adjacent(&diagram, i, j) < 4 {
                    sum += 1;
                }
            }
        }
    }

    println!("{sum}");
}

fn num_adjacent(diagram: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
    let mut sum = 0;

    for i in 0..=2 {
        match diagram.get((y + i).wrapping_sub(1)) {
            Some(v) => {
                for j in 0..=2 {
                    if i == 1 && j == 1 {
                        continue;
                    }
                    match v.get((x + j).wrapping_sub(1)) {
                        Some(&c) => {
                            if c == '@' {
                                sum += 1;
                            }
                        }

                        None => {}
                    }
                }
            }

            None => {}
        }
    }

    sum
}
