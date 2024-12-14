use regex::Regex;
use std::io;

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

    // step through the loop until you see it
    let mut seconds = 0;
    let mut filler = String::new();
    loop {
        let mut robot_map: Vec<Vec<i32>> =
            vec![vec![0; X_DIM.try_into().unwrap()]; Y_DIM.try_into().unwrap()];

        step_robots(&mut robots);
        for robot in &robots {
            robot_map[robot.p.1 as usize][robot.p.0 as usize] += 1;
        }

        for line in robot_map {
            for num in line {
                if num == 0 {
                    print!(".");
                } else {
                    print!("{num}")
                }
            }
            println!("");
        }

        seconds += 1;
        println!["{seconds}"];
        io::stdin()
            .read_line(&mut filler)
            .expect("Failed to read line");
    }
}

fn step_robots(robots: &mut Vec<Robot>) {
    for robot in robots {
        if robot.v.0 >= 0 {
            robot.p.0 = (robot.p.0 + robot.v.0) % X_DIM;
        } else {
            let mut dist_from_right_wall = (X_DIM - 1) - robot.p.0;
            dist_from_right_wall = (dist_from_right_wall + robot.v.0.abs()) % X_DIM;
            robot.p.0 = (X_DIM - 1) - dist_from_right_wall;
        }

        if robot.v.1 >= 0 {
            robot.p.1 = (robot.p.1 + robot.v.1) % Y_DIM;
        } else {
            let mut dist_from_bottom_wall = (Y_DIM - 1) - robot.p.1;
            dist_from_bottom_wall = (dist_from_bottom_wall + robot.v.1.abs()) % Y_DIM;
            robot.p.1 = (Y_DIM - 1) - dist_from_bottom_wall;
        }
    }
}
