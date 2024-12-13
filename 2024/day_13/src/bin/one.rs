use matrix_mc::{matrix, rref};
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let re = Regex::new(r"(\+|=)(?<x>\d+).*(\+|=)(?<y>\d+)").unwrap();

    let buttons_and_prizes: Vec<(i32, i32)> = re
        .captures_iter(&input)
        .map(|caps| {
            let x: i32 = caps.name("x").unwrap().as_str().parse().unwrap();
            let y: i32 = caps.name("y").unwrap().as_str().parse().unwrap();
            (x, y)
        })
        .collect();

    let mut a_buttons: Vec<(i32, i32)> = vec![];
    let mut b_buttons: Vec<(i32, i32)> = vec![];
    let mut prize_positions: Vec<(i32, i32)> = vec![];
    for i in (0..buttons_and_prizes.len()).step_by(3) {
        a_buttons.push(buttons_and_prizes[i]);
        b_buttons.push(buttons_and_prizes[i + 1]);
        prize_positions.push(buttons_and_prizes[i + 2]);
    }

    for i in 0..a_buttons.len() {
        let mut crane_matrix = matrix![
            a_buttons[i].0, b_buttons[i].0, prize_positions[i].0;
            a_buttons[i].1, b_buttons[i].1, prize_positions[i].1;
        ];

        rref(&mut crane_matrix);

        if (crane_matrix[0][2] - crane_matrix[0][2].round()).abs() < 0.0001
            && (crane_matrix[1][2] - crane_matrix[1][2].round()).abs() < 0.0001
        {
            let a_presses: i32 = crane_matrix[0][2].round() as i32;
            let b_presses: i32 = crane_matrix[1][2].round() as i32;
            if a_presses >= 0 && a_presses <= 100 && b_presses >= 0 && b_presses <= 100 {
                sum += 3 * a_presses + b_presses;
            }
        }
    }

    println!("{sum}");
}
