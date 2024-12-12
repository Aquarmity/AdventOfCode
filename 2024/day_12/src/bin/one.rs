use std::collections::HashSet;

fn main() {
    let input: Vec<&str> = include_str!("./input.txt").split_whitespace().collect();
    let mut sum = 0;

    let mut map: Vec<Vec<char>> = vec![];
    for line in input {
        map.push(line.chars().collect());
    }

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != ' ' {
                let mut visited_pos: HashSet<(usize, usize)> = HashSet::new();
                let (perimeter, area) = find_perimeter_and_area(&mut map, i, j, &mut visited_pos);
                sum += perimeter * area;
            }
        }
    }

    println!("{sum}");
}

fn find_perimeter_and_area(
    map: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    visited_pos: &mut HashSet<(usize, usize)>,
) -> (i32, i32) {
    let (mut perimeter, mut area) = (0, 1);
    let region_char = map[i][j];

    map[i][j] = ' ';
    visited_pos.insert((i, j));

    if i > 0 {
        if map[i - 1][j] == region_char {
            let result = find_perimeter_and_area(map, i - 1, j, visited_pos);
            perimeter += result.0;
            area += result.1;
        } else if !visited_pos.contains(&(i - 1, j)) {
            perimeter += 1;
        }
    } else {
        perimeter += 1;
    }
    if i < map.len() - 1 {
        if map[i + 1][j] == region_char {
            let result = find_perimeter_and_area(map, i + 1, j, visited_pos);
            perimeter += result.0;
            area += result.1;
        } else if !visited_pos.contains(&(i + 1, j)) {
            perimeter += 1;
        }
    } else {
        perimeter += 1;
    }
    if j > 0 {
        if map[i][j - 1] == region_char {
            let result = find_perimeter_and_area(map, i, j - 1, visited_pos);
            perimeter += result.0;
            area += result.1;
        } else if !visited_pos.contains(&(i, j - 1)) {
            perimeter += 1;
        }
    } else {
        perimeter += 1;
    }
    if j < map[i].len() - 1 {
        if map[i][j + 1] == region_char {
            let result = find_perimeter_and_area(map, i, j + 1, visited_pos);
            perimeter += result.0;
            area += result.1;
        } else if !visited_pos.contains(&(i, j + 1)) {
            perimeter += 1;
        }
    } else {
        perimeter += 1;
    }

    return (perimeter, area);
}
