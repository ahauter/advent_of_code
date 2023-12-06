use std::collections::VecDeque;
use std::io;
use std::io::BufRead;

fn get_num_matches(winning_numbers: Vec<i32>, personal_numbers: Vec<i32>) -> u32 {
    let mut winning_number_count = 0;
    for num in winning_numbers {
        if personal_numbers.contains(&num) {
            winning_number_count += 1;
        }
    }
    return winning_number_count;
}

fn calculate_score(winning_numbers: Vec<i32>, personal_numbers: Vec<i32>) -> i32 {
    let winning_number_count = get_num_matches(winning_numbers, personal_numbers);
    let base: i32 = 2;
    let result = base.pow(winning_number_count - 1);
    return result;
}

fn remove_option_string(s: Option<&str>) -> &str {
    match s {
        Some(value) => {
            return value;
        }
        None => return "",
    }
}

fn remove_card_prefix(scratch_card_info: &String) -> &str {
    let mut card_strings = scratch_card_info.split(":");
    card_strings.next();
    let card = card_strings.next();
    return remove_option_string(card);
}

fn split_winning_numbers(raw_scratch_card_numbers: &str) -> (&str, &str) {
    let mut number_strings = raw_scratch_card_numbers.split("|");
    let winning_num = number_strings.next();
    let personal_num = number_strings.next();
    return (
        remove_option_string(winning_num),
        remove_option_string(personal_num),
    );
}

fn parse_numbers(raw_numbers: &str) -> Vec<i32> {
    let numbers: Vec<i32> = raw_numbers
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    return numbers;
}

fn main() {
    let mut total = 0;
    let mut scratch_card_raw = String::from("0");
    let mut multi_card = VecDeque::new();
    multi_card.push_back(0);
    while scratch_card_raw.len() > 0 {
        if scratch_card_raw == "end" {
            break;
        }
        scratch_card_raw = io::stdin().lock().lines().next().unwrap().unwrap();
        let card_numbers = remove_card_prefix(&scratch_card_raw);
        let (winning, personal) = split_winning_numbers(card_numbers);
        let (winning, personal) = (parse_numbers(winning), parse_numbers(personal));
        let num_current_card = multi_card.pop_front();
        let num_current_card = 1 + match num_current_card {
            Some(num) => num,
            None => 0,
        };
        println!("");
        println!("{}", num_current_card);
        total += num_current_card;
        let num_matches = get_num_matches(winning, personal);
        for i in 0..num_matches {
            match multi_card.get(i as usize) {
                Some(_) => multi_card[i as usize] += num_current_card,
                None => multi_card.push_back(num_current_card),
            }
        }
        for num in &multi_card {
            print!("{}, ", num);
        }
    }

    println!("{}", total)
}
