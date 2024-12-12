use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum FacingDirection {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

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
                let mut perimeter: Vec<(usize, usize, FacingDirection)> = vec![];
                let area = find_sides_and_area(&mut map, i, j, &mut visited_pos, &mut perimeter);

                let mut sides = 0;
                while !perimeter.is_empty() {
                    let (y, x, dir) = perimeter[0];
                    sides += 1;
                    if dir == FacingDirection::UP || dir == FacingDirection::DOWN {
                        if x > 0 {
                            let mut x1 = x - 1;
                            while perimeter.contains(&(y, x1, dir)) {
                                perimeter.remove(
                                    perimeter.iter().position(|a| *a == (y, x1, dir)).unwrap(),
                                );
                                if x1 > 0 {
                                    x1 -= 1;
                                } else {
                                    break;
                                }
                            }
                        }
                        if x < map[i].len() - 1 {
                            let mut x1 = x + 1;
                            while perimeter.contains(&(y, x1, dir)) {
                                perimeter.remove(
                                    perimeter.iter().position(|a| *a == (y, x1, dir)).unwrap(),
                                );
                                if x1 < map[i].len() - 1 {
                                    x1 += 1;
                                } else {
                                    break;
                                }
                            }
                        }
                    } else {
                        if y > 0 {
                            let mut y1 = y - 1;
                            while perimeter.contains(&(y1, x, dir)) {
                                perimeter.remove(
                                    perimeter.iter().position(|a| *a == (y1, x, dir)).unwrap(),
                                );
                                if y1 > 0 {
                                    y1 -= 1;
                                } else {
                                    break;
                                }
                            }
                        }
                        if y < map[i].len() - 1 {
                            let mut y1 = y + 1;
                            while perimeter.contains(&(y1, x, dir)) {
                                perimeter.remove(
                                    perimeter.iter().position(|a| *a == (y1, x, dir)).unwrap(),
                                );
                                if y1 < map[i].len() - 1 {
                                    y1 += 1;
                                } else {
                                    break;
                                }
                            }
                        }
                    }

                    perimeter.remove(0);
                }

                sum += sides * area;
            }
        }
    }

    println!("{sum}");
}

fn find_sides_and_area(
    map: &mut Vec<Vec<char>>,
    i: usize,
    j: usize,
    visited_pos: &mut HashSet<(usize, usize)>,
    perimeter: &mut Vec<(usize, usize, FacingDirection)>,
) -> i32 {
    let mut area = 1;
    let region_char = map[i][j];

    map[i][j] = ' ';
    visited_pos.insert((i, j));

    if i > 0 {
        if map[i - 1][j] == region_char {
            let result = find_sides_and_area(map, i - 1, j, visited_pos, perimeter);
            area += result;
        } else if !visited_pos.contains(&(i - 1, j)) {
            perimeter.push((i, j, FacingDirection::UP));
        }
    } else {
        perimeter.push((i, j, FacingDirection::UP));
    }
    if i < map.len() - 1 {
        if map[i + 1][j] == region_char {
            let result = find_sides_and_area(map, i + 1, j, visited_pos, perimeter);
            area += result;
        } else if !visited_pos.contains(&(i + 1, j)) {
            perimeter.push((i, j, FacingDirection::DOWN));
        }
    } else {
        perimeter.push((i, j, FacingDirection::DOWN));
    }
    if j > 0 {
        if map[i][j - 1] == region_char {
            let result = find_sides_and_area(map, i, j - 1, visited_pos, perimeter);
            area += result;
        } else if !visited_pos.contains(&(i, j - 1)) {
            perimeter.push((i, j, FacingDirection::LEFT));
        }
    } else {
        perimeter.push((i, j, FacingDirection::LEFT));
    }
    if j < map[i].len() - 1 {
        if map[i][j + 1] == region_char {
            let result = find_sides_and_area(map, i, j + 1, visited_pos, perimeter);
            area += result;
        } else if !visited_pos.contains(&(i, j + 1)) {
            perimeter.push((i, j, FacingDirection::RIGHT));
        }
    } else {
        perimeter.push((i, j, FacingDirection::RIGHT));
    }

    return area;
}
