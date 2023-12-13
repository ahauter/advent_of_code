use num::Integer;
use std::{collections::HashMap, env, fs};

#[derive(Debug)]
struct NodeDirections<'a> {
    left: &'a str,
    right: &'a str,
}

fn build_map(raw_map: &str) -> HashMap<&str, NodeDirections> {
    HashMap::from_iter(raw_map.lines().filter(|l| l.len() > 0).map(|raw_node| {
        dbg!(&raw_node);
        (
            &raw_node[0..3],
            NodeDirections {
                left: &raw_node[7..10],
                right: &raw_node[12..15],
            },
        )
    }))
}

fn get_num_cycles(position: &str, directions: &str, map: &HashMap<&str, NodeDirections>) -> u64 {
    let mut position = position.clone();
    let mut steps = 0;
    while !position.ends_with("Z") {
        for direction in directions.chars() {
            let posible_dirs = map.get(position).unwrap();
            position = match direction {
                'R' => posible_dirs.right,
                'L' => posible_dirs.left,
                x => panic!("Not a valid direction: {}!", x),
            };
            steps += 1;
            if position.ends_with("Z") {
                break;
            }
        }
    }
    return steps;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap();
    let raw_values = fs::read_to_string(file_path).unwrap();
    let directions: &str = raw_values.lines().collect::<Vec<&str>>().first().unwrap();
    let raw_map = &raw_values[directions.len()..];
    let map = build_map(raw_map);
    let min_steps: Vec<u64> = map
        .keys()
        .filter(|pos| pos.ends_with("A"))
        .map(|starting_pos| get_num_cycles(&starting_pos, &directions, &map))
        .collect();
    let mut steps = 1;
    for min_step in min_steps {
        steps = steps.lcm(&min_step);
    }
    dbg!(steps);
}
