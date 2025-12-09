use std::cmp;

fn main() {
    let input = include_str!("./input.txt")
        .split_whitespace()
        .map(|s| s.split_once(',').unwrap());

    let red_tiles: Vec<(i64, i64)> = input
        .map(|xy| (xy.0.parse().unwrap(), xy.1.parse().unwrap()))
        .collect();

    let mut max_area: i64 = 0;
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let area = valid_rect_area(i, j, &red_tiles);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{max_area}");
}

fn valid_rect_area(i: usize, j: usize, red_tiles: &Vec<(i64, i64)>) -> i64 {
    let (one, two) = (
        cmp::max(red_tiles[i], red_tiles[j]),
        cmp::min(red_tiles[i], red_tiles[j]),
    );
    let (x1, y1) = one;
    let (x2, y2) = two;
    let (ymax, ymin) = (cmp::max(y1, y2), cmp::min(y1, y2));

    if red_tiles
        .iter()
        .filter(|(x, y)| *x > x2 && *x < x1 && *y > ymin && *y < ymax)
        .count()
        > 0
    {
        return 0;
    }
    for i in 0..red_tiles.len() {
        let current = red_tiles[i];
        if current.0 > x2 && current.0 < x1 {
            let next = red_tiles[(i + 1) % red_tiles.len()];
            if next.0 == current.0
                && cmp::max(current.1, next.1) >= ymax
                && cmp::min(current.1, next.1) <= ymin
            {
                return 0;
            }
        }
        if current.1 > ymin && current.1 < ymax {
            let next = red_tiles[(i + 1) % red_tiles.len()];
            if next.1 == current.1
                && cmp::max(current.0, next.0) >= x1
                && cmp::min(current.0, next.0) <= x2
            {
                return 0;
            }
        }
    }

    (x1 - x2 + 1) * (ymax - ymin + 1)
}
