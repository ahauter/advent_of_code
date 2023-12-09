use regex::Regex;
use std::cmp::Ordering;
use std::env;
use std::fs;
use std::usize;

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn overlap(&self, other: &Range) -> Option<Range> {
        if other.start >= self.end || other.end <= self.start {
            return None;
        }
        let start = match self.start.cmp(&other.start) {
            Ordering::Greater => self.start,
            _ => other.start,
        };
        let end = match self.end.cmp(&other.end) {
            Ordering::Greater => other.end,
            _ => self.end,
        };
        return Some(Range { start, end });
    }

    fn diff(&self, other: &Range) -> Vec<Range> {
        let mut result: Vec<Range> = Vec::new();
        //handle non-overlap
        if self.end < other.start || self.start > other.end {
            result.push(self.clone());
            return result;
        }
        if self.start < other.start {
            result.push(Range {
                start: self.start.clone(),
                end: other.start.clone(),
            });
        }
        if self.end > other.end {
            result.push(Range {
                start: other.end.clone(),
                end: self.end.clone(),
            });
        }
        return result;
    }
}

#[derive(Debug, Clone)]
struct RangeMap {
    range: Range,
    offset: i64,
}

impl RangeMap {
    fn apply(&self, other_ranges: &Vec<Range>) -> (Vec<Range>, Vec<Range>) {
        let mut new_ranges: Vec<Range> = Vec::new();
        let mut old_ranges: Vec<Range> = Vec::new();
        for other_range in other_ranges {
            let overlap = other_range.overlap(&self.range);
            old_ranges.append(&mut other_range.diff(&self.range));
            match overlap {
                Some(r) => new_ranges.push(Range {
                    start: r.start + self.offset,
                    end: r.end + self.offset,
                }),
                _ => (),
            };
        }
        return (new_ranges, old_ranges);
    }
}

fn apply_multiple_maps(maps: &Vec<RangeMap>, ranges: &Vec<Range>) -> Vec<Range> {
    let mut result: Vec<Range> = Vec::new();
    let mut ranges = ranges.clone();
    for map in maps {
        let (new_ranges, old_ranges) = map.apply(&ranges);
        result.extend_from_slice(&new_ranges);
        ranges = old_ranges.clone();
    }
    result.append(&mut ranges);
    return result;
}

fn make_map_from_range(source_start: i64, dest_start: i64, range: i64) -> RangeMap {
    return RangeMap {
        range: Range {
            start: source_start,
            end: source_start + range,
        },
        offset: dest_start - source_start,
    };
}

fn make_single_map_step(raw_map: &str) -> Vec<RangeMap> {
    let mut result: Vec<RangeMap> = Vec::new();
    let values: Vec<&str> = raw_map.split_whitespace().collect();
    assert!(values.len() % 3 == 0);
    let mut i = 0;
    while i < values.len() {
        let source_start = values[i + 1 as usize].trim().parse().unwrap();
        let dest_start = values[i as usize].trim().parse().unwrap();
        let range = values[i + 2 as usize].trim().parse().unwrap();
        i += 3;
        let new_map = make_map_from_range(source_start, dest_start, range);
        result.push(new_map);
    }
    return result;
}

fn make_maps(raw_map_data: &str) -> Vec<Vec<RangeMap>> {
    let mut maps = Vec::new();
    let re = Regex::new(r"(\S+) map:").unwrap();
    let splits: Vec<&str> = re.split(raw_map_data).collect();
    for split in splits {
        if !split.contains("-") {
            maps.push(make_single_map_step(split))
        }
    }
    return maps;
}

fn parse_seeds(raw_seeds: &str) -> Vec<Range> {
    let re = Regex::new(r"[0-9]+").unwrap();
    let seeds: Vec<i64> = re
        .find_iter(raw_seeds)
        .map(|s| s.as_str().trim().parse().unwrap())
        .collect();
    let mut actual_seeds = Vec::new();
    assert!(seeds.len() % 2 == 0);
    let mut i = 0;
    while i < seeds.len() {
        actual_seeds.push(Range {
            start: seeds[i],
            end: seeds[i] + seeds[i + 1],
        });
        i += 2;
    }
    return actual_seeds;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap();
    let raw_seed_info = fs::read_to_string(file_path).expect("Cannot read file");
    let seeds = raw_seed_info.lines().next().unwrap();
    let map_info = raw_seed_info.strip_prefix(seeds).unwrap();
    let mut seeds = parse_seeds(seeds);
    let maps = make_maps(map_info);
    let mut lowest_spot: i64 = i64::MAX;
    for map in &maps {
        dbg!(&seeds);
        dbg!(&map);
        seeds = apply_multiple_maps(map, &seeds);
    }
    for s in seeds {
        lowest_spot = match lowest_spot.cmp(&s.start) {
            Ordering::Greater => s.start,
            _ => lowest_spot,
        };
    }
    println!("Lowest seed = {}", lowest_spot);
}
