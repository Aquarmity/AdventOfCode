use std::vec;

fn main() {
    let input: Vec<&str> = include_str!("./input.txt").split_whitespace().collect();
    let mut sum = 0;
    let base: u128 = 10;

    let mut input_nums: Vec<u128> = vec![];
    for num in input {
        input_nums.push(num.parse().unwrap());
    }

    for num in input_nums {
        let mut temp: Vec<u128> = vec![];
        temp.push(num);

        for _i in 0..25 {
            for j in 0..temp.len() {
                if temp[j] == 0 {
                    temp[j] = 1;
                } else if temp[j].to_string().len() % 2 == 0 {
                    temp.push(
                        temp[j] % base.pow((temp[j].to_string().len() / 2).try_into().unwrap()),
                    );
                    temp[j] /= base.pow((temp[j].to_string().len() / 2).try_into().unwrap());
                } else {
                    temp[j] *= 2024;
                }
            }
        }

        sum += temp.len();
    }

    println!("{sum}");
}
