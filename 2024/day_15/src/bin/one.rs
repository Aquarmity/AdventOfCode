enum FacingDirection {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let mut input_iter = input.split("\n\n");
    let map_input = input_iter.next().unwrap();
    let instructions: Vec<char> = input_iter
        .next()
        .unwrap()
        .replace("\n", "")
        .chars()
        .collect();

    let mut map: Vec<Vec<char>> = vec![];
    for line in map_input.lines() {
        map.push(line.chars().collect());
    }

    let mut start_pos: (usize, usize) = (0, 0);
    for i in 0..map.len() {
        for j in 0..map.len() {
            if map[i][j] == '@' {
                start_pos = (i, j);
                map[i][j] = '.';
            }
        }
    }

    let mut cur_pos = start_pos;
    for instr in instructions {
        robot_move(&mut map, &mut cur_pos, instr);
    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                sum += 100 * i + j;
            }
        }
    }

    println!("{sum}");
}

fn robot_move(map: &mut Vec<Vec<char>>, cur_pos: &mut (usize, usize), instr: char) {
    match instr {
        '^' => match map[cur_pos.0 - 1][cur_pos.1] {
            '.' => {
                cur_pos.0 -= 1;
            }

            'O' => {
                if push_blocks(map, (cur_pos.0 - 1, cur_pos.1), FacingDirection::UP) {
                    cur_pos.0 -= 1;
                    map[cur_pos.0][cur_pos.1] = '.';
                }
            }

            _ => {}
        },

        '>' => match map[cur_pos.0][cur_pos.1 + 1] {
            '.' => {
                cur_pos.1 += 1;
            }

            'O' => {
                if push_blocks(map, (cur_pos.0, cur_pos.1 + 1), FacingDirection::RIGHT) {
                    cur_pos.1 += 1;
                    map[cur_pos.0][cur_pos.1] = '.';
                }
            }

            _ => {}
        },

        'v' => match map[cur_pos.0 + 1][cur_pos.1] {
            '.' => {
                cur_pos.0 += 1;
            }

            'O' => {
                if push_blocks(map, (cur_pos.0 + 1, cur_pos.1), FacingDirection::DOWN) {
                    cur_pos.0 += 1;
                    map[cur_pos.0][cur_pos.1] = '.';
                }
            }

            _ => {}
        },

        '<' => match map[cur_pos.0][cur_pos.1 - 1] {
            '.' => {
                cur_pos.1 -= 1;
            }

            'O' => {
                if push_blocks(map, (cur_pos.0, cur_pos.1 - 1), FacingDirection::LEFT) {
                    cur_pos.1 -= 1;
                    map[cur_pos.0][cur_pos.1] = '.';
                }
            }

            _ => {}
        },

        _ => {}
    }
}

fn push_blocks(map: &mut Vec<Vec<char>>, box_pos: (usize, usize), dir: FacingDirection) -> bool {
    match dir {
        FacingDirection::UP => match map[box_pos.0 - 1][box_pos.1] {
            '.' => {
                map[box_pos.0 - 1][box_pos.1] = 'O';
                return true;
            }

            'O' => return push_blocks(map, (box_pos.0 - 1, box_pos.1), dir),

            _ => {
                return false;
            }
        },

        FacingDirection::DOWN => match map[box_pos.0 + 1][box_pos.1] {
            '.' => {
                map[box_pos.0 + 1][box_pos.1] = 'O';
                return true;
            }

            'O' => return push_blocks(map, (box_pos.0 + 1, box_pos.1), dir),

            _ => {
                return false;
            }
        },

        FacingDirection::LEFT => match map[box_pos.0][box_pos.1 - 1] {
            '.' => {
                map[box_pos.0][box_pos.1 - 1] = 'O';
                return true;
            }

            'O' => return push_blocks(map, (box_pos.0, box_pos.1 - 1), dir),

            _ => {
                return false;
            }
        },

        FacingDirection::RIGHT => match map[box_pos.0][box_pos.1 + 1] {
            '.' => {
                map[box_pos.0][box_pos.1 + 1] = 'O';
                return true;
            }

            'O' => return push_blocks(map, (box_pos.0, box_pos.1 + 1), dir),

            _ => {
                return false;
            }
        },
    }
}
