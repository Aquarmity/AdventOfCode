fn main() {
    let input = include_str!("./input.txt");
    let mut sum: i32 = 0;
    let mut cur_pos = 50;

    for line in input.lines() {
        let (dir, mag) = line.split_at(1);

        let last_pos = cur_pos;
        match dir {
            "L" => {
                cur_pos -= mag.parse::<i32>().unwrap();
                if cur_pos <= 0 {
                    sum += -cur_pos / 100 + 1;
                    if last_pos == 0 {
                        sum -= 1;
                    }
                }
            }
            "R" => {
                cur_pos += mag.parse::<i32>().unwrap();
                if cur_pos >= 100 {
                    sum += cur_pos / 100;
                }
            }
            _ => {}
        }

        cur_pos = cur_pos.rem_euclid(100);
    }

    println!("{sum}");
}
