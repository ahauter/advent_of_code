use std::{env, fs};
fn parse_sequence(seq: &str) -> Vec<i64> {
    seq.split(" ").map(|num| num.parse().unwrap()).collect()
}

fn reduce_sequence(seq: &Vec<i64>) -> Vec<i64> {
    let mut new_seq = Vec::new();
    for i in 1..seq.len() {
        let new_val = seq.get(i).unwrap() - seq.get(i - 1).unwrap();
        new_seq.push(new_val);
    }
    return new_seq;
}

fn get_next_element(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|num| *num == 0) {
        return 0;
    }
    let derivative = reduce_sequence(seq);
    let next_element = get_next_element(&derivative);
    return seq.last().unwrap() + next_element;
}

fn get_prev_element(seq: &Vec<i64>) -> i64 {
    if seq.iter().all(|num| *num == 0) {
        return 0;
    }
    let derivative = reduce_sequence(seq);
    let next_element = get_prev_element(&derivative);
    return seq.first().unwrap() - next_element;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).unwrap();
    let raw_input = fs::read_to_string(file_path).unwrap();
    let sequences: Vec<Vec<i64>> = raw_input.lines().map(|seq| parse_sequence(seq)).collect();
    dbg!(sequences
        .iter()
        .map(|seq| get_next_element(seq))
        .sum::<i64>());
    dbg!(sequences
        .iter()
        .map(|seq| get_prev_element(seq))
        .sum::<i64>());
}
