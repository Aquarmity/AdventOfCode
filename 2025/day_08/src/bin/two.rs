use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("./input.txt");
    let mut box_coordinates: Vec<(i64, i64, i64)> = vec![];
    let mut distances: BTreeMap<OrderedFloat<f32>, (usize, usize)> = BTreeMap::new();

    for line in input.lines() {
        let mut line_iter = line.splitn(3, ',');
        box_coordinates.push((
            line_iter.next().unwrap().parse().unwrap(),
            line_iter.next().unwrap().parse().unwrap(),
            line_iter.next().unwrap().parse().unwrap(),
        ));
    }

    for i in 0..box_coordinates.len() {
        for j in (i + 1)..box_coordinates.len() {
            let distance = (((box_coordinates[i].0 - box_coordinates[j].0).pow(2)
                + (box_coordinates[i].1 - box_coordinates[j].1).pow(2)
                + (box_coordinates[i].2 - box_coordinates[j].2).pow(2))
                as f32)
                .sqrt();

            distances.insert(OrderedFloat(distance), (i, j));
        }
    }

    let distance_pairs: Vec<&(usize, usize)> = distances.values().collect();

    let mut circuit: BTreeSet<usize> = BTreeSet::new();
    let mut disconnected_boxes: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut distances_iter = distance_pairs.iter();
    let mut next_pair = distances_iter.next().unwrap();

    circuit.insert(next_pair.0);
    circuit.insert(next_pair.1);

    while circuit.len() < box_coordinates.len() {
        next_pair = distances_iter.next().unwrap();
        if try_insert(&mut circuit, **next_pair) {
            for pair in &disconnected_boxes {
                if circuit.contains(&pair.0) {
                    circuit.insert(pair.1);
                } else if circuit.contains(&pair.1) {
                    circuit.insert(pair.0);
                }
            }
        } else {
            disconnected_boxes.insert(**next_pair);
        }
    }

    println!(
        "{}",
        box_coordinates[next_pair.0].0 * box_coordinates[next_pair.1].0,
    );
}

fn try_insert(circuit: &mut BTreeSet<usize>, next_pair: (usize, usize)) -> bool {
    match (
        circuit.contains(&next_pair.0),
        circuit.contains(&next_pair.1),
    ) {
        (true, true) => true,
        (true, false) => {
            circuit.insert(next_pair.1);
            true
        }
        (false, true) => {
            circuit.insert(next_pair.0);
            true
        }
        (false, false) => false,
    }
}
