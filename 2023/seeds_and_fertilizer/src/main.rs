use regex::Regex;
use std::cmp::Ordering;
use std::collections::hash_map::HashMap;
use std::env;
use std::fs;
use std::usize;

fn make_map_from_range(source_start: u32, dest_start: u32, range: u32) -> HashMap<u32, u32> {
    let mut result = HashMap::new();
    for i in 0..range {
        result.insert(source_start + i, dest_start + i);
    }
    return result;
}
fn make_single_map(raw_map: &str) -> HashMap<u32, u32> {
    println!("{}", raw_map);
    let mut map = HashMap::new();
    let values: Vec<&str> = raw_map.split_whitespace().collect();
    assert!(values.len() % 3 == 0);
    let mut i = 0;
    while i < values.len() {
        println!("{}", i);
        let source_start = values[i + 1 as usize].trim().parse().unwrap();
        let dest_start = values[i as usize].trim().parse().unwrap();
        let range = values[i + 2 as usize].trim().parse().unwrap();
        i += 3;
        let new_map = make_map_from_range(source_start, dest_start, range);
        map.extend(new_map.into_iter().map(|(k, v)| (k.clone(), v.clone())));
    }
    return map;
}

fn make_maps(raw_map_data: &str) -> Vec<HashMap<u32, u32>> {
    let mut maps = Vec::new();
    let re = Regex::new(r"(\S+) map:").unwrap();
    let splits: Vec<&str> = re.split(raw_map_data).collect();
    dbg!(splits.len());
    for split in splits {
        dbg!(split);
        if !split.contains("-") {
            maps.push(make_single_map(split))
        }
    }
    return maps;
}

fn parse_seeds(raw_seeds: &str) -> Vec<u32> {
    let re = Regex::new(r"[0-9]+").unwrap();
    let seeds = re
        .find_iter(raw_seeds)
        .map(|s| s.as_str().trim().parse().unwrap())
        .collect();
    return seeds;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap();
    let raw_seed_info = fs::read_to_string(file_path).expect("Cannot read file");
    let seeds = raw_seed_info.lines().next().unwrap();
    dbg!(seeds);
    let map_info = raw_seed_info.strip_prefix(seeds).unwrap();
    let seeds = parse_seeds(seeds);
    let maps = make_maps(map_info);
    let mut lowest_spot: u32 = u32::MAX;
    for mut seed in seeds {
        for map in &maps {
            dbg!(seed);
            seed = match map.get(&seed) {
                Some(num) => *num,
                None => seed,
            };
        }
        lowest_spot = match lowest_spot.cmp(&seed) {
            Ordering::Greater => seed,
            _ => lowest_spot,
        };
    }
    println!("Lowest seed = {}", lowest_spot);
}
