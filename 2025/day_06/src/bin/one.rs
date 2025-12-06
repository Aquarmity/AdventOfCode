fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let mut worksheet: Vec<Vec<i64>> = vec![];
    let mut operators: Vec<&str> = vec![];

    for line in input.lines() {
        if line != input.lines().last().unwrap() {
            let line_nums: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            worksheet.push(line_nums);
        } else {
            operators = line.split_whitespace().collect();
        }
    }

    for i in 0..worksheet[0].len() {
        match operators[i] {
            "*" => {
                let mut column_prod: i64 = 1;
                for j in 0..worksheet.len() {
                    column_prod *= worksheet[j][i];
                }
                sum += column_prod;
            }
            "+" => {
                let mut column_sum: i64 = 0;
                for j in 0..worksheet.len() {
                    column_sum += worksheet[j][i];
                }
                sum += column_sum;
            }
            _ => {}
        }
    }

    println!("{sum}");
}
