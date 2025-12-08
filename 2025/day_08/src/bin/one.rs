use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("./input.txt");
    let mut product = 1;
    let mut box_coordinates: Vec<(i64, i64, i64)> = vec![];
    let num_cables: usize = 1000;
    let mut shortest_distances: BTreeMap<OrderedFloat<f32>, (usize, usize)> = BTreeMap::new();

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

            if shortest_distances.len() == num_cables {
                if distance < shortest_distances.last_key_value().unwrap().0.into_inner() {
                    shortest_distances.pop_last();
                    shortest_distances.insert(OrderedFloat(distance), (i, j));
                }
            } else {
                shortest_distances.insert(OrderedFloat(distance), (i, j));
            }
        }
    }

    let mut shortest_pairs: Vec<&(usize, usize)> = shortest_distances.values().collect();
    shortest_pairs.sort();

    let mut circuits: Vec<BTreeSet<usize>> = vec![];
    'outer: for (box1, box2) in &shortest_pairs {
        for i in 0..circuits.len() {
            if circuits[i].contains(box1) && circuits[i].contains(box2) {
                continue 'outer;
            }
        }
        let mut new_circuit: BTreeSet<usize> = BTreeSet::new();
        construct_circuit(&shortest_pairs, &mut new_circuit, (*box1, *box2));
        circuits.push(new_circuit);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    for i in 0..3 {
        product *= circuits[i].len();
    }
    println!("{product}");
}

fn construct_circuit(
    shortest_pairs: &Vec<&(usize, usize)>,
    circuit: &mut BTreeSet<usize>,
    next_pair: (usize, usize),
) {
    match (
        circuit.contains(&next_pair.0),
        circuit.contains(&next_pair.1),
    ) {
        (true, true) => return,
        (true, false) => {
            circuit.insert(next_pair.1);
            for pair in shortest_pairs
                .iter()
                .filter(|(a, b)| a == &next_pair.1 || b == &next_pair.1)
            {
                construct_circuit(&shortest_pairs, circuit, **pair);
            }
        }
        (false, true) => {
            circuit.insert(next_pair.0);
            for pair in shortest_pairs
                .iter()
                .filter(|(a, b)| a == &next_pair.0 || b == &next_pair.0)
            {
                construct_circuit(&shortest_pairs, circuit, **pair);
            }
        }
        (false, false) => {
            circuit.insert(next_pair.0);
            circuit.insert(next_pair.1);
            for pair in shortest_pairs.iter().filter(|(a, b)| {
                a == &next_pair.0 || b == &next_pair.0 || a == &next_pair.0 || b == &next_pair.1
            }) {
                construct_circuit(&shortest_pairs, circuit, **pair);
            }
        }
    }
}
