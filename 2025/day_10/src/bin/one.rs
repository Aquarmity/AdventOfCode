use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let mut sum = 0;

    for line in input.lines() {
        let diagram_re = Regex::new(r"[\.#]+").unwrap();
        let diagram = diagram_re.find(&line).unwrap().as_str();

        let mut diagram_as_bits: u16 = 0;
        for (i, c) in diagram.char_indices() {
            if c == '#' {
                diagram_as_bits += 2u16.pow(i as u32);
            }
        }

        let buttons_re = Regex::new(r"\((?<w>[\d,]+)").unwrap();
        let buttons: Vec<u16> = buttons_re
            .captures_iter(&line)
            .map(|caps| {
                let mut toggle: u16 = 0;
                for n in caps.name("w").unwrap().as_str().split(',') {
                    toggle += 2u16.pow(n.parse().unwrap());
                }
                toggle
            })
            .collect();

        for i in 0..buttons.len() {
            let result = configure_indicator_lights(0, diagram_as_bits, &buttons, i + 1);
            if result != u32::MAX {
                sum += result;
                break;
            }
        }
        println!("{sum}");
    }

    println!("{sum}");
}

fn configure_indicator_lights(
    current_state: u16,
    diagram: u16,
    buttons: &Vec<u16>,
    max_depth: usize,
) -> u32 {
    if max_depth == 0 {
        return u32::MAX;
    }

    for button in buttons {
        if current_state ^ button == diagram {
            return 1;
        }
    }

    let mut min_presses: u32 = u32::MAX;
    for button in buttons {
        let config =
            configure_indicator_lights(current_state ^ button, diagram, buttons, max_depth - 1);

        if config < min_presses {
            min_presses = config + 1;
        }
    }

    min_presses
}
