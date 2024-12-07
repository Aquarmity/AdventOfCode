fn main() {
    let input = include_str!("./input.txt");
    let mut sum: i64 = 0;

    for line in input.lines() {
        let nums: Vec<&str> = line.split(':').collect();
        let total: i64 = nums[0].parse().unwrap();
        let mut operands: Vec<&str> = nums[1].split(' ').collect();
        operands.remove(0);
        let mut operand_nums: Vec<i64> = vec![];
        for n in operands {
            operand_nums.push(n.parse().unwrap());
        }

        if find_combination(total, operand_nums[0], &operand_nums, 0) {
            sum += total;
        }
    }

    println!("{sum}");
}

fn find_combination(total: i64, temp_total: i64, operand_nums: &Vec<i64>, index: usize) -> bool {
    if index == operand_nums.len() - 1 {
        return total == temp_total;
    }

    return find_combination(
        total,
        temp_total + operand_nums[index + 1],
        &operand_nums,
        index + 1,
    ) || find_combination(
        total,
        temp_total * operand_nums[index + 1],
        &operand_nums,
        index + 1,
    ) || find_combination(
        total,
        temp_total * find_scale(operand_nums[index + 1]) + operand_nums[index + 1],
        &operand_nums,
        index + 1,
    );
}

fn find_scale(mut num: i64) -> i64 {
    let mut scale = 10;
    num = num / 10;
    while num != 0 {
        num = num / 10;
        scale *= 10;
    }

    return scale;
}
