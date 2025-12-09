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
            let area = rect_area(red_tiles[i], red_tiles[j]);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{max_area}");
}

fn rect_area((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)
}
