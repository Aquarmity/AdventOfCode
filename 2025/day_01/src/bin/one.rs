fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;
    let mut cur_pos = 50;

    for line in input.lines() {
        let (dir, mag) = line.split_at(1);
        match dir {
            "L" => {
                cur_pos += mag.parse::<i32>().unwrap();
            }
            "R" => {
                cur_pos -= mag.parse::<i32>().unwrap();
            }
            _ => {}
        }

        cur_pos %= 100;
        if cur_pos == 0 {
            sum += 1;
        }
    }

    println!("{sum}");
}
