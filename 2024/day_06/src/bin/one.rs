fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;
    let grid: Vec<&str> = input.split('\n').collect();
    let mut visited_pos: Vec<(usize, usize)> = vec![];

    enum FacingDirection {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    }

    let mut cur_pos: (usize, usize, FacingDirection) = (0, 0, FacingDirection::UP);
    for line in &grid {
        if line.contains('^') {
            cur_pos.1 = line.find('^').unwrap();
            break;
        }
        cur_pos.0 += 1;
    }

    let dim_y = grid.len();
    let dim_x = grid[0].len();

    while cur_pos.0 > 0 && cur_pos.0 < dim_y - 1 && cur_pos.1 > 0 && cur_pos.1 < dim_x - 1 {
        match cur_pos.2 {
            FacingDirection::UP => {
                if grid[cur_pos.0 - 1].as_bytes()[cur_pos.1] == b'.'
                    || grid[cur_pos.0 - 1].as_bytes()[cur_pos.1] == b'^'
                {
                    if !visited_pos.contains(&(cur_pos.0, cur_pos.1)) {
                        sum += 1;
                        visited_pos.push((cur_pos.0, cur_pos.1));
                    }
                    cur_pos.0 -= 1;
                } else {
                    cur_pos.2 = FacingDirection::RIGHT;
                }
            }

            FacingDirection::DOWN => {
                if grid[cur_pos.0 + 1].as_bytes()[cur_pos.1] == b'.'
                    || grid[cur_pos.0 - 1].as_bytes()[cur_pos.1] == b'^'
                {
                    if !visited_pos.contains(&(cur_pos.0, cur_pos.1)) {
                        sum += 1;
                        visited_pos.push((cur_pos.0, cur_pos.1));
                    }
                    cur_pos.0 += 1;
                } else {
                    cur_pos.2 = FacingDirection::LEFT;
                }
            }

            FacingDirection::LEFT => {
                if grid[cur_pos.0].as_bytes()[cur_pos.1 - 1] == b'.'
                    || grid[cur_pos.0 - 1].as_bytes()[cur_pos.1] == b'^'
                {
                    if !visited_pos.contains(&(cur_pos.0, cur_pos.1)) {
                        sum += 1;
                        visited_pos.push((cur_pos.0, cur_pos.1));
                    }
                    cur_pos.1 -= 1;
                } else {
                    cur_pos.2 = FacingDirection::UP;
                }
            }

            FacingDirection::RIGHT => {
                if grid[cur_pos.0].as_bytes()[cur_pos.1 + 1] == b'.'
                    || grid[cur_pos.0 - 1].as_bytes()[cur_pos.1] == b'^'
                {
                    if !visited_pos.contains(&(cur_pos.0, cur_pos.1)) {
                        sum += 1;
                        visited_pos.push((cur_pos.0, cur_pos.1));
                    }
                    cur_pos.1 += 1;
                } else {
                    cur_pos.2 = FacingDirection::DOWN;
                }
            }
        }
    }

    sum += 1;

    println!("{sum}");
}
