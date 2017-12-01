use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::io;
use std::process::exit;

fn get_input() -> io::Result<String> {
    let mut input = String::new();

    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        let mut file = File::open(path)?;

        file.read_to_string(&mut input)?;
    } else {
        io::stdin().read_to_string(&mut input)?;
    }

    Ok(input)
}

fn sum_consecutive_matches(input: &str) -> u32 {
    let mut digits = input.trim().chars()
                                 .filter_map(|c| { c.to_digit(10) })
                                 .peekable();

    let mut sum = 0;
    let first = *digits.peek().unwrap();
    while let Some(this) = digits.next() {
        let next = digits.peek().unwrap_or(&first);
        if this == *next {
            sum += this;
        }
    }

    sum
}

fn sum_opposite_matches(input: &str) -> u32 {
    let digits: Vec<_> = input.trim().chars()
                                     .filter_map(|c| { c.to_digit(10) })
                                     .collect();

    let mut sum = 0;
    let d = digits.len() / 2;
    for i in 0..digits.len() {
        if digits[i] == digits[(i + d) % digits.len()] {
            sum += digits[i];
        }
    }

    sum
}

fn main() {
    let input = get_input().unwrap_or_else(|err| {
        writeln!(io::stderr(), "{}", err).unwrap();
        exit(1)
    });

    println!("Part 1: {}", sum_consecutive_matches(&input));
    println!("Part 2: {}", sum_opposite_matches(&input));
}
