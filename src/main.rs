use aoc2024::*;
use std::io::{self, Read};

fn main() {
    let day = std::env::args().nth(1).expect("must provide a day e.g. 1");
    // TODO: Macro for this?
    let run = match day.as_ref() {
        "1" => d01::run,
        _ => panic!("must provide a valid day that has been implemented e.g. 1"),
    };

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    run(&input);
}
