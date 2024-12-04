fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    let xmas = "XMAS";
    let samx = "SAMX";

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'X' {
                sum += horizontal_match(&grid, i, j, xmas);
                sum += vertical_match(&grid, i, j, xmas);
                sum += left_diagonal_match(&grid, i, j, xmas);
                sum += right_diagonal_match(&grid, i, j, xmas);
            } else if grid[i][j] == 'S' {
                sum += horizontal_match(&grid, i, j, samx);
                sum += vertical_match(&grid, i, j, samx);
                sum += left_diagonal_match(&grid, i, j, samx);
                sum += right_diagonal_match(&grid, i, j, samx);
            }
        }
    }

    println!("{sum}");
}

fn horizontal_match(grid: &Vec<Vec<char>>, start_i: usize, start_j: usize, word: &str) -> i32 {
    let mut word_match = 0;
    if start_j <= grid[start_i].len() - word.len() {
        let mut test_word = String::new();
        for j in start_j..start_j + word.len() {
            test_word.push(grid[start_i][j]);
        }
        if word == test_word {
            word_match = 1;
        }
    }
    word_match
}

fn vertical_match(grid: &Vec<Vec<char>>, start_i: usize, start_j: usize, word: &str) -> i32 {
    let mut word_match = 0;
    if start_i <= grid.len() - word.len() {
        let mut test_word = String::new();
        for i in start_i..start_i + word.len() {
            test_word.push(grid[i][start_j]);
        }
        if word == test_word {
            word_match = 1;
        }
    }
    word_match
}

fn left_diagonal_match(grid: &Vec<Vec<char>>, start_i: usize, start_j: usize, word: &str) -> i32 {
    let mut word_match = 0;
    if start_i <= grid.len() - word.len() && start_j >= word.len() - 1 {
        let mut test_word = String::new();
        for n in 0..word.len() {
            test_word.push(grid[start_i + n][start_j - n]);
        }
        if word == test_word {
            word_match = 1;
        }
    }
    word_match
}

fn right_diagonal_match(grid: &Vec<Vec<char>>, start_i: usize, start_j: usize, word: &str) -> i32 {
    let mut word_match = 0;
    if start_i <= grid.len() - word.len() && start_j <= grid[start_i].len() - word.len() {
        let mut test_word = String::new();
        for n in 0..word.len() {
            test_word.push(grid[start_i + n][start_j + n]);
        }
        if word == test_word {
            word_match = 1;
        }
    }
    word_match
}
