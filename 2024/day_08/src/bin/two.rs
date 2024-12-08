use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    let mut antinode_grid: Vec<Vec<i32>> = vec![vec![0; grid.len()]; grid.len()];
    let mut antennas: HashSet<char> = HashSet::new();

    for line in &grid {
        for c in line {
            if *c != '.' {
                antennas.insert(*c);
            }
        }
    }

    for c in antennas {
        let mut antenna_pos: Vec<(usize, usize)> = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == c {
                    antenna_pos.push((i, j));
                }
            }
        }

        for i in 0..antenna_pos.len() {
            for j in i + 1..antenna_pos.len() {
                let (x1, y1) = antenna_pos[i];
                let (x2, y2) = antenna_pos[j];
                let (mut cur_x, mut cur_y) = (x1, y1);
                antinode_grid[cur_x][cur_y] = 1;

                if y1 >= y2 {
                    let (dist_x, dist_y) = (x2 - x1, y1 - y2);
                    while cur_x >= dist_x && cur_y + dist_y < grid.len() {
                        antinode_grid[cur_x - dist_x][cur_y + dist_y] = 1;
                        cur_x -= dist_x;
                        cur_y += dist_y;
                    }
                    (cur_x, cur_y) = (x1, y1);
                    while cur_x + dist_x < grid.len() && cur_y >= dist_y {
                        antinode_grid[cur_x + dist_x][cur_y - dist_y] = 1;
                        cur_x += dist_x;
                        cur_y -= dist_y;
                    }
                } else {
                    let (dist_x, dist_y) = (x2 - x1, y2 - y1);
                    while cur_x >= dist_x && cur_y >= dist_y {
                        antinode_grid[cur_x - dist_x][cur_y - dist_y] = 1;
                        cur_x -= dist_x;
                        cur_y -= dist_y;
                    }
                    (cur_x, cur_y) = (x1, y1);
                    while cur_x + dist_x < grid.len() && cur_y + dist_y < grid.len() {
                        antinode_grid[cur_x + dist_x][cur_y + dist_y] = 1;
                        cur_x += dist_x;
                        cur_y += dist_y;
                    }
                }
            }
        }
    }

    for list in &antinode_grid {
        for i in list {
            sum += i;
        }
    }

    println!("{sum}");
}
