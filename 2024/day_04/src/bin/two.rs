fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    let mas = "MAS";
    let sam = "SAM";

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'M' {
                if i <= grid.len() - 3 && j <= grid[i].len() - 3 {
                    if grid[i][j + 2] == 'M' {
                        if left_diagonal_match(&grid, i, j + 2, mas)
                            && right_diagonal_match(&grid, i, j, mas)
                        {
                            sum += 1;
                        }
                    } else if grid[i][j + 2] == 'S' {
                        if left_diagonal_match(&grid, i, j + 2, sam)
                            && right_diagonal_match(&grid, i, j, mas)
                        {
                            sum += 1;
                        }
                    }
                }
            } else if grid[i][j] == 'S' {
                if i <= grid.len() - 3 && j <= grid[i].len() - 3 {
                    if grid[i][j + 2] == 'M' {
                        if left_diagonal_match(&grid, i, j + 2, mas)
                            && right_diagonal_match(&grid, i, j, sam)
                        {
                            sum += 1;
                        }
                    } else if grid[i][j + 2] == 'S' {
                        if left_diagonal_match(&grid, i, j + 2, sam)
                            && right_diagonal_match(&grid, i, j, sam)
                        {
                            sum += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{sum}");
}

fn left_diagonal_match(grid: &Vec<Vec<char>>, start_i: usize, start_j: usize, word: &str) -> bool {
    let mut word_match = false;
    if start_i <= grid.len() - word.len() && start_j >= word.len() - 1 {
        let mut test_word = String::new();
        for n in 0..word.len() {
            test_word.push(grid[start_i + n][start_j - n]);
        }
        if word == test_word {
            word_match = true;
        }
    }
    word_match
}

fn right_diagonal_match(grid: &Vec<Vec<char>>, start_i: usize, start_j: usize, word: &str) -> bool {
    let mut word_match = false;
    if start_i <= grid.len() - word.len() && start_j <= grid[start_i].len() - word.len() {
        let mut test_word = String::new();
        for n in 0..word.len() {
            test_word.push(grid[start_i + n][start_j + n]);
        }
        if word == test_word {
            word_match = true;
        }
    }
    word_match
}
