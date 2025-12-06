fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let mut worksheet: Vec<Vec<char>> = vec![];
    let mut operators: Vec<char> = vec![];
    let mut problem_numbers: Vec<Vec<i64>> = vec![];

    for line in input.lines() {
        worksheet.push(line.chars().collect());
    }

    let mut column_numbers: Vec<i64> = vec![];
    for i in 0..worksheet[0].len() {
        let mut current_number: String = String::new();

        for j in 0..worksheet.len() {
            if worksheet[j][i] == ' ' {
                continue;
            } else if worksheet[j][i].is_numeric() {
                current_number.push(worksheet[j][i]);
            } else {
                operators.push(worksheet[j][i]);
            }
        }

        if current_number != String::new() {
            column_numbers.push(current_number.parse().unwrap());
        }

        if current_number == String::new() || i == worksheet[0].len() - 1 {
            problem_numbers.push(column_numbers.clone());
            column_numbers.clear();
        }
    }

    for i in 0..problem_numbers.len() {
        match operators[i] {
            '*' => {
                let mut column_prod: i64 = 1;
                for n in &problem_numbers[i] {
                    column_prod *= n;
                }
                sum += column_prod;
            }
            '+' => {
                let mut column_sum: i64 = 0;
                for n in &problem_numbers[i] {
                    column_sum += n;
                }
                sum += column_sum;
            }
            _ => {}
        }
    }

    println!("{sum}");
}
