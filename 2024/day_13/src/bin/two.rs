use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let re = Regex::new(r"(\+|=)(?<x>\d+).*(\+|=)(?<y>\d+)").unwrap();

    let buttons_and_prizes: Vec<(i64, i64)> = re
        .captures_iter(&input)
        .map(|caps| {
            let x: i64 = caps.name("x").unwrap().as_str().parse().unwrap();
            let y: i64 = caps.name("y").unwrap().as_str().parse().unwrap();
            (x, y)
        })
        .collect();

    let mut a_buttons: Vec<(i64, i64)> = vec![];
    let mut b_buttons: Vec<(i64, i64)> = vec![];
    let mut prize_positions: Vec<(i64, i64)> = vec![];
    for i in (0..buttons_and_prizes.len()).step_by(3) {
        a_buttons.push(buttons_and_prizes[i]);
        b_buttons.push(buttons_and_prizes[i + 1]);
        prize_positions.push(buttons_and_prizes[i + 2]);
    }

    for pos in &mut prize_positions {
        pos.0 += 10000000000000;
        pos.1 += 10000000000000;
    }

    for i in 0..a_buttons.len() {
        let divisor = a_buttons[i].0 * b_buttons[i].1 - a_buttons[i].1 * b_buttons[i].0;

        let c1 = (prize_positions[i].0 * b_buttons[i].1 - prize_positions[i].1 * b_buttons[i].0)
            / divisor;
        let c2 = (a_buttons[i].0 * prize_positions[i].1 - a_buttons[i].1 * prize_positions[i].0)
            / divisor;

        if (prize_positions[i].0 * b_buttons[i].1 - prize_positions[i].1 * b_buttons[i].0) % divisor
            == 0
            && (a_buttons[i].0 * prize_positions[i].1 - a_buttons[i].1 * prize_positions[i].0)
                % divisor
                == 0
        {
            if c1 >= 0 && c2 >= 0 {
                sum += 3 * c1 + c2;
            }
        }
    }

    println!("{sum}");
}
