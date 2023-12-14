use core::cmp::max;
use std::{collections::HashMap, env, fs};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
enum CubeColor {
    Red,
    Blue,
    Green,
}

type Draw = HashMap<CubeColor, i64>;
struct Game {
    draws: Vec<Draw>,
    number: i64,
}

fn parse_draw(raw_value: &str) -> Draw {
    HashMap::from_iter(raw_value.split(",").map(|s| {
        let s: Vec<&str> = s.split_whitespace().collect();
        let color = match *s.last().unwrap() {
            "red" => CubeColor::Red,
            "green" => CubeColor::Green,
            "blue" => CubeColor::Blue,
            _ => panic!("Not the right color!"),
        };
        let value = s.first().unwrap().parse().unwrap();
        return (color, value);
    }))
}

fn parse_game(raw_value: &str) -> Game {
    let game_data: Vec<String> = raw_value.split(":").map(|s| String::from(s)).collect();
    let number: i64 = game_data
        .first()
        .unwrap()
        .to_owned()
        .strip_prefix("Game ")
        .unwrap()
        .parse()
        .unwrap();
    let draws: Vec<Draw> = game_data
        .last()
        .unwrap()
        .split(";")
        .map(|d| parse_draw(d))
        .collect();
    return Game { number, draws };
}

fn get_map_max(
    color: &CubeColor,
    map1: &HashMap<CubeColor, i64>,
    map2: &HashMap<CubeColor, i64>,
) -> i64 {
    return max(
        *map1.get(color).unwrap_or(&0),
        *map2.get(color).unwrap_or(&0),
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap();
    let raw_values = fs::read_to_string(file_path).unwrap();
    let max_values: Draw = HashMap::from([
        (CubeColor::Green, 13),
        (CubeColor::Red, 12),
        (CubeColor::Blue, 14),
    ]);
    let result: i64 = raw_values
        .lines()
        .map(|raw_game| parse_game(raw_game))
        .filter(|game| {
            game.draws.iter().all(|draw| {
                draw.into_iter()
                    .all(|color_info| max_values.get(color_info.0).unwrap() >= color_info.1)
            })
        })
        .map(|game| game.number)
        .sum();
    dbg!(result);
    let games: Vec<Game> = raw_values
        .lines()
        .map(|raw_game| parse_game(raw_game))
        .collect();
    let game_power: i64 = games
        .iter()
        .map(|game| {
            let mut max_values: Draw = HashMap::from([
                (CubeColor::Green, 0),
                (CubeColor::Red, 0),
                (CubeColor::Blue, 0),
            ]);
            game.draws.iter().for_each(|draw| {
                draw.keys().for_each(|color| {
                    let max_ref = max_values.clone();
                    if let Some(x) = max_values.get_mut(color) {
                        *x = get_map_max(color, &max_ref, draw);
                    }
                })
            });
            return max_values.values().fold(1, |a, b| a * b);
        })
        .sum();
    dbg!(game_power);
}
