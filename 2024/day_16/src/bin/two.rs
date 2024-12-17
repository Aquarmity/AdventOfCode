use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::{HashMap, VecDeque},
};

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;
    let mut maze: Vec<Vec<char>> = vec![];

    for line in input.lines() {
        maze.push(line.chars().collect());
    }

    let mut end_pos = (-1, -1);
    let mut unvisited_nodes: PriorityQueue<(i32, i32), Reverse<i32>> = PriorityQueue::new();
    let mut node_dirs: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut visited_nodes: HashMap<(i32, i32), i32> = HashMap::new();
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] == 'S' {
                unvisited_nodes.push((i as i32, j as i32), Reverse(0));
                node_dirs.insert((i as i32, j as i32), (0, 1));
            }

            if maze[i][j] == '.' {
                unvisited_nodes.push((i as i32, j as i32), Reverse(i32::MAX));
            }

            if maze[i][j] == 'E' {
                maze[i][j] = '.';
                end_pos = (i as i32, j as i32);
                unvisited_nodes.push((i as i32, j as i32), Reverse(i32::MAX));
            }
        }
    }

    while !unvisited_nodes.is_empty() {
        let cur_node = unvisited_nodes.pop();
        match cur_node {
            Some(node) => {
                let ((x, y), priority) = node;

                let cur_dir = *node_dirs.get(&(x, y)).unwrap();
                visited_nodes.insert((x, y), priority.0);
                maze[x as usize][y as usize] = 'O';

                if maze[(x + cur_dir.0) as usize][(y + cur_dir.1) as usize] == '.' {
                    if priority.0 + 1
                        < unvisited_nodes
                            .get_priority(&(x + cur_dir.0, y + cur_dir.1))
                            .unwrap()
                            .0
                    {
                        unvisited_nodes.change_priority(
                            &(x + cur_dir.0, y + cur_dir.1),
                            Reverse(priority.0 + 1),
                        );
                        node_dirs.insert((x + cur_dir.0, y + cur_dir.1), (cur_dir.0, cur_dir.1));
                    }
                }
                if maze[(x + cur_dir.1) as usize][(y + cur_dir.0) as usize] == '.' {
                    if priority.0 + 1001
                        < unvisited_nodes
                            .get_priority(&(x + cur_dir.1, y + cur_dir.0))
                            .unwrap()
                            .0
                    {
                        unvisited_nodes.change_priority(
                            &(x + cur_dir.1, y + cur_dir.0),
                            Reverse(priority.0 + 1001),
                        );
                        node_dirs.insert((x + cur_dir.1, y + cur_dir.0), (cur_dir.1, cur_dir.0));
                    }
                }
                if maze[(x - cur_dir.1) as usize][(y - cur_dir.0) as usize] == '.' {
                    if priority.0 + 1001
                        < unvisited_nodes
                            .get_priority(&(x - cur_dir.1, y - cur_dir.0))
                            .unwrap()
                            .0
                    {
                        unvisited_nodes.change_priority(
                            &(x - cur_dir.1, y - cur_dir.0),
                            Reverse(priority.0 + 1001),
                        );
                        node_dirs.insert((x - cur_dir.1, y - cur_dir.0), (-cur_dir.1, -cur_dir.0));
                    }
                }
            }

            None => {}
        }
    }

    let mut backwards_walk: VecDeque<(i32, i32)> = VecDeque::new();
    let mut backwalk_dirs: VecDeque<(i32, i32)> = VecDeque::new();
    if *visited_nodes.get(&(end_pos.0 + 1, end_pos.1)).unwrap()
        == *visited_nodes.get(&end_pos).unwrap() - 1
    {
        backwalk_dirs.push_back((1, 0));
    }
    backwards_walk.push_back(end_pos);
    while !backwards_walk.is_empty() {
        let cur_pos = backwards_walk.pop_front().unwrap();
        let cur_points = *visited_nodes.get(&cur_pos).unwrap();
        let (x, y) = backwalk_dirs.pop_front().unwrap();

        if maze[cur_pos.0 as usize][cur_pos.1 as usize] == 'O' {
            if visited_nodes.contains_key(&(cur_pos.0 + x, cur_pos.1 + y)) {
                if *visited_nodes.get(&(cur_pos.0 + x, cur_pos.1 + y)).unwrap() == cur_points - 1
                    || *visited_nodes.get(&(cur_pos.0 + x, cur_pos.1 + y)).unwrap()
                        == cur_points - 1001
                    || *visited_nodes.get(&(cur_pos.0 + x, cur_pos.1 + y)).unwrap()
                        == cur_points + 999
                {
                    backwards_walk.push_back((cur_pos.0 + x, cur_pos.1 + y));
                    backwalk_dirs.push_back((x, y));
                }
            }
            for (mx, my) in vec![(1, 1), (-1, -1)] {
                if visited_nodes.contains_key(&(cur_pos.0 + y * my, cur_pos.1 + x * mx)) {
                    if *visited_nodes
                        .get(&(cur_pos.0 + y * my, cur_pos.1 + x * mx))
                        .unwrap()
                        == cur_points - 1
                    {
                        backwards_walk.push_back((cur_pos.0 + y * my, cur_pos.1 + x * mx));
                        backwalk_dirs.push_back((y * my, x * mx));
                    }
                }
            }

            maze[cur_pos.0 as usize][cur_pos.1 as usize] = '.';
            sum += 1;
        }
    }

    println!("{sum}");
}
