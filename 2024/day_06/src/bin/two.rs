fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;
    let mut grid: Vec<&str> = input.split('\n').collect();
    let mut visited_pos: Vec<(usize, usize, FacingDirection)> = vec![];

    #[derive(PartialEq, Clone)]
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

    // I don't know why this is an issue???
    grid.remove(130);

    let dim_y = grid.len();
    let dim_x = grid[0].len();
    let start_pos = cur_pos.clone();
    let mut original_path: Vec<(usize, usize)> = vec![];

    // this can be rewritten so much better but it WORKED SO WHO CARES
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i].as_bytes()[j] == b'^'
                || grid[i].as_bytes()[j] == b'#'
                || (!original_path.contains(&(i, j)) && (i, j) != (0, 0))
            {
                continue;
            }
            cur_pos = start_pos.clone();
            visited_pos.push(cur_pos.clone());
            while cur_pos.0 > 0 && cur_pos.0 < dim_y - 1 && cur_pos.1 > 0 && cur_pos.1 < dim_x - 1 {
                println!("{}, {}, {}", i, j, sum);
                match &cur_pos.2 {
                    FacingDirection::UP => {
                        if grid[cur_pos.0 - 1].as_bytes()[cur_pos.1] != b'#'
                            && (cur_pos.0 - 1, cur_pos.1) != (i, j)
                        {
                            cur_pos.0 -= 1;
                            if visited_pos.contains(&cur_pos) {
                                sum += 1;
                                break;
                            }
                            visited_pos.push(cur_pos.clone());
                        } else {
                            cur_pos.2 = FacingDirection::RIGHT;
                        }
                    }

                    FacingDirection::DOWN => {
                        if grid[cur_pos.0 + 1].as_bytes()[cur_pos.1] != b'#'
                            && (cur_pos.0 + 1, cur_pos.1) != (i, j)
                        {
                            cur_pos.0 += 1;
                            if visited_pos.contains(&cur_pos) {
                                sum += 1;
                                break;
                            }
                            visited_pos.push(cur_pos.clone());
                        } else {
                            cur_pos.2 = FacingDirection::LEFT;
                        }
                    }

                    FacingDirection::LEFT => {
                        if grid[cur_pos.0].as_bytes()[cur_pos.1 - 1] != b'#'
                            && (cur_pos.0, cur_pos.1 - 1) != (i, j)
                        {
                            cur_pos.1 -= 1;
                            if visited_pos.contains(&cur_pos) {
                                sum += 1;
                                break;
                            }
                            visited_pos.push(cur_pos.clone());
                        } else {
                            cur_pos.2 = FacingDirection::UP;
                        }
                    }

                    FacingDirection::RIGHT => {
                        if grid[cur_pos.0].as_bytes()[cur_pos.1 + 1] != b'#'
                            && (cur_pos.0, cur_pos.1 + 1) != (i, j)
                        {
                            cur_pos.1 += 1;
                            if visited_pos.contains(&cur_pos) {
                                sum += 1;
                                break;
                            }
                            visited_pos.push(cur_pos.clone());
                        } else {
                            cur_pos.2 = FacingDirection::DOWN;
                        }
                    }
                }
            }

            if (i, j) == (0, 0) {
                for pos in &visited_pos {
                    original_path.push((pos.0, pos.1));
                }
            }

            visited_pos.clear();
        }
    }

    println!("{sum}");
}
