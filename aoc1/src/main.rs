use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let filename = env::args().nth(1).expect("file name as argument expected");

    let mut file = File::open(filename).expect("file could not be opened");

    let mut input = String::new();
    file.read_to_string(&mut input).expect("could not read file");

    let changes = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("input must be a numeral"))
        .collect::<Vec<_>>();

    let result = sum_changes(changes, 0);

    println!("result: {}", result);
}

fn sum_changes(changes: Vec<i32>, initial_frequency: i32) -> i32 {
    changes.iter().fold(initial_frequency, |acc, x| acc + x)
}
