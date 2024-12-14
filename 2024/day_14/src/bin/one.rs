use regex::Regex;

struct Robot {
    v: (i32, i32),
    p: (i32, i32),
}

const X_DIM: i32 = 101;
const Y_DIM: i32 = 103;

fn main() {
    let input = include_str!("./input.txt");

    let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>[-|\d]+),(?<vy>[-|\d]+)").unwrap();

    let mut robots: Vec<Robot> = re
        .captures_iter(&input)
        .map(|caps| {
            let temp = Robot {
                p: (
                    caps.name("px").unwrap().as_str().parse().unwrap(),
                    caps.name("py").unwrap().as_str().parse().unwrap(),
                ),
                v: (
                    caps.name("vx").unwrap().as_str().parse().unwrap(),
                    caps.name("vy").unwrap().as_str().parse().unwrap(),
                ),
            };

            temp
        })
        .collect();

    for robot in &mut robots {
        if robot.v.0 >= 0 {
            robot.p.0 = (robot.p.0 + (robot.v.0 * 100)) % X_DIM;
        } else {
            let mut dist_from_right_wall = (X_DIM - 1) - robot.p.0;
            dist_from_right_wall = (dist_from_right_wall + (robot.v.0 * 100).abs()) % X_DIM;
            robot.p.0 = (X_DIM - 1) - dist_from_right_wall;
        }

        if robot.v.1 >= 0 {
            robot.p.1 = (robot.p.1 + (robot.v.1 * 100)) % Y_DIM;
        } else {
            let mut dist_from_bottom_wall = (Y_DIM - 1) - robot.p.1;
            dist_from_bottom_wall = (dist_from_bottom_wall + (robot.v.1 * 100).abs()) % Y_DIM;
            robot.p.1 = (Y_DIM - 1) - dist_from_bottom_wall;
        }
    }

    let mut quadrant_sums: (i32, i32, i32, i32) = (0, 0, 0, 0);
    for robot in robots {
        if robot.p.0 < X_DIM / 2 && robot.p.1 < Y_DIM / 2 {
            quadrant_sums.0 += 1;
        }
        if robot.p.0 < X_DIM / 2 && robot.p.1 > Y_DIM / 2 {
            quadrant_sums.1 += 1;
        }
        if robot.p.0 > X_DIM / 2 && robot.p.1 < Y_DIM / 2 {
            quadrant_sums.2 += 1;
        }
        if robot.p.0 > X_DIM / 2 && robot.p.1 > Y_DIM / 2 {
            quadrant_sums.3 += 1;
        }
    }

    let sum = quadrant_sums.0 * quadrant_sums.1 * quadrant_sums.2 * quadrant_sums.3;

    println!("{sum}");
}
