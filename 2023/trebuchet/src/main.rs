use regex::Regex;
use std::{env, fs};

fn parse_digit(digit: &str) -> i64 {
    let v = match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        y => y.parse().unwrap(),
    };

    return v;
}

fn get_digits(calibration_value: &str) -> i64 {
    let calibration = calibration_value
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    let re = Regex::new("[0-9]").unwrap();
    let digits = re.find_iter(&calibration);
    let digits: Vec<i64> = digits.map(|d| parse_digit(d.as_str())).collect();

    return digits.first().unwrap() * 10 + digits.last().unwrap();
}

fn parse_input(input: &str, replace: bool) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if replace {
                line.to_string()
                    .replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine")
            } else {
                line.to_string()
            }
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap();
    let raw_values = fs::read_to_string(file_path).unwrap();
    let calibration_values: i64 = raw_values.lines().map(|l| get_digits(l)).sum();
    dbg!(calibration_values);
}

#[test]
fn test_get_digits() {
    let test_cases = vec![
        ("onetwothree", 13),
        ("one23", 13),
        ("seventeeneight", 78),
        ("threefourrjshnlzm9nineeightdzllhjhl8gdtqmb", 38),
        ("9cqhrtwo1five12m", 92),
        ("pb89hzbnxgbrrd", 89),
        ("jreight1five", 85),
        ("four18pmcrdbkcdcjcsix2fhssbzjjjvzhnjjk", 42),
        ("rn4sjbjtn", 44),
        ("vbt3kmkskgn5l", 35),
        ("threexsrfcqdlll9bfjk", 39),
        ("zfourninekjztvzphtc9eight6", 46),
        ("75three1six67", 77),
        ("eight73seven41fsxmkdqd", 81),
        ("threeeightlccnztnrfx2mtqvfcqtjx9lfnkmldpm", 39),
        ("14seven92sevensixxzrmhtchqk", 16),
        ("1nhjllhkglzseven6kfqfszkfgb6lhfljnspj", 16),
    ];
    for (input, expected) in test_cases {
        assert!(
            get_digits(input) == expected,
            "{}",
            format!(
                "input {} did not match the expected output {}",
                input, expected
            )
        );
    }
}
#[test]
fn test_valid_outputs() {
    let file_path = "./src/input.txt";
    let raw_values = fs::read_to_string(file_path).unwrap();
    raw_values.lines().for_each(|l| {
        assert!(
            10 < get_digits(l) && 100 > get_digits(l),
            "{}",
            format!("{} produced invalid output: {}", l, get_digits(l))
        )
    });
}

#[test]
fn test_against_answer() {
    let file_path = "./src/input.txt";
    let raw_values = fs::read_to_string(file_path).unwrap();
    raw_values.lines().for_each(|l| {
        assert!(
            get_digits(l) == parse_input(l, true) as i64,
            "{}",
            format!("{} produced invalid output: {}", l, get_digits(l))
        )
    });
}
